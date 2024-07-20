use std::{ 
    str, net, thread
};

// use log::{ info, warn, error, debug, trace };
// use tokio::task;

use anyhow::Result as AnyResult;

use lws::{ 
    settings, logger, pktf, dd_cache,
    settings::Settings, 
    handle_rx_packet::handle_rx_packet,
};

#[tokio::main]
async fn main() -> AnyResult<()> {

    let settings = settings::get_or_init();

    logger::init_logger_once(settings);

    log::debug!("{:?}", settings);

    dd_cache::init_dd_cache();

    udp_server(settings).await?;

    Ok(())

}

async fn udp_server(settings: &Settings) -> AnyResult<()> {

    let socket = net::UdpSocket::bind(&settings.udp_server.addr).expect("UdpSocket::bind() must work");
    let mut buf = [0; 1024];
    loop {

        let (n, addr) = socket.recv_from(&mut buf).expect("socket.recv_from() must always work");

        if n > 1024 {
            log::error!("the received packet is longer than {}: {}", 1024, n);
            continue;
        }        
        
        let buf = &mut buf[..n];
        
        // log::trace!("{}", hex::encode(&buf));



        let protocol_version = match pktf::ProtocolVersion::from_value(buf[0]) {
            Ok(x) => x,
            Err(e) => {
                log::error!("pktf::ProtocolVersion::from_value() error: {:?}", e);
                continue;
            }
        };

        let pktf_mtype = match pktf::MType::from_value(buf[3]) {
            Ok(x) => x,
            Err(e) => {
                log::error!("pktf::MType::from_value() error: {:?}", e);
                continue;
            }
        };

        let gw_eui: u64 = u64::from_le_bytes(buf[4..12].try_into().unwrap());




        if !matches!(protocol_version, pktf::ProtocolVersion::V2) {
			log::error!(
				"Message received from {}; Invalid protocol version: {:?}",
				&addr, &protocol_version,
			);
			log::error!("  data: {:?}", hex::encode(&buf));
			continue;
		}

        match pktf_mtype {
            pktf::MType::PushData => {

                // Send ACK
                let ack_msg = &mut buf[0..4];
                ack_msg[3] = pktf::MType::PushAck as u8;
                socket.send_to(ack_msg, &addr)
                    .expect("socket.send_to() must always work");


                let push_data_str = match str::from_utf8(&buf[12..n]) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("str::from_utf8() error {:?}", e);
                        continue;
                    }
                };
            
                let push_data_struct: pktf::PushData = match serde_json::from_str(push_data_str) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("serde_json::from_str() error {:?}", e);
                        continue;
                    }
                };

                if let Some(stat) = push_data_struct.stat {
                    log::trace!(
                        "PUSH_DATA_STAT received from Gateway: x{:16x} IP: {} Port: {} Stat: {:?}",
                        &gw_eui, &addr.ip(), &addr.port(), stat,
                    );
                }

                if let Some(rxpk) = push_data_struct.rxpk {
                    for rx_packet in rxpk {

                        thread::spawn(move || {

                            
                            let sp_fact = if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] };
                            let sp_fact = sp_fact.parse::<u8>().unwrap();

                            let dd_data = dd_cache::DDData {
                                gw_eui: gw_eui,
                                freq: rx_packet.freq,
                                sp_fact,
                                rssi: rx_packet.rssi,
                                snr: rx_packet.lsnr
                            };

                            let is_first = dd_cache::add_data(dd_data, &rx_packet.data);
                            if !is_first { return };

                            thread::sleep(dd_cache::DD_PERIOD);

                            let collected_dd_data = dd_cache::take_collected_data(&rx_packet.data);

                            handle_rx_packet(collected_dd_data, &rx_packet);

                            // dedup::clean_cache();

                        });

                    }
                }
               
            },
            
            pktf::MType::PullData => {

                // Send ACK
                let ack_msg = &mut buf[0..4];
                ack_msg[3] = pktf::MType::PullAck as u8;
                socket.send_to(ack_msg, &addr).expect("socket.send_to() must always work");

                log::trace!(
                    "PULL_DATA received from Gateway: x{:16x} IP: {} Port: {} Data: {}",
                    gw_eui, &addr.ip(), &addr.port(), hex::encode(&buf),
                );

            },
            pktf::MType::TxAck => {
                log::trace!(
                    "TX_ACK received from Gateway: x{:16x} IP:{} Port:{} Data: {}",
                    &gw_eui, &addr.ip(), &addr.port(), hex::encode(&buf),
                );
            },

            // DOWNLINK MESSAGES that are always invalid...
            pktf::MType::PushAck | pktf::MType::PullAck | pktf::MType::PullResp => {
                log::error!("Invalid pktf::MType: {:?}", pktf_mtype);
                continue;
            },

        }

    }

}

/*
async fn send_to_app_server(http_body: String) -> AnyResult<()> {

    let settings = settings::get_settings_once();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_static("SECRET"));
    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        // .gzip(true)
        .default_headers(headers)
        .timeout(time::Duration::from_secs(setngs.remote_application_server.timeout))
        .build()?;

    let res = client
        // .post("http://httpbin.org/anything")
        .post(&setngs.remote_application_server.url)
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
*/