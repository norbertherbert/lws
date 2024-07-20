use serde::Deserialize;

use anyhow::{ Result as AnyResult, anyhow };

//********************************
//* ProtocolVersion
//********************************

#[repr(u8)]
#[derive(Debug)]
pub enum ProtocolVersion {
	V1 = 1,
	V2 = 2,
}
impl ProtocolVersion {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            1 => Ok(ProtocolVersion::V1),
            2 => Ok(ProtocolVersion::V2),
            _ => Err(anyhow!("invalid PktfProtocolVersion value: {}", value)),
        }
    }
	pub fn valid_value(value: u8) -> bool {
		(0 < value) && (value < 3) 
	}

}

//********************************
//* MType
//********************************

#[repr(u8)]
#[derive(Debug)]
pub enum MType {
	PushData = 0,
	PushAck  = 1,
	PullData = 2,
	PullResp = 3,
	PullAck  = 4,
	TxAck    = 5,
}

impl MType {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 => Ok(MType::PushData),
            1 => Ok(MType::PushAck),
            2 => Ok(MType::PullData),
            3 => Ok(MType::PullResp),
            4 => Ok(MType::PullAck),
            5 => Ok(MType::TxAck),
            _ => Err(anyhow!("invalid PktfMType value: {}", value)),
        }
    }
	pub fn valid_value(value: u8) -> bool {
		value < 6
	}
} 

//********************************
//* RXPacket
//********************************

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RXPacket {
    #[serde(default)]
	pub time: String, // | string | UTC time of pkt RX, us precision, ISO 8601 'compact' format
    #[serde(default)]
	pub tmms: i64,    // | number | GPS time of pkt RX, number of milliseconds since 06.Jan.1980
	pub tmst: i64,    // | number | Internal timestamp of "RX finished" event (32b unsigned)
	pub freq: f32,    // | number | RX central frequency in MHz (unsigned float, Hz precision)
	pub chan: i32,    // | number | Concentrator "IF" channel used for RX (unsigned integer)
	pub rfch: i32,    // | number | Concentrator "RF chain" used for RX (unsigned integer)
	pub stat: i32,    // | number | CRC status: 1 = OK, -1 = fail, 0 = no CRC
	pub modu: String, // | string | Modulation identifier "LORA" or "FSK"
	pub datr: String, // | string | LoRa datarate identifier (eg. SF12BW500)
	// pub datr u32,  // | number | FSK datarate (unsigned, in bits per second)
	pub codr: String, // | string | LoRa ECC coding rate identifier
	pub rssi: i32,    // | number | RSSI in dBm (signed integer, 1 dB precision)
	pub lsnr: f32,    // | number | Lora SNR ratio in dB (signed float, 0.1 dB precision)
	pub size: i32,    // | number | RF packet payload size in bytes (unsigned integer)
	pub data: String, // | string | Base64 encoded RF packet payload, padded
}

//********************************
//* Stat
//********************************

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Stat {
	#[serde(default)]
	pub time:  String,
	#[serde(default)]
	pub lati:  f32,
	#[serde(default)]
	pub long:  f32,
	#[serde(default)]
	pub alti:  i32,
	#[serde(default)]
	pub rxnb:  i32,
	#[serde(default)]
	pub rxok:  i32,
	#[serde(default)]
	pub rxfw:  i32,
	#[serde(default)]
	pub ackr:  f32,
	#[serde(default)]
	pub dwnb:  i32,
	#[serde(default)]
	pub txnb:  i32,
	#[serde(default)]
	pub temp:  f32,
	#[serde(default)]
	pub cpur:  f32,
	#[serde(default)]
	pub memr:  f32,
	#[serde(default)]
	pub count: i64,
}

//********************************
//* PushData
//********************************

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PushData {
    #[serde(default)]
	pub rxpk: Option<Vec<RXPacket>>,
    #[serde(default)]
	pub stat: Option<Stat>,
}
