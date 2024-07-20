pub mod f_ctrl_ul;
pub mod f_ctrl_dl;

use std::fmt;
use anyhow::Result as AnyResult;

use crate::lorawan::phy_payload::mhdr::dir::Dir;
use f_ctrl_ul::{ FCtrlULComps, FCtrlUL };
use f_ctrl_dl::{ FCtrlDLComps, FCtrlDL };

//********************************
//* FCtrlEnumComps
//********************************

// TODO: Not sure if this is needed!
pub enum FCtrlEnumComps {
    UL(FCtrlULComps), 
    DL(FCtrlDLComps)
}
impl fmt::Display for FCtrlEnumComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            FCtrlEnumComps::UL(v) => {
                write!(f, "{:width$}", v, width=width)
            }
            FCtrlEnumComps::DL(v) => {
                write!(f, "{:width$}", v, width=width)
            }
        }
    }
}

//********************************
//* FCtrlEnum
//********************************

pub enum FCtrlEnum {
    UL(FCtrlUL), 
    DL(FCtrlDL)
}
impl fmt::Display for FCtrlEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            FCtrlEnum::UL(v) => {
                write!(f, "{:width$}", v, width=width)
            }
            FCtrlEnum::DL(v) => {
                write!(f, "{:width$}", v, width=width)
            }
        }
    }
}
impl<'a> FCtrlEnum {

    pub fn from_value_no_check(value: u8, dir: Dir) -> Self {
        match dir {
            Dir::Uplink => {
                Self::UL(
                    FCtrlUL::from_value_no_check(value)
                )
            },
            Dir::Downlink => {
                Self::DL(
                    FCtrlDL::from_value_no_check(value)
                )
            }
        }
    }
    pub fn from_value(value: u8, dir: Dir) -> AnyResult<Self> {

        // TODO: Check if this is needed!

        Ok(Self::from_value_no_check(value, dir))
    }

    pub fn from_comps(comps: &FCtrlEnumComps) -> FCtrlEnum {
        let f_ctrl: FCtrlEnum = match comps {
            FCtrlEnumComps::UL(v) => {
                FCtrlEnum::UL(
                    FCtrlUL::from_comps(v)
                )
            },
            FCtrlEnumComps::DL(v) => {
                FCtrlEnum::DL(
                    FCtrlDL::from_comps(v)
                )
            },
        };
        f_ctrl
    }

    pub fn as_value(&self) -> u8 {
        let value = match self {
            FCtrlEnum::UL(v) => {
                v.as_value()
            },
            FCtrlEnum::DL(v) => {
                v.as_value()
            },
        };
        value
    }

    pub fn to_comps(&self) -> FCtrlEnumComps {
        let f_ctrl_comps = match self {
            FCtrlEnum::UL(v) => {
                FCtrlEnumComps::UL(v.to_comps())
            },
            FCtrlEnum::DL(v) => {
                FCtrlEnumComps::DL(v.to_comps())
            },
        };
        f_ctrl_comps
    }

}

