use std::{
    fmt,
    sync::{
        Mutex,
        OnceLock, 
    },
    collections::HashMap,
    time::{
        SystemTime,
        Duration,
    },
};

// This is the data that needs to be deduplicated
type DDSubject = String;

pub const DD_PERIOD: Duration = Duration::from_millis(300);

static DD_CACHE: OnceLock<Mutex<HashMap<DDSubject, CollectedDDDataWithTimestamp>>> = OnceLock::new();

#[derive(Debug)]
pub struct DDData {
    pub gw_eui: u64,
    pub sp_fact: u8,
    pub freq: f32,
    pub rssi: i32,
    pub snr: f32,
}
impl fmt::Display for DDData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}gw_eui:  0x{:016x}\n\
                {padding}sp_fact: {}\n\
                {padding}freq:    {}\n\
                {padding}rssi:    {}\n\
                {padding}snr:     {}\n\
            ", 
            self.gw_eui,
            self.sp_fact,
            self.freq,
            self.rssi,
            self.snr,
        )
    }
}


pub struct CollectedDDDataWithTimestamp {
    pub collected_dd_data: Vec<DDData>,
    pub ts: SystemTime,
}
impl fmt::Display for CollectedDDDataWithTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        // let padding = " ".repeat(width);
        self.collected_dd_data.iter().fold(Ok(()), |result, item| {
            result.and_then(|_| {
                writeln!(
                    f, 
                    "\
                        {:width$}\
                    ",
                    item, 
                    width=width
                )
            })
        })
    }
}


pub fn init_dd_cache() {
    let _ = DD_CACHE.set(Mutex::new(HashMap::new()));
}

pub fn add_data(dd_data: DDData, dd_subject: &DDSubject) -> bool {

    let mut dd_cache = DD_CACHE
        .get()
        .unwrap()
        .lock()
        .unwrap();

    match dd_cache.get_mut(dd_subject) {
        None => {
            let collected_dd_data_with_timestamp = CollectedDDDataWithTimestamp {
                collected_dd_data: vec![dd_data], 
                ts: SystemTime::now() 
            };
            dd_cache.insert(dd_subject.clone(), collected_dd_data_with_timestamp);
            true
        },
        Some(collected_dd_data_with_timestamp) => {
            if collected_dd_data_with_timestamp.ts.elapsed().unwrap() < DD_PERIOD {
                collected_dd_data_with_timestamp.collected_dd_data.push(dd_data);
                collected_dd_data_with_timestamp.ts = SystemTime::now(); 
                false
            } else {
                let collected_dd_data_with_timestamp = CollectedDDDataWithTimestamp { 
                    collected_dd_data: vec![dd_data], 
                    ts: SystemTime::now() 
                };
                dd_cache.insert(dd_subject.clone(), collected_dd_data_with_timestamp);
                true
            }
        },
    }

}

pub fn take_collected_data(dd_subject: &DDSubject) -> Vec<DDData> {

    let mut dd_cache = DD_CACHE
        .get()
        .unwrap()
        .lock()
        .unwrap();

    match dd_cache.remove(dd_subject)
    {
        Some(collected_dd_data_with_timestamp) => {
            if collected_dd_data_with_timestamp.ts.elapsed().unwrap() > DD_PERIOD {
                vec!()
            } else {
                collected_dd_data_with_timestamp.collected_dd_data
            }
        },
        None => {
            vec!()
        }
    }
}

pub fn clean_cache() {

    let mut dd_cache = DD_CACHE
        .get()
        .unwrap()
        .lock()
        .unwrap();

    dd_cache.retain(|_, collected_dd_data_with_timestamp| { 
        collected_dd_data_with_timestamp.ts.elapsed().unwrap() < DD_PERIOD
    });

}
