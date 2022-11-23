use std::fmt;

use log::{
    // info, warn, 
    error, 
    // debug, trace
};
use serde::{Deserialize};

use crate::lorawan_crypto::{
    // aes128_encrypt, aes128_decrypt, aes128_cmac, 
    phy_data_crypt, 
    phy_data_calculate_mic
};

#[derive(Debug)]
pub enum PktfProtocolVersion {
	V1 = 1,
	V2 = 2,
}
impl PktfProtocolVersion {
    pub fn from_u8(n: u8) -> Self {
        match n {
            1 => PktfProtocolVersion::V1,
            2 => PktfProtocolVersion::V2,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum PktfMType {
	PushData = 0,
	PushAck  = 1,
	PullData = 2,
	PullResp = 3,
	PullAck  = 4,
	TxAck    = 5,
}
impl PktfMType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => PktfMType::PushData,
            1 => PktfMType::PushAck,
            2 => PktfMType::PullData,
            3 => PktfMType::PullResp,
            4 => PktfMType::PullAck,
            5 => PktfMType::TxAck,
            _ => panic!("Invalid Packet Forwarder Message Type!"),
        }
    }
}

#[derive(Debug)]
pub enum MType {
    JoinRequest         = 0,
    JoinAccept          = 1,
    UnconfirmedDataUp   = 2,
    UnconfirmedDataDown = 3,
    ConfirmedDataUp     = 4,
    ConfirmedDataDown   = 5,
    RejoinRequest       = 6,
}
impl MType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => MType::JoinRequest,
            1 => MType::JoinAccept,
            2 => MType::UnconfirmedDataUp,
            3 => MType::UnconfirmedDataDown,
            4 => MType::ConfirmedDataUp,
            5 => MType::ConfirmedDataDown,
            6 => MType::RejoinRequest,
            _ => panic!("MType Value: {}", n),
        }
    }
}

#[derive(Debug)]
pub enum Major {
	LorawanR1 = 0,
	Rfu1      = 1,
    Rfu2      = 2,
	Rfu3      = 3,

}
impl Major {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Major::LorawanR1,
            1 => Major::Rfu1,
            2 => Major::Rfu1,
            3 => Major::Rfu1,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Dir {
	Uplink   = 0,
	Downlink = 1,
}
impl Dir {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Dir::Uplink,
            1 => Dir::Downlink,
            _ => panic!(),
        }
    }
}

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

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Stat {
	pub time:  String,
	pub lati:  f32,
	pub long:  f32,
	pub alti:  i32,
	pub rxnb:  i32,
	pub rxok:  i32,
	pub rxfw:  i32,
	pub ackr:  f32,
	pub dwnb:  i32,
	pub txnb:  i32,
	pub temp:  f32,
	pub cpur:  f32,
	pub memr:  f32,
	pub count: i64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PushData {
    #[serde(default)]
	pub rxpk: Option<Vec<RXPacket>>,
    #[serde(default)]
	pub stat: Option<Stat>,
}

#[derive(Debug)]
pub struct Buf {
    pub buf: [u8; 256],
    pub len: usize
}
impl Buf {
    pub fn new() -> Buf {
        Buf { buf: [0_u8; 256], len: 0 }
    }
    pub fn new_from_slice(slice: &[u8]) -> Buf {
        let mut buf = [0_u8; 256];
        let len = slice.len();
        buf[0..slice.len()].copy_from_slice(slice);
        Buf { buf, len }
    }
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        self.buf[0..slice.len()].copy_from_slice(slice);
        self.len = slice.len()
    }
    pub fn to_slice(&self) -> &[u8] {
        &self.buf[0..self.len]
    }
}
impl fmt::Display for Buf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_slice()))
    }
}

#[derive(Debug)]
pub struct PHYDataComps {                 // 12..(M+5)
	pub mhdr:            u8,              // 1 byte,               MHDR.MType = UNCONFIRMED_DATA_DOWN | CONFIRMED_DATA_DOWN | UNCONFIRMED_DATA_DOWN | CONFIRMED_DATA_DOWN
	pub dev_addr:        u32,             // 4 bytes
	pub f_ctrl:          u8,              // 1 byte,               FCtrl.FOptsLen = len(FOpts)
	pub f_cnt:           u16,             // 2 byte
	pub f_opts:          Option<Buf>,     //Option<&'a [u8]>,   // 0..15 bytes,
	pub f_port:          Option<u8>,      //Option<u8>,         // 0..1 byte,            Ignored IF FRMPayload==nil
	pub frm_payload:     Option<Buf>,     //Option<&'a [u8]>,   // 0..(M-8-len(f_opts)),
	pub enc_frm_payload: Option<Buf>,     //Option<&'a [u8]>,   //                       Used by PHYDataComps_t.Build() as input if AppSKey==nil otherwise ignored
	pub mic:             [u8; 4],         //&'a [u8],           // 4 bytes,              Used in PHYDataComps_t.Build() as input if NwkSKey==nil otherwise ignored
	pub calc_mic:        Option<[u8; 4]>, //Option<&'a [u8]>,   //                       Used by PHYData_t.ToComps() as output if NwkSKey!=nil
}
impl fmt::Display for PHYDataComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
r#"
    MHDR:          0x{:02x}
    DevAddr:       0x{:08x}
    FCtrl:         0x{:02x}
    FCnt:          {}
    FOpts:         {}
    FPort:         {}
    FRMPayload:    {}
    encFRMPayload: {}
    MIC:           {}
    calcMIC:       {}"#, 
            self.mhdr,
            self.dev_addr,
            self.f_ctrl,
            self.f_cnt,
            match &self.f_opts {
                Some(f_opts) => format!("{}", f_opts),
                None => String::from("")
            },
            match &self.f_port {
                Some(f_port) => format!("{}", f_port),
                None => String::from("")
            },
            match &self.frm_payload {
                Some(frm_payload) => format!("{}", frm_payload),
                None => String::from("")
            },
            match &self.enc_frm_payload {
                Some(enc_frm_payload) => format!("{}", enc_frm_payload),
                None => String::from("")
            },
            hex::encode(self.mic),
            match &self.calc_mic {
                Some(calc_mic) => hex::encode(calc_mic),
                None => String::from("")
            }
        )
    }
}
impl<'a> PHYDataComps {
    pub fn new(b: &'a [u8], app_s_key: Option<&'a [u8; 16]>, nwk_s_key: Option<&'a [u8; 16]>) -> Result<PHYDataComps, &'static str> {

        let mhdr:            u8;
        let dev_addr:        u32;
        let f_ctrl:          u8;
        let f_cnt:           u16;
        let f_opts:          Option<Buf>; //Option<&'a [u8]>;
        let f_port:          Option<u8>;
        let frm_payload:     Option<Buf>; //Option<&'a [u8]>;
        let enc_frm_payload: Option<Buf>; //Option<&'a [u8]>;
        let mic:             [u8; 4]; //&'a [u8];
        let calc_mic:        Option<[u8; 4]>; //Option<&'a [u8]>;

        let l = b.len();
        let f_opts_len: usize;
        let err_msg_pfx = "PHYDataComps.from_bytes() error: ";
 
        // Error if there is no room in PHYDAta for mandatory components
        if l < 12 {
            error!("{}{}", err_msg_pfx, "there is no room in PHYDAta for mandatory components");
            return Err("TODO: Error handling"); // TODO: Error handling
        }

        mhdr = b[0];
        // TODO: Check IF MHDR is correct

        dev_addr  = (b[1] as u32) + ((b[2] as u32)<<8) + ((b[3] as u32)<<16) + ((b[4] as u32)<<24);

        f_ctrl = b[5];

        f_opts_len = (f_ctrl & 0b00001111) as usize;

       	// Error if there is no room for FCtrl.FOptsLen bytes in PHYData for FOpts
        if 8+f_opts_len > l - 4  {
            error!("{}{}", err_msg_pfx, "there is no room for FCtrl.FOptsLen bytes in PHYData for FOpts");
            return Err("TODO: Error handling"); // TODO: Error handling
        }

        f_cnt = (b[6] as u16) + ((b[7] as u16)<<8);

        if f_opts_len > 0 {
            f_opts = Some(Buf::new_from_slice(&b[8..8+f_opts_len]));
        } else {
            f_opts = None;
        }

        // Error if FPort exists but FRMPayload does not exist
        if 9+f_opts_len == l-4 {
            error!("{}{}", err_msg_pfx, "FPort exists but FRMPayload does not exist");
            return Err("TODO: Error handling"); // TODO: Error handling
        }

        // Do it only if both FPort and FRMPayload exists
        if 9+f_opts_len < (l-4) {
            f_port = Some(b[8 + f_opts_len]);

            let enc_frm_payload_value = Buf::new_from_slice(&b[9+f_opts_len .. l-4]);
            
            if let Some(app_s_key) = app_s_key {
                frm_payload = Some(
                    phy_data_crypt(app_s_key, &enc_frm_payload_value, &Buf::new_from_slice(&b[..8]), 0_u32).unwrap()
                );
            } else {
                frm_payload = None;
            }
            
            enc_frm_payload = Some(enc_frm_payload_value);

            /*
            if appSKey != nil {
                // TODO: to set fCnt32LastKnown
                let fCnt32LastKnown = FCnt_t(0)
                frmPayload, err := PHYData_Crypt(appSKey, phy_data_comps.EncFRMPayload, pd[:8], fCnt32LastKnown)
                if err != nil {
                    return phy_data_comps, fmt.Errorf("%s%w", errMsgPfx, err)
                }
                phy_data_comps.FRMPayload = frmPayload
            }
            */

        } else {
            f_port = None;
            frm_payload = None;
            enc_frm_payload = None;
        }

        mic = [b[l-4],b[l-3],b[l-2],b[l-1]];
        // calc_mic = Some(mic);


        if let Some(nwk_s_key) = nwk_s_key {
            calc_mic = Some(
                phy_data_calculate_mic(nwk_s_key, &Buf::new_from_slice(&b[..b.len()-4]), 0_u32).unwrap()
            );
        } else {
            calc_mic = None;
        }


        /*
        if nwkSKey != nil {
            // TODO: set fCnt32LastKnown
            fCnt32LastKnown := FCnt_t(0)
            calcMIC, err := PHYData_CalculateMIC(nwkSKey, pd[:len(pd)-4], fCnt32LastKnown)
            if err != nil {
                return pdComps, fmt.Errorf("%s%w", errMsgPfx, err)
            }
            pdComps.CalcMIC = calcMIC[:4]
        }
        */

        Ok(
            PHYDataComps{
                mhdr,
                dev_addr,
                f_ctrl,
                f_cnt,
                f_opts,
                f_port,
                frm_payload,
                enc_frm_payload,
                mic,
                calc_mic,
            }
        )

    }
}