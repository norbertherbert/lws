use std::{
    thread, 
    sync::{
        Arc, 
        Mutex,
        // OnceLock, 
        // RwLock,
    },
    collections::{
        HashMap,
        HashSet,
    },
    time::Duration,
};


#[derive(Default, Clone)]
pub struct DeviceContextV10x {

    // pub cipher_id: String,
    // pub version_id: String,
    pub nwk_s_key: [u8; 16],
    // pub f_nwk_s_int_key: [u8; 16],
    // pub s_nwk_s_int_key: [u8; 16],
    // pub nwk_s_enc_key: [u8; 16],
    pub app_s_key: [u8; 16],
    pub dev_addr: u32,

    pub f_cnt_up: u32,
    pub f_cnt_down: u32,
    // pub n_f_cnt_down: u32,
    // pub a_f_cnt_down: u32,
    // pub rj_cnt_02: u16,

    pub active_channels: HashMap<u8, (u32, u8)>, // 00000000 00000000 00000000 00000007 
    pub pending_mac_cmds: HashSet<u8>,
    pub recent_gateways: HashSet<u64>,
    pub best_gateway: u64,

}

#[derive(Default, Clone)]
pub struct DeviceContextV12x {

    pub cipher_id: String,
    pub version_id: String,
    pub f_nwk_s_int_key: [u8; 16],
    pub s_nwk_s_int_key: [u8; 16],
    pub nwk_s_enc_key: [u8; 16],
    pub app_s_key: [u8; 16],
    pub dev_addr: u32,

    pub f_cnt_up: u32,
    pub n_f_cnt_down: u32,
    pub a_f_cnt_down: u32,
    pub rj_cnt_02: u16,

    pub active_channels: HashMap<u8, (u32, u8)>, // 00000000 00000000 00000000 00000007 
    pub pending_mac_cmds: HashSet<u8>,
    pub recent_gateways: HashSet<u64>,
    pub best_gateway: u64,

}

#[derive(Clone)]
pub enum DeviceContext {
    V10x(DeviceContextV10x),
    V12x(DeviceContextV12x),
}


#[derive(Default, Clone)]
pub struct DB(Arc<Mutex<HashMap<u64, DeviceContext>>>);
impl DB {

    pub fn new(n: usize) -> Self {
        DB(Arc::new(Mutex::new(HashMap::with_capacity(n))))
    }

    pub fn insert(&self, dev_eui: u64, device_context: DeviceContext) {
        self.0.lock().unwrap().insert(dev_eui, device_context);
    }

    pub fn init(&mut self) {
        let dev_eui_10: u64 = 0xaabbccddaabbcc10;
        let device_context_10 = DeviceContextV10x{
            nwk_s_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            app_s_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            dev_addr: 0x11223344, 
            .. DeviceContextV10x::default()
        };
        let dev_eui_12: u64 = 0xaabbccddaabbcc12;
        let device_context_12 = DeviceContextV12x{
            f_nwk_s_int_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            s_nwk_s_int_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            nwk_s_enc_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            app_s_key: hex::decode("aabbccddaabbccddaabbccddaabbccdd").unwrap().try_into().unwrap(),
            dev_addr: 0x11223344, 
            .. DeviceContextV12x::default()
        };
    
        self.insert(dev_eui_10, DeviceContext::V10x(device_context_10));
        self.insert(dev_eui_12, DeviceContext::V12x(device_context_12));
    }

    pub fn get_device_context(&self, dev_eui: u64) -> Option<DeviceContext> {
        match self.0
            .lock()
            .unwrap()
            .get(&dev_eui)
        {
            Some(ctx) => {
                Some(ctx.clone())
            },
            None => {
                None
            }
        }
    }

    pub fn set_f_cnt_up(&mut self, dev_eui: u64, val: u32) {
        match
            self.0
                .lock()
                .unwrap()
                .get_mut(&dev_eui) 
        {
            Some(ctx) => {
                match ctx {
                    DeviceContext::V10x(ctx) => {
                        ctx.f_cnt_up = val;
                    },
                    DeviceContext::V12x(ctx) => {
                        ctx.f_cnt_up = val;
                    },
                }
            },
            None => {
    
            }
        };
    }
    
    pub fn set_f_cnt_down(&mut self, dev_eui: u64, val: u32) {
        match
            self.0
                .lock()
                .unwrap()
                .get_mut(&dev_eui) 
        {
            Some(ctx) => {
                match ctx {
                    DeviceContext::V10x(ctx) => {
                        ctx.f_cnt_down = val;
                    },
                    DeviceContext::V12x(_) => {
                    },
                }
            },
            None => {
    
            }
        };
    }
    
    pub fn set_n_f_cnt_down(&mut self, dev_eui: u64, val: u32) {
        match
            self.0
                .lock()
                .unwrap()
                .get_mut(&dev_eui) 
        {
            Some(ctx) => {
                match ctx {
                    DeviceContext::V10x(_) => {
                    },
                    DeviceContext::V12x(ctx) => {
                        ctx.n_f_cnt_down = val;
                    },
                }
            },
            None => {
    
            }
        };
    }
    
    pub fn set_a_f_cnt_down(&mut self, dev_eui: u64, val: u32) {
        match
            self.0
                .lock()
                .unwrap()
                .get_mut(&dev_eui) 
        {
            Some(ctx) => {
                match ctx {
                    DeviceContext::V10x(_) => {          
                    },
                    DeviceContext::V12x(ctx) => {
                        ctx.a_f_cnt_down = val;
                    },
                }
            },
            None => {
    
            }
        };
    }

}







pub fn main() {

    let mut db = DB::new(20);
    db.init();

    let dev_eui: u64 = 0xaabbccddaabbcc12;

    db.set_f_cnt_down(dev_eui, 5);


    let mut db1 = db.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(15));
            match db1.get_device_context(dev_eui).unwrap() {
                DeviceContext::V10x(ctx) => {
                    let cnt = ctx.f_cnt_down;
                    db1.set_f_cnt_down(dev_eui, cnt+1);
                },
                DeviceContext::V12x(ctx) => {
                    let cnt = ctx.n_f_cnt_down;
                    db1.set_n_f_cnt_down(dev_eui, cnt+1);
                },
            };
        }
    });


    let mut db2 = db.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(20));
            match db2.get_device_context(dev_eui).unwrap() {
                DeviceContext::V10x(ctx) => {
                    let cnt = ctx.f_cnt_down;
                    db2.set_f_cnt_down(dev_eui, cnt+1);
                },
                DeviceContext::V12x(ctx) => {
                    let cnt = ctx.n_f_cnt_down;
                    db2.set_n_f_cnt_down(dev_eui, cnt+1);
                },
            };
        }
    });

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(5));

        match db.get_device_context(dev_eui).unwrap() {
            DeviceContext::V10x(ctx) => {
                let cnt = ctx.f_cnt_down;
                println!("Current f_cnt_down value: {}", cnt);
            },
            DeviceContext::V12x(ctx) => {
                let cnt = ctx.n_f_cnt_down;
                println!("Current n_f_cnt_down value: {}", cnt);
            },
        };
        
    }

    handle1.join().unwrap();
    handle2.join().unwrap();

    match db.get_device_context(dev_eui).unwrap() {
        DeviceContext::V10x(ctx) => {
            let cnt = ctx.f_cnt_down;
            println!("Final f_cnt_down value: {}", cnt);
        },
        DeviceContext::V12x(ctx) => {
            let cnt = ctx.n_f_cnt_down;
            println!("Final n_f_cnt_down value: {}", cnt);
        },
    };

}
