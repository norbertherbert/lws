// use futures::executor::block_on;

use log::{info, 
    // warn, 
    error, debug, trace};

use std::time::Duration;
use std::{str, net::UdpSocket};

use tokio::task;

use lws::{get_settings_once, init_logger_once};

use lws::lorawan_pdu::{
    PktfMType, MType, 
    // RXPacket, Stat, 
    PushData, PHYDataComps
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let settings = get_settings_once();
    init_logger_once();

    debug!("{:?}", settings);

    // error!("this is an error");
    // info!("this is an info");
    // warn!("this is a warn");

    udp_server().await?;

    Ok(())

}

async fn udp_server() -> Result<(), Box<dyn std::error::Error>> {

    let settings = get_settings_once();

    let socket = UdpSocket::bind(&settings.udp_server.addr).unwrap();
    let mut buf = [0; 1000];
    loop {
        let (n, addr) = socket.recv_from(&mut buf).unwrap();
        let buf = &mut buf[..n];
        
        // trace!("{}", hex::encode(&buf));

        let protocol_version = buf[0];
        let pktf_mtype = PktfMType::from_u8(buf[3]); // TODO: manage panic
        let gateway_id = hex::encode(&buf[4..12]);

        if protocol_version != 2 { // PKTF_PROTOCOL_VERSION_V2 {
			error!(
				"Message received from {}; Invalid protocol version: {}",
				&addr, &protocol_version,
			);
			error!("  data: {:?}", hex::encode(&buf));
			continue;
		}

        match pktf_mtype {
            PktfMType::PushData => {

                /*
                trace!(
                    "PUSH_DATA received from Gateway: {} {} {}",
                    &gateway_id, &addr, str::from_utf8(&buf[12..n]).unwrap(),
                );
                */

                // TODO: Error handling!
                let push_data_struct: PushData = serde_json::from_str(
                    str::from_utf8(&buf[12..n]).unwrap()
                ).unwrap();

                if let Some(stat) = push_data_struct.stat {
                    trace!(
                        "PUSH_DATA_STAT received from Gateway: {} {} {:?}",
                        &gateway_id, &addr, stat,
                    );
                }

                if let Some(rxpk) = push_data_struct.rxpk {
                    for rx_packet in rxpk {

                        let d = base64::decode(rx_packet.data).unwrap();

                        /*
                        debug!(
                            "PUSH_DATA_RXPK received from Gateway: {} {} {}",
                            &gateway_id, &addr, hex::encode(&d),
                        );
                        */
    
                        let mtype_u8 = (d[0] & 0b11100000) >> 5;
                        let mtype = MType::from_u8(mtype_u8);

                        debug!(
                            "PUSH_DATA_RXPK:{:?} received from Gateway: {} {} {}",
                            mtype, &gateway_id, &addr, hex::encode(&d),
                        );

                        match mtype {
                            MType::JoinRequest => {
                                debug!("JoinRequest");
                            },
                            MType::JoinAccept => {
                                debug!("JoinAccept");
                            },
                            MType::UnconfirmedDataUp | MType::ConfirmedDataUp => {

                                debug!("UnconfirmedDataUp|ConfirmedDataUp");

                                let mut app_s_key_array = [0_u8; 16];
                                hex::decode_to_slice(&settings.default_key, &mut app_s_key_array).expect("decoding of default_key from config file has failed");
                                let app_s_key = Some(&app_s_key_array);
                                let nwk_s_key = app_s_key.clone();

                                // let app_s_key: Option<&[u8; 16]> = Some(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);
                                // let nwk_s_key: Option<&[u8; 16]> = Some(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);

                                let phy_data_comps = PHYDataComps::new(&d, app_s_key, nwk_s_key).unwrap();
                                
                                debug!("phy_data_comps: {}", phy_data_comps);



                                let dev_addr_text = format!("{:08x}", phy_data_comps.dev_addr);
                                let f_cnt_text = format!("{}", phy_data_comps.f_cnt);
                                let f_port_text = match &phy_data_comps.f_port {
                                    Some(f_port) => format!("{}", f_port),
                                    None => String::from("")
                                };
                                let frm_payload_text = match &phy_data_comps.frm_payload {
                                    Some(frm_payload) => format!("{}", frm_payload),
                                    None => String::from("")
                                };
                                let mic_ok = phy_data_comps.calc_mic == Some(phy_data_comps.mic);

                                info!(
                                    "DevAddr: 0x{}, FCnt: {}, FPort: {}, Data: \"{}\", MIC_ok: {}",
                                    dev_addr_text, f_cnt_text, f_port_text, frm_payload_text, mic_ok
                                );

                                let sp_fact: &str;
                                if &rx_packet.datr[3..4] == "B" {
                                    sp_fact = &rx_packet.datr[2..3];
                                } else {
                                    sp_fact = &rx_packet.datr[2..4];
                                }

                                // let sp_fact_text = &rx_packet.datr[2..4];
                                // let lrr_id_text = &gateway_id;
                                // let lrr_rssi_text = rx_packet.rssi;
                                // let lrr_snr_text = rx_packet.lsnr;

                                if mic_ok || settings.remote_application_server.forward_incorrect_mic {
                                    let data_to_send = format!(
r#"
{{ 
    "DevEUI_Uplink": {{ 
        "DevAddr": "{}", 
        "FCntUp": "{}", 
        "FPort": "{}", 
        "payload_hex": "{}",
        "SpFact": "{}",
        "Lrrs": {{ 
            "Lrr": [
                {{
                    "Lrrid": "{}",
                    "LrrRSSI": "{}",
                    "LrrSNR": "{}",
                }} 
            ] 
        }} 
    }}
}}
"#,
                                        dev_addr_text, f_cnt_text, f_port_text, frm_payload_text,
                                        sp_fact, &gateway_id, rx_packet.rssi, rx_packet.lsnr,

                                    );
                                    let _concurrent_future = task::spawn(
                                        async {
                                            let result = send_to_app_server(data_to_send).await;
                                            match result {
                                                Ok(()) => { println!("Message sent to AS") }
                                                Err(..) => { println!("Failed to forward to AS") }
                                            }
                                        }
                                    );
                                }




                            },
                            MType::UnconfirmedDataDown | MType::ConfirmedDataDown => {
                                debug!("UnconfirmedDataDown|ConfirmedDataDown");
                            },
                            MType::RejoinRequest => {
                                debug!("- RejoinRequest -");
                            },
                        }

                    }
                }

            },
            PktfMType::PullData => {
                trace!(
                    "PULL_DATA received from Gateway: {} {} {}",
                    &gateway_id, &addr, hex::encode(&buf),
                );
                let ack_msg = &mut buf[0..4];
                ack_msg[3] = PktfMType::PullAck as u8;
                socket.send_to(ack_msg, &addr).unwrap();
            },
            PktfMType::TxAck => {
                trace!(
                    "TX_ACK received from Gateway: {} {} {}",
                    &gateway_id, &addr, hex::encode(&buf),
                );
            },

            PktfMType::PushAck | PktfMType::PullAck => {} | PktfMType::PullResp => {
                // TODO: Invalid PktfMType
            },

        }

    }

}


async fn send_to_app_server(http_body: String) -> Result<(), Box<dyn std::error::Error>> {

    let settings = get_settings_once();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_static("SECRET"));
    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        // .gzip(true)
        .default_headers(headers)
        .timeout(Duration::from_secs(settings.remote_application_server.timeout))
        .build()?;

    let res = client
        // .post("http://httpbin.org/anything")
        .post(&settings.remote_application_server.url)
        // .post("https://webhook.site/fe6dd9e6-6f92-45ec-b842-d313422da3e3xxxx")
        // .post("https://asdf")
        // .json(http_body)
        .body(http_body)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Successful request!");
    } else {
        println!("HTTP Error happenes Status: {:?}", res.status());
    }

    let res_text = res
        .text()
        .await?;
        // .json::<JSONResponse>()
        // .await?

    println!("{}", res_text);

    Ok(())

}
