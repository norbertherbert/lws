use std::{
    thread, 
    time::Duration,
};
use lws::devctx;
use lws::devctx::DeviceContext;

pub fn main() {

    devctx::init_db();

    let dev_eui: u64 = 0xaabbccddaabbcc10;

    devctx::set_f_cnt_up(dev_eui, 5);


    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(15));
            match devctx::get_device_context(dev_eui).unwrap() {
                DeviceContext::V10x(ctx) => {
                    let cnt = ctx.f_cnt_down;
                    devctx::set_f_cnt_down(dev_eui, cnt+1);
                },
                DeviceContext::V12x(ctx) => {
                    let cnt = ctx.n_f_cnt_down;
                    devctx::set_n_f_cnt_down(dev_eui, cnt+1);
                },
            };
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(20));
            match devctx::get_device_context(dev_eui).unwrap() {
                DeviceContext::V10x(ctx) => {
                    let cnt = ctx.f_cnt_down;
                    devctx::set_f_cnt_down(dev_eui, cnt+1);
                },
                DeviceContext::V12x(ctx) => {
                    let cnt = ctx.n_f_cnt_down;
                    devctx::set_n_f_cnt_down(dev_eui, cnt+1);
                },
            };
        }
    });

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(5));

        match devctx::get_device_context(dev_eui).unwrap() {
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

    match devctx::get_device_context(dev_eui).unwrap() {
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
