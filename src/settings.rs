use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;
use std::sync::Once;

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(remote = "log::LevelFilter")]
pub enum LogLevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

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

static ONCE_CONTROLLER_FOR_SETTINGS: Once = Once::new();
static mut SETTINGS: Option<Settings> = None;

pub fn get_settings_once() -> &'static Settings {
    unsafe {
        ONCE_CONTROLLER_FOR_SETTINGS.call_once(|| {
            SETTINGS = Some(get_settings().unwrap());
        });
        SETTINGS.as_ref().unwrap()
    }
}

fn get_settings() -> Result<Settings, ConfigError> {
    
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let s = Config::builder()
        .add_source(File::with_name("config/default.toml"))
        .add_source(File::with_name(&format!("config/{}.toml", run_mode)).required(false))
        .add_source(File::with_name("config/local.toml").required(false))
        .add_source(Environment::with_prefix("lws"))
        .build()?;
    s.try_deserialize()

}



