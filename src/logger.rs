use crate::get_settings_once;

// use log::LevelFilter;

use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::append::{
    console::ConsoleAppender, 
    rolling_file::{
        RollingFileAppender,
        policy::compound::{
            CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
        }
    }
};

use std::sync::Once;

static ONCE_CONTROLLER_FOR_LOGGER: Once = Once::new();

pub fn init_logger_once() {
    ONCE_CONTROLLER_FOR_LOGGER.call_once(|| {
        init_logger();
    });
}

fn init_logger() {

    let settings = get_settings_once();
    
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // let log_line_pattern_debug = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";
    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {m}{n}";

    let trigger_size = settings.log.file_size; // byte_unit::n_mb_bytes!(30) as u64;
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern = &(settings.log.dir.clone() + "/archive_{}.log");
    
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build();

    let roller = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build(&(settings.log.dir.clone() + "/current.log"), compound_policy)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("roller", Box::new(roller)))
        // .logger(
        //     Logger::builder()
        //         .appender("roller")
        //         .build("rolling_logger", settings.log.level),
        // )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("roller")
                .build(settings.log.level)
        )
        .unwrap();

    let _log_handle = log4rs::init_config(config).unwrap();

}
