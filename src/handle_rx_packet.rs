
use base64::engine::{ Engine as _, general_purpose::STANDARD as BASE64 };
use crate::{
    settings,
    dd_cache::DDData,
    pktf::RXPacket,
    lorawan::{
        self,
        crypto::{crypto10, crypto12},
        enums::{RJType, Dir},
    }
};

pub fn handle_rx_packet(collected_dd_data: Vec<DDData>, rx_packet: &RXPacket) {


        let settings = settings::get_or_init();

        let mut phy_payload: Vec<u8> = match BASE64.decode(rx_packet.data.clone()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("BASE64.decode() error {:?}", e);
                return;
            }                            
        };

        // let Ok(d) = BASE64.decode(rx_packet.data) else {
        //     log::error!("BASE64.decode() error {:?}", e);
        //     continue;
        // };

        // log::debug!(
        //     "PUSH_DATA_RXPK received from Gateway: {} {} {}",
        //     &gateway_id, &addr, hex::encode(&d),
        // );

        let phy_payload_len = phy_payload.len();

        let mhdr = phy_payload[0];

        let mhdr_m_type = match lorawan::MType::from_value((phy_payload[0] & 0b11100000) >> 5) {
            Ok(v) => v,
            Err(e) => {
                log::error!("lorawan::MType::from_value() error: {:?}", e);
                return;
            }
        };

        let mhdr_rfu = (mhdr & 0b00011100) >> 2;

        let mhdr_major = match lorawan::Major::from_value(mhdr & 0b00000011) {
            Ok(v) => v,
            Err(e) => {
                log::error!("lorawan::Major::from_value() error: {:?}", e);
                return;
            }
        };

        let gw_euis = &collected_dd_data.iter().map(|dd_data| format!("{:016x}", dd_data.gw_eui)).collect::<Vec<String>>();

        log::debug!(
            "PUSH_DATA_RXPK:{:?} received from Gateways: x{:?}, Data: {}",
            mhdr_m_type, 
            gw_euis, 
            hex::encode(&phy_payload),
        );

        match mhdr_m_type {

            lorawan::MType::JoinRequest => {

                /*
                ││..join_request [18]
                ││  ├── join_eui [8]
                ││  ├── dev_eui [8]
                ││  └── dev_nonce [2]
                */

                if phy_payload_len != 23 {
                    log::error!("Invalid Join Request length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                    return;
                }
                let join_eui = u64::from_le_bytes(phy_payload[1..9].try_into().unwrap());
                let dev_eui = u64::from_le_bytes(phy_payload[9..17].try_into().unwrap());
                let dev_nonce = u16::from_le_bytes(phy_payload[17..20].try_into().unwrap());

                let mic: [u8; 4] = (&phy_payload[phy_payload_len - 4..]).try_into().unwrap();

                // TODO: to get the appropriate key from Deevice Context based on dev_eui
                // this is just a mockup
                let mut nwk_key_array = [0_u8; 16];
                hex::decode_to_slice(&settings.default_key, &mut nwk_key_array)
                    .expect("decoding of default_key from config file has failed");
                // let nwk_key = Some(nwk_key_array);
                

                // let calculated_mic = crypto::aes128_cmac(
                //     &nwk_key_array, &phy_payload[0..19]
                // );
                let calculated_mic = crypto10::join_frame_calculate_mic(&nwk_key_array, &phy_payload);

                let is_mic_ok = Some( mic == calculated_mic );





/*
                let app_key_array = nwk_key_array.clone();
                let join_nonce: u32 = 0x000001;
                let net_id: u32 = 0x000001; 


                let app_s_key = crypto::derive_s_key(
                    &app_key_array,
                    KeyType::AppSKey, 
                    join_nonce,
                    net_id as u64, // Workaround to add net_id instead of join_eui for the case of LoRaWAN1.0.x
                    dev_nonce
                );

                let nwk_s_key = crypto::derive_s_key(
                    &nwk_key_array,
                    KeyType::FNwkSIntKey, 
                    join_nonce,
                    net_id as u64, // Workaround to add net_id instead of join_eui for the case of LoRaWAN1.0.x
                    dev_nonce
                );
*/






                let print_record = format!( 
"
JoinRequest: {}
    MHDR: {}
        MType: {:?}
        RFU:   {}
        Major: {:?}
    MACPayload: {}
        JoinEUI:  {} 
        DevEUI:   {}
        DevNonce: {}
        MIC:         {}
        MIC_OK:      {}
    MetaData:
        SpFact:   {}
        Freq:     {}
        RSSI:     {}
        SNR:      {}
",
                    hex::encode(&phy_payload),
                    format!("0x{:02x}", mhdr), 
                    mhdr_m_type, mhdr_rfu, mhdr_major,
                    hex::encode(&phy_payload[1..phy_payload_len-4]),
                    format!("0x{:016x}", join_eui),
                    format!("0x{:016x}", dev_eui),
                    format!("0x{:02x}", dev_nonce),
                    hex::encode(&mic),
                    match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },

                    if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                    rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,

                );
                    
                println!("{}", print_record);
                    
                let log_record = format!(
                    r#"{{"MType":"{:?}", "JoinEUI":"0x{:016x}", "DevEUI":"0x{:016x}", "DevNonce":"0x{:04x}", "MIC":"{}", "MIC_OK":"{}", "SpFact":"{}", "Freq":"{}", "RSSI":"{}", "SNR":"{}"}}"#,
                    mhdr_m_type, join_eui, dev_eui, dev_nonce,
                    hex::encode(&mic),
                    match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                    if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                    rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                );

                log::info!("{}", log_record);
                // println!("{}", log_record);

            },

            lorawan::MType::JoinAccept => {

                /*
                ││..join_accept [12|28]
                ││  ├── join_nonce [3]
                ││  ├── home_net_id [3]
                ││  ├── dev_addr [4]
                ││  ├── jadl_settings [1]
                ││  │   ├── opt_neg [.1]
                ││  │   ├── rx1_dr_offset [.3]
                ││  │   └── rx2_data_rate [.4]
                ││  ├── rx_delay [1]
                ││  └── cf_list [0|16]
                ││      ├┐ (cf_list_content_enum [15])
                ││      ││..dynamic_channel_list [15]
                ││      ││..fixed_channel_mask [15]
                ││      ││  ├── ch4freq [3]
                ││      ││  ├── ch5freq [3]
                ││      ││  ├── ch6freq [3]
                ││      ││  ├── ch7freq [3]
                ││      ││  └── ch8freq [3]
                ││      ││..new_join_eui_and_js_cookie [15]
                ││      │   ├── new_join_eui [8]
                ││      │   └── security_cookie [7]
                ││      └── cf_list_type [1]
                */

                if !matches!(phy_payload_len, 17 | 33) {
                    log::error!("Invalid Join Accept length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                    return;
                }

                // TODO: to be checked from session info
                // let triggered_by_join_request = true;


                // TODO: to get the appropriate key from Deevice Context based on dev_eui
                // this is just a mockup
                let mut nwk_key_array = [0_u8; 16];
                hex::decode_to_slice(&settings.default_key, &mut nwk_key_array)
                    .expect("decoding of default_key from config file has failed");
                // let nwk_key = Some(nwk_key_array);

                crypto10::join_accept_decrypt( &nwk_key_array, &mut phy_payload );

                // let calculated_mic = crypto::aes128_cmac(&nwk_key_array, &phy_payload[0..phy_payload_len - 4]);
                let calculated_mic = crypto10::join_frame_calculate_mic(&nwk_key_array, &phy_payload);

                let mic: [u8; 4] = phy_payload[phy_payload_len - 4..].try_into().unwrap();
                let is_mic_ok = Some( mic == calculated_mic );


                // let join_nonce = u32::from_le_bytes(phy_payload[1..4].try_into().unwrap());
                let join_nonce = u32::from_le_bytes(
                    [phy_payload[1], phy_payload[2], phy_payload[3], 0]
                );

                // let home_net_id = u32::from_le_bytes(phy_payload[4..7].try_into().unwrap());
                let home_net_id = u32::from_le_bytes(
                    [phy_payload[4], phy_payload[5], phy_payload[6], 0]
                );

                let dev_addr = u32::from_le_bytes(phy_payload[7..11].try_into().unwrap());

                let jadl_settings = phy_payload[11];
                let jadl_settings_opt_neg = (jadl_settings & 0b10000000) >> 7;
                let jadl_settings_rx1_dr_offset = (jadl_settings & 0b01110000) >> 4;
                let jadl_settings_rx2_data_rate = jadl_settings & 0b00001111;
                let rx_delay = phy_payload[12];

                let cf_list: Option<&[u8]> = if phy_payload_len == 33 {
                    Some(&phy_payload[13..19])
                } else { 
                    None 
                };
                
                let print_record = format!( 
"
JoinAccept: {}
    MHDR: {}
        MType: {:?}
        RFU:   {}
        Major: {:?}
    MACPayload: {}
        JoinNonce:    {} 
        HomeNetID:    {}
        DevAddr:      {}
        JADLSettings: {}
            OptNeg:       {}
            RX1DROffset:  {}
            RX2Datarate:  {}
        RXDelay:      {}
        CFList:       {}
        MIC:         {}
        MIC_OK:      {}
    MetaData:
        SpFact:   {}
        Freq:     {}
        RSSI:     {}
        SNR:      {}
",
                    hex::encode(&phy_payload),
                    format!("0x{:02x}", mhdr), 
                    mhdr_m_type, mhdr_rfu, mhdr_major,
                    hex::encode(&phy_payload[1..phy_payload_len-4]),
                    format!("0x{:06x}", join_nonce),
                    format!("0x{:06x}", home_net_id),
                    format!("0x{:08x}", dev_addr),
                    format!("0x{:02x}", jadl_settings),
                    jadl_settings_opt_neg, jadl_settings_rx1_dr_offset, jadl_settings_rx2_data_rate,
                    format!("0x{:02x}", rx_delay),
                    match cf_list { Some(v) => hex::encode(v), None => "".to_owned() },
                    hex::encode(mic),
                    match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                    if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                    rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                );
                    
                println!("{}", print_record);
                    

                log::debug!("JoinAccept");
            },

            lorawan::MType::UnconfirmedDataUp | 
            lorawan::MType::ConfirmedDataUp |
            lorawan::MType::UnconfirmedDataDown | 
            lorawan::MType::ConfirmedDataDown => {

                /*
                ││..data [7..M]
                │   ├── fhdr [7..22]
                │   │   ├─ dev_addr [4]
                │   │   ├─ f_ctrl [1]
                │   │   │  ├── adr [0.1]
                │   │   │  ├┐ (adr_ack_req_or_rfu_enum [0.1])
                │   │   │  ││..adr_ack_req [0.1]
                │   │   │  ││..rfu [0.1]
                │   │   │  ├── ack [0.1]
                │   │   │  ├┐ (class_b_or_f_pending_enum [0.1])
                │   │   │  ││..class_b [0.1]
                │   │   │  ││..f_pending [0.1]
                │   │   │  └── f_opts_len [0.4]
                │   │   ├──── f_cnt [2]
                │   │   └──── f_opts [0..15]
                │   ├── fport [0|1]
                │   └── frm_payload [0..M-8]
                */
                
                log::debug!("DataFrame");

                if phy_payload_len < 12 || 234 < phy_payload_len {
                    log::error!("Invalid Data frame length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                    return;
                }

                let f_ctrl_value = phy_payload[5];
                let f_ctrl_f_opts_len = (f_ctrl_value & 0b00001111) as usize;

                if 12 + f_ctrl_f_opts_len > phy_payload_len ||  // There is no room for FOpts
                13 + f_ctrl_f_opts_len == phy_payload_len    // FPort exists but FRMPayload does not exist
                {
                    log::error!("Invalid Data frame length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                    return;
                }

                // let mhdr = phy_payload[0];
                // let mhdr_m_type = (mhdr & 0b11100000) >> 5;
                // let mhdr_rfu = (mhdr & 0b00011100) >> 2;
                // let mhdr_major = mhdr & 0b00000011;

                let dev_addr = u32::from_le_bytes(phy_payload[1..5].try_into().unwrap());

                // TODO: Implement advanced filtering
                if dev_addr != 0x04000f20 {
                    log::debug!("Unknown DevAddr: 0x{:08x}", dev_addr);
                    return;
                }

                let dir = mhdr_m_type.get_dir();

                // TODO: to get the appropriate keys and f_cnt from Deevice Context based on dev_addr and dir
                // this is just a mockup
                let mut app_s_key_array = [0_u8; 16];
                hex::decode_to_slice(&settings.default_key, &mut app_s_key_array)
                    .expect("decoding of default_key from config file has failed");
                let nwk_s_key_array = app_s_key_array.clone();
                let app_s_key = Some(app_s_key_array); // TODO: get it from device conttext
                let nwk_s_key = Some(nwk_s_key_array); // TODO: get it from device conttext
                let f_cnt32_last_known = 0; // TODO: get it from device conttext

                // let f_ctrl_value = phy_payload[5];
                let f_ctrl_adr = (f_ctrl_value & 0b10000000) == 0b10000000;
                let f_ctrl_adr_ack_req_or_rfu = (f_ctrl_value & 0b01000000) == 0b01000000;
                let f_ctrl_ack = (f_ctrl_value & 0b00100000) == 0b00100000;
                let f_ctrl_class_b_or_f_pending = (f_ctrl_value & 0b00010000) == 0b00010000;
                // let f_ctrl_f_opts_len = (f_ctrl_value & 0b00001111) as usize;

                let f_cnt = u16::from_le_bytes(phy_payload[6..8].try_into().unwrap());

                let mut f_opts: Vec<u8> = Vec::with_capacity(f_ctrl_f_opts_len);
                f_opts.extend_from_slice(&phy_payload[8..8+f_ctrl_f_opts_len]);

                let f_port = if phy_payload_len > 0 {
                    Some(phy_payload[8 + f_ctrl_f_opts_len])
                } else {
                    None
                };

                let mut frm_payload: Vec<u8> = Vec::with_capacity(phy_payload_len);
                if phy_payload_len > 0 {
                    frm_payload.extend_from_slice(&phy_payload[9 + f_ctrl_f_opts_len .. phy_payload_len - 4]);
                }

                let mic: [u8; 4] = phy_payload[phy_payload_len - 4..].try_into().unwrap();

                let is_mic_ok: Option<bool>; 

                let f_cnt32 = (f_cnt32_last_known & 0xffff0000) + (f_cnt as u32);

                if let Some(nwk_s_key) = nwk_s_key {

                    let calculated_mic = crypto10::data_frame_calculate_mic(
                        phy_payload.as_ref(),
                        &nwk_s_key,
                        dev_addr,
                        f_cnt32,
                    );
                    is_mic_ok = Some(calculated_mic == mic);

                } else {
                    is_mic_ok = None;
                }

                if let Some(app_s_key) = app_s_key {

                    crypto10::frm_payload_crypt(
                        frm_payload.as_mut(), 
                        &app_s_key,
                        dir,
                        dev_addr,
                        f_cnt32,
                    ).unwrap();

                };

                let print_record = format!( 
"
PHYPayload: {}
    MHDR:        {}
        MType:       {:?}
        RFU:         {}
        Major:       {:?}
    MACPayload:  {}
        FHDR:           {}
        DevAddr:        {} 
        FCtrl:          {}
        ADR:            {} 
        {}      {}
        ACK:            {}
        {}      {}
        FOptsLen:       {}
        FCnt:           {}
        FOpts:          {}
        FPort:          {}
        FRMPayload:     {}
        MIC:         {}
        MIC_OK:      {}
    MetaData:
        SpFact:   {}
        Freq:     {}
        RSSI:     {}
        SNR:      {}
",
                    hex::encode(&phy_payload),
                    format!("0x{:02x}", mhdr), 
                    mhdr_m_type, mhdr_rfu, mhdr_major,
                    hex::encode(&phy_payload[1..phy_payload_len-4]),
                    hex::encode(&phy_payload[1..7+f_ctrl_f_opts_len]),
                    format!("0x{:08x}", dev_addr),
                    format!("0x{:02x}", f_ctrl_value),
                    f_ctrl_adr,
                    match dir { Dir::Uplink => "ADRAckReq:", Dir::Downlink => "RFU:      " },
                    f_ctrl_adr_ack_req_or_rfu,
                    f_ctrl_ack,
                    match dir { Dir::Uplink => "ClassB:   ", Dir::Downlink => "FPending: " },
                    f_ctrl_class_b_or_f_pending,
                    f_ctrl_f_opts_len,
                    f_cnt,
                    hex::encode(&f_opts),
                    match f_port { Some(v) => format!("{}", v), None => "".to_owned() },
                    hex::encode(&frm_payload),
                    hex::encode(mic),
                    match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                    if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                    rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,

                );

                println!("{}", print_record);

                let log_record = format!(
                    r#"{{"MType":"{:?}", "DevAddr":"0x{:08x}", "FCtrl_ADR":"{}", "FCtrl_{}":"{}", "FCtrl_ACK":"{}", "FCtrl_{}":"{}", "FCtrl_FOptsLen":"{}", "FCnt":"{}", "FOpts":"{}", "FPort":"{}", "FRMPayload":"{}", "MIC":"{}", "MIC_OK":"{}", "SpFact":"{}", "Freq":"{}", "RSSI":"{}", "SNR":"{}"}}"#,
                    mhdr_m_type, dev_addr, f_ctrl_adr,
                    match dir { Dir::Uplink => "ADRAckReq", Dir::Downlink => "RFU" },
                    f_ctrl_adr_ack_req_or_rfu, f_ctrl_ack,
                    match dir { Dir::Uplink => "ClassB", Dir::Downlink => "FPending" },
                    f_ctrl_class_b_or_f_pending, f_ctrl_f_opts_len, f_cnt,
                    hex::encode(&f_opts),
                    match f_port { Some(v) => format!("{}", v), None => "".to_owned() },
                    hex::encode(&frm_payload),
                    hex::encode(mic),
                    match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                    if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                    rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                );

                log::info!("{}", log_record);
                // println!("{}", log_record);

            },
            lorawan::MType::RejoinRequest => {

                // ││..rejoin_request [14|19]
                // ││  ├── rejoin_type [1]
                // ││  ├┐ (net_id_or_dev_eui_enum [3|8])
                // ││  ││..net_id [3]
                // ││  ││..join_eui [8]
                // ││  ├── dev_eui [8]
                // ││  └┐ (rj_count_02_or_rj_count_1_enum [2])
                // ││   │..rj_count_02 [2]
                // ││   │..rj_count_1 [2]

                let rj_type = match RJType::from_value(phy_payload[1]) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("RJRType::from_value() error: {:?}", e);
                        return;
                    }
                };

                let is_mic_ok: Option<bool>;

                // TODO: get key from db
                let s_nwk_s_int_key = Some(&[0_u8; 16]);
                let js_int_key = Some(&[0_u8; 16]);

                match rj_type {
                    RJType::Type0 | RJType::Type2 => {

                        if phy_payload_len != 19 {
                            log::error!("Invalid Rejoin Request length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                            return;
                        }

                        let mut net_id_array = [0_u8; 4];
                        net_id_array[0..3].copy_from_slice(&phy_payload[2..5]);
                        let net_id = u32::from_le_bytes(net_id_array);
                        let dev_eui = u64::from_le_bytes(phy_payload[10..18].try_into().unwrap());
                        let rj_count_02 = u16::from_le_bytes(phy_payload[18..21].try_into().unwrap());
                        let mic: [u8; 4] = (&phy_payload[phy_payload_len - 4..]).try_into().unwrap();

                        is_mic_ok = match s_nwk_s_int_key {
                            Some(s_nwk_s_int_key) => {
                                let calculated_mic = crypto12::join_request_calculate_mic(
                                    s_nwk_s_int_key,
                                    phy_payload.as_mut(), 
                                );
                                Some( mic == calculated_mic )
                            },
                            None => {
                                None
                            }
                        };

                        let print_record = format!( 
"
JoinRequest: {}
    MHDR: {}
        MType: {:?}
        RFU:   {}
        Major: {:?}
    MACPayload: {}
        RJType    {:?}
        NetID:    {} 
        DevEUI:   {}
        RJCount02:{}
        MIC:         {}
        MIC_OK:      {}
    MetaData:
        SpFact:   {}
        Freq:     {}
        RSSI:     {}
        SNR:      {}
",

                            hex::encode(&phy_payload),
                            format!("0x{:02x}", mhdr), 
                            mhdr_m_type, mhdr_rfu, mhdr_major,
                            hex::encode(&phy_payload[1..phy_payload_len-4]),
                            rj_type,
                            format!("0x{:016x}", net_id),
                            format!("0x{:016x}", dev_eui),
                            format!("0x{:02x}", rj_count_02),
                            hex::encode(&mic),
                            match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                            if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                            rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                        );

                        println!("{}", print_record);
                            
                        let log_record = format!(
                            r#"{{"MType":"{:?}", "RJType":"{:?}", "NetID":"0x{:016x}", "DevEUI":"0x{:016x}", "RJCount02":"0x{:04x}", "MIC":"{}", "MIC_OK":"{}", "SpFact":"{}", "Freq":"{}", "RSSI":"{}", "SNR":"{}"}}"#,
                            mhdr_m_type, rj_type, net_id, dev_eui, rj_count_02,
                            hex::encode(&mic),
                            match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                            if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                            rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                        );

                        log::info!("{}", log_record);
                        // println!("{}", log_record);

                    },
                    RJType::Type1 => {
                        if phy_payload_len != 24 {
                            log::error!("Invalid Rejoin Request length: ({}) {}", phy_payload_len, hex::encode(&phy_payload));
                            return;
                        }

                        let join_eui = u64::from_le_bytes(phy_payload[2..10].try_into().unwrap());
                        let dev_eui = u64::from_le_bytes(phy_payload[10..18].try_into().unwrap());
                        let rj_count_1 = u16::from_le_bytes(phy_payload[18..21].try_into().unwrap());
                        let mic: [u8; 4] = (&phy_payload[phy_payload_len - 4..]).try_into().unwrap();

                        is_mic_ok = match js_int_key {
                            Some(js_int_key) => {
                                let calculated_mic = crypto12::join_request_calculate_mic(
                                    js_int_key,
                                    phy_payload.as_mut(), 
                                );
                                Some( mic == calculated_mic )
                            },
                            None => {
                                None
                            }
                        };

                        let print_record = format!( 
"
JoinRequest: {}
    MHDR: {}
        MType: {:?}
        RFU:   {}
        Major: {:?}
    MACPayload: {}
        RJType    {:?}
        JoinEUI:  {} 
        DevEUI:   {}
        RJCount1: {}
        MIC:         {}
        MIC_OK:      {}
    MetaData:
        SpFact:   {}
        Freq:     {}
        RSSI:     {}
        SNR:      {}
",
                            hex::encode(&phy_payload),
                            format!("0x{:02x}", mhdr), 
                            mhdr_m_type, mhdr_rfu, mhdr_major,
                            hex::encode(&phy_payload[1..phy_payload_len-4]),
                            rj_type,
                            format!("0x{:016x}", join_eui),
                            format!("0x{:016x}", dev_eui),
                            format!("0x{:02x}", rj_count_1),
                            hex::encode(&mic),
                            match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                            if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                            rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                        );
                        
                        println!("{}", print_record);
                            
                        let log_record = format!(
                            r#"{{"MType":"{:?}", "RJType":"{:?}", "JoinEUI":"0x{:016x}", "DevEUI":"0x{:016x}", "RJCount1":"0x{:04x}", "MIC":"{}", "MIC_OK":"{}", "SpFact":"{}", "Freq":"{}", "RSSI":"{}", "SNR":"{}"}}"#,
                            mhdr_m_type, rj_type, join_eui, dev_eui, rj_count_1,
                            hex::encode(&mic),
                            match is_mic_ok { Some(v) => format!("{}", v), None => "".to_owned() },
                            if &rx_packet.datr[3..4] == "B" { &rx_packet.datr[..3] } else { &rx_packet.datr[..4] },
                            rx_packet.freq, rx_packet.rssi, rx_packet.lsnr,
                        );

                        log::info!("{}", log_record);
                        // println!("{}", log_record);
                    }
                }
            },
        }

}
