pub mod f_ctrl_enum;
pub mod f_opts;

use std::fmt;
use anyhow::{
    Result as AnyResult,
    anyhow
};
use std::borrow::Cow;

use crate::lorawan::phy_payload::mhdr::dir::Dir;
use f_ctrl_enum::{
    FCtrlEnum,
    f_ctrl_ul::FCtrlUL,
    f_ctrl_dl::FCtrlDL,
};
use f_opts::FOpts;


//
// FHDR{7..22}
//     DevAddr{4}
//     FCtrl{1} - ( FCtrlDL{1} | FCtrlUL{1} )
//     FCnt{2}
//     FOpts{0..15}
//

pub struct FHDRComps {
	pub dev_addr: u32,             // 4 bytes
	pub f_ctrl:   FCtrlEnum,           // 1 byte
	pub f_count:  u16,             // 2 bytes
	pub f_opts:   Option<FOpts>, // 0..15 bytes,
}
impl fmt::Display for FHDRComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            "\n\
                DevAddr:       0x{:08x}\n\
                FCtrl:\n\
                    {:4}\n\
                FCnt:          {}\n\
                FOpts:         {}\n\
            ", 
            self.dev_addr,
            self.f_ctrl,
            self.f_count,
            match &self.f_opts {
                Some(f_opts) => hex::encode(&f_opts),
                None => "".to_owned(),
            },
        )
    }
}

pub struct FHDR<'a> {
    pub bytes: Cow<'a, [u8]>,  // 7..22 bytes
}
impl<'a> FHDR<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) }
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            7..=22 => { 
                Ok( Self::from_bytes_no_check(bytes) )
            },
            other => { 
                Err(anyhow!("Invalid length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: FHDRComps) -> FHDR<'a> {
        let mut bytes = Vec::with_capacity(22);
        bytes.extend(comps.dev_addr.to_le_bytes());
        bytes.push(comps.f_ctrl.as_value());
        bytes.extend(comps.f_count.to_le_bytes());
        if let Some(v) = comps.f_opts {
            bytes.extend(v);
        }
        FHDR{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_comps(&self) -> FHDRComps {
        FHDRComps {
            dev_addr: self.dev_addr(),
            f_ctrl: self.f_ctrl(Dir::Uplink),
            f_count: self.f_count(),
            f_opts: self.f_opts(),
        }
    }

    pub fn dev_addr(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
    }

    pub fn f_ctrl(&self, dir: Dir) -> FCtrlEnum {
        FCtrlEnum::from_value_no_check(self.bytes[4], dir)
    }

    pub fn f_ctrl_ul(&self) -> FCtrlUL {
        FCtrlUL::from_value_no_check(self.bytes[4])
    }
    pub fn f_ctrl_dl(&self) -> FCtrlDL {
        FCtrlDL::from_value_no_check(self.bytes[4])
    }

    pub fn f_count(&self) -> u16 {
        u16::from_le_bytes(self.bytes[5..7].try_into().unwrap())
    }

    pub fn f_opts(&self) -> Option<FOpts> {
        if self.bytes.len() > 7 {
            Some(self.bytes[7..].into())
        } else {
            None
        }
    }

}
impl fmt::Display for FHDR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(
            f, 
            "\
                {padding}Bytes:     {}\n\
                {:width$}\
            ",
            hex::encode(&self.bytes),
            self.to_comps(),
            width = width
        )
    }
}

