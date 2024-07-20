use std::time::SystemTime;
use lws::dd_cache::{
    DDData,
    CollectedDDDataWithTimestamp,
};



fn main() { 

    let collected_dd_data_with_timestamp = CollectedDDDataWithTimestamp{
        collected_dd_data: vec![
            DDData {
                gw_eui: 0x0000000011111111,
                freq: 0.0,
                sp_fact: 7,
                rssi: 0,
                snr: 0.0,
            },
            DDData {
                gw_eui: 0x0000000022222222,
                freq: 0.0,
                sp_fact: 7,
                rssi: 0,
                snr: 0.0,
            },
            DDData {
                gw_eui: 0x0000000033333333,
                freq: 0.0,
                sp_fact: 7,
                rssi: 0,
                snr: 0.0,
            },

        ],
        ts: SystemTime::now()
    };

    let x = &collected_dd_data_with_timestamp.collected_dd_data.iter().map(|dd_data| format!("{:016x}", dd_data.gw_eui)).collect::<Vec<String>>();

    println!("MAPPED: {:?}", x);
    println!("ALL:\n{:4}", &collected_dd_data_with_timestamp);

}