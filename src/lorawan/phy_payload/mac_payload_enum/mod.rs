pub mod join_request;
pub mod rejoin_request;
pub mod join_accept;
pub mod data;

use std::fmt;
use anyhow::Result as AnyResult;


use join_request::JoinRequestComps;
use join_request::JoinRequest;
use rejoin_request::{
    rejoin_type::RejoinType,
    rejoin_request_content_enum::{
        rejoin_request_02::{
            RejoinRequest02Comps, 
            RejoinRequest02,
        }, 
        rejoin_request_1::{
            RejoinRequest1Comps, 
            RejoinRequest1,
        }
    }
};
use join_accept::JoinAcceptComps;
use join_accept::JoinAccept;
use data::DataComps;
use data::Data;

use super::mhdr::f_type::FType;


/*
********************************
* MACPayloadEnumComps
********************************
*/

pub enum MACPayloadEnumComps<'a> {
    JoinRequest(JoinRequestComps),
    RejoinRequest02(RejoinRequest02Comps),
    RejoinRequest1(RejoinRequest1Comps),
    JoinAccept(JoinAcceptComps<'a>),
    Data(DataComps<'a>),
}
impl fmt::Display for MACPayloadEnumComps<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            MACPayloadEnumComps::JoinRequest(v) => {
                write!(f, "{:width$}", v, width=width)
            }
            MACPayloadEnumComps::RejoinRequest02(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnumComps::RejoinRequest1(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnumComps::JoinAccept(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnumComps::Data(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
        }
    }
}



// ********************************
// * MACPayloadEnum
// ********************************

#[derive(Debug)]
pub enum MACPayloadEnum<'a> {
    JoinRequest(JoinRequest<'a>),
    RejoinRequest02(RejoinRequest02<'a>),
    RejoinRequest1(RejoinRequest1<'a>),
    JoinAccept(JoinAccept<'a>),
    Data(Data<'a>),
}
impl fmt::Display for MACPayloadEnum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            MACPayloadEnum::JoinRequest(v) => {
                write!(f, "{:width$}", v, width=width)
            }
            MACPayloadEnum::RejoinRequest02(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnum::RejoinRequest1(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnum::JoinAccept(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            MACPayloadEnum::Data(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
        }
    }
}

impl<'a> MACPayloadEnum<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8], f_type: FType) -> MACPayloadEnum {
        let mac_payload = match f_type {
            FType::JoinRequest => { 
                MACPayloadEnum::JoinRequest(
                    JoinRequest::from_bytes_no_check(bytes)
                ) 
            },
            FType::JoinAccept => {
                MACPayloadEnum::JoinAccept( 
                    JoinAccept::from_bytes_no_check(bytes)
                ) 
            },
            FType::UnconfirmedDataUp | FType::UnconfirmedDataDown | FType::ConfirmedDataUp | FType::ConfirmedDataDown  => { 
                MACPayloadEnum::Data(
                    Data::from_bytes_no_check(bytes)
                )
            },
            FType::RejoinRequest => {
                let rj_type = RejoinType::from_value_no_check(bytes[0]);
                match rj_type {
                    RejoinType::RejoinType0 | RejoinType::RejoinType2 => {
                        MACPayloadEnum::RejoinRequest02(
                            RejoinRequest02::from_bytes_no_check(bytes)
                        )
                    },
                    RejoinType::RejoinType1 => {
                        MACPayloadEnum::RejoinRequest1(
                            RejoinRequest1::from_bytes_no_check(bytes)
                        )
                    },
                }
            },
        };
        mac_payload
    }

    pub fn from_bytes(bytes: &'a [u8], f_type: FType) -> AnyResult<MACPayloadEnum> {
        let mac_payload = match f_type {
            FType::JoinRequest => { 
                MACPayloadEnum::JoinRequest(
                    JoinRequest::from_bytes(bytes)?
                ) 
            },
            FType::JoinAccept => {
                MACPayloadEnum::JoinAccept( 
                    JoinAccept::from_bytes(bytes)?
                ) 
            },
            FType::UnconfirmedDataUp | FType::UnconfirmedDataDown | FType::ConfirmedDataUp | FType::ConfirmedDataDown  => { 
                MACPayloadEnum::Data(
                    Data::from_bytes(bytes)?
                )
            },
            FType::RejoinRequest => {
                let rj_type = RejoinType::from_value(bytes[0])?;
                match rj_type {
                    RejoinType::RejoinType0 | RejoinType::RejoinType2 => {
                        MACPayloadEnum::RejoinRequest02(
                            RejoinRequest02::from_bytes(bytes)?
                        )
                    },
                    RejoinType::RejoinType1 => {
                        MACPayloadEnum::RejoinRequest1(
                            RejoinRequest1::from_bytes(bytes)?
                        )
                    },
                }
            },
        };
        Ok(mac_payload)
    }

    pub fn from_comps(comps: &'a MACPayloadEnumComps) -> MACPayloadEnum<'a> {
        let mac_payload = match comps {
            MACPayloadEnumComps::JoinRequest(v) => {
                MACPayloadEnum::JoinRequest(
                    JoinRequest::from_comps(v)
                )
            },
            MACPayloadEnumComps::RejoinRequest02(v) => {
                MACPayloadEnum::RejoinRequest02(
                    RejoinRequest02::from_comps(v)
                )
            },
            MACPayloadEnumComps::RejoinRequest1(v) => {
                MACPayloadEnum::RejoinRequest1(
                    RejoinRequest1::from_comps(v)
                )
            },
            MACPayloadEnumComps::JoinAccept(v) => {
                MACPayloadEnum::JoinAccept(
                    JoinAccept::from_comps(v)
                )
            },
            MACPayloadEnumComps::Data(v) => {
                MACPayloadEnum::Data(
                    Data::from_comps(v)
                )
            },
        };
        mac_payload
    }

    pub fn as_bytes(&self) -> &[u8] {
        let bytes = match self {
            MACPayloadEnum::JoinRequest(v) => {
                v.as_bytes()
            },
            MACPayloadEnum::RejoinRequest02(v) => {
                v.as_bytes()
            },
            MACPayloadEnum::RejoinRequest1(v) => {
                v.as_bytes()
            },
            MACPayloadEnum::JoinAccept(v) => {
                v.as_bytes()
            },
            MACPayloadEnum::Data(v) => {
                v.as_bytes()
            },
        };
        bytes
    }

    pub fn to_comps(&self) -> MACPayloadEnumComps {
        let mac_payload_comps = match self {
            MACPayloadEnum::JoinRequest(v) => {
                MACPayloadEnumComps::JoinRequest(v.to_comps())
            },
            MACPayloadEnum::RejoinRequest02(v) => {
                MACPayloadEnumComps::RejoinRequest02(v.to_comps())
            },
            MACPayloadEnum::RejoinRequest1(v) => {
                MACPayloadEnumComps::RejoinRequest1(v.to_comps())
            },
            MACPayloadEnum::JoinAccept(v) => {
                MACPayloadEnumComps::JoinAccept(v.to_comps())
            },
            MACPayloadEnum::Data(v) => {
                MACPayloadEnumComps::Data(v.to_comps())
            },
        };
        mac_payload_comps
    }

}
