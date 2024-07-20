
use std::{process, sync::Once};
use log4rs::{
    config::{
        Appender,
        Config,
        // Logger,
        Root
    },
    encode::pattern::PatternEncoder,
    append::{
        console::ConsoleAppender, 
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy, 
                roll::fixed_window::FixedWindowRoller, 
                trigger::size::SizeTrigger,
            }
        }
    }
};
use crate::settings::Settings;

static ONCE_CONTROLLER_FOR_LOGGER: Once = Once::new();

pub fn init_logger_once(settings: &Settings) {
    ONCE_CONTROLLER_FOR_LOGGER.call_once(|| {
        init_logger(settings);
    });
}

fn init_logger(settings: &Settings) {

    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {m}{n}";

    // CREATING THE CONSOLE APPENDER

    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build();

    // CREATING THE ROLLING FILE APPENDER

    let trigger = Box::new(
        SizeTrigger::new(
            settings.log.file_size
        )
    );

    let roller_pattern = &(settings.log.dir.clone() + "/archive_{}.log");
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                process::exit(1)
            })
    );

    let rolling_file_appender = RollingFileAppender::builder()
        .encoder(
            Box::new(PatternEncoder::new(log_line_pattern))
        )
        .build(
            &(settings.log.dir.clone() + "/current.log"), 
            Box::new(CompoundPolicy::new(trigger, roller))
        )
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1)
        });

    // CREATING THE CONFIG

    let config = Config::builder()
        .appender(Appender::builder().build("console_appender", Box::new(console_appender)))
        .appender(Appender::builder().build("rolling_file_appender", Box::new(rolling_file_appender)))
        // .logger(
        //     Logger::builder()
        //         .appender("rolling_file_appender")
        //         .build("rolling_file_logger", settings.log.level),
        // )
        // .logger(
        //     Logger::builder()
        //         .appender("console_appender")
        //         .build("console_logger", settings.log.level),
        // )
        .build(
            Root::builder()
                .appender("console_appender")
                .appender("rolling_file_appender")
                .build(settings.log.level)
        )
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1)
        });


    // INITIATING THE LOGGER

    let _log_handle = log4rs::init_config(config)
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1)
        });

}
