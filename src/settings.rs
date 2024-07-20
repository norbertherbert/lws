use std:: {
    env, process,
    sync::OnceLock,
};
use config::{Config, ConfigError};
use serde_derive::Deserialize;

// This is the local, manual copy of the definition of the external log::LevelFilter type

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(remote = "log::LevelFilter")]
pub enum LogLevelFilterDef {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// The definition of the "Settings" structure

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct UdpServer {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RemoteApplicationServer {
    pub url: String,
    pub timeout: u64,
    pub forward_incorrect_mic: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Log {
    pub dir: String,
    pub file_size: u64,
    #[serde(with = "LogLevelFilterDef")]
    pub level: log::LevelFilter,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub default_key: String,
    pub udp_server: UdpServer,
    pub remote_application_server: RemoteApplicationServer,
    pub log: Log,
}
impl Settings {
    fn new() -> Result<Settings, ConfigError> {

        // the default values of the "Settings" structure

        let cfg_txt = 
r#"
debug = true
default_key = "00000000000000000000000000000000"

[udp_server]
addr = "0.0.0.0:1700"

[remote_application_server]
url = "http://localhost"
timeout = 5           # seconds
forward_incorrect_mic = false

[log]
dir = "log"
file_size = 100000    # bytes
level = "Trace"       # Error|Warn|Info|Debug|Trace
"#;

        let _default_settings = Settings {
            debug: true,
            default_key: "00000000000000000000000000000000".to_owned(),
            udp_server: UdpServer {
                addr: "0.0.0.0:1700".to_owned(), 
            },
            remote_application_server: RemoteApplicationServer {
                url: "http://localhost".to_owned(),
                timeout: 5, // seconds
                forward_incorrect_mic: false,
            },
            log: Log {
                dir: "log".to_owned(),
                file_size: 100_000, // bytes
                level: log::LevelFilter::Trace, // Error|Warn|Info|Debug|Trace
            }
        };

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        Config::builder()
            .add_source(config::File::with_name("config/default.toml"))
            .add_source(config::File::with_name(&format!("config/{}.toml", run_mode)).required(false))
            .add_source(config::File::with_name("config/local.toml").required(false))
            .add_source(config::Environment::with_prefix("lws"))
            .build()?
            .try_deserialize::<Settings>()
    }
}



pub static SETTINGS: OnceLock<Settings> = OnceLock::new();


pub fn init() {
    SETTINGS.set(
        Settings::new().unwrap_or_else(|e| {
            eprintln!("error: {:?}", e);
            process::exit(1);
        })
    ).unwrap_or_else(|v| {
        eprintln!("The Settings cannot be set (probably because it has already been set) {:?}", v);
        process::exit(1);     
    });
}

pub fn get_or_init() -> &'static Settings {
    SETTINGS.get_or_init(|| {
        Settings::new().unwrap_or_else(|e| { 
            eprintln!("error: {:?}", e);
            process::exit(1);
        })
    })
}
