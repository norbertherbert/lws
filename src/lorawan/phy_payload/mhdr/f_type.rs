use anyhow::{ Result as AnyResult, anyhow };


use super::dir::Dir;

//********************************
//* FType
//********************************

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum FType {
    JoinRequest         = 0,
    JoinAccept          = 1,
    UnconfirmedDataUp   = 2,
    UnconfirmedDataDown = 3,
    ConfirmedDataUp     = 4,
    ConfirmedDataDown   = 5,
    RejoinRequest       = 6,
}
impl FType {
    pub fn from_value_no_check(value: u8) -> Self {
        match value {
            0 => FType::JoinRequest,
            1 => FType::JoinAccept,
            2 => FType::UnconfirmedDataUp,
            3 => FType::UnconfirmedDataDown,
            4 => FType::ConfirmedDataUp,
            5 => FType::ConfirmedDataDown,
            6 => FType::RejoinRequest,
            _ => panic!("invalid FType value: {}", value),
        }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0..=6 => Ok(Self::from_value_no_check(value)),
            _ => Err(anyhow!("invalid FType value: {}", value)),
        }
    }
    pub fn get_dir(&self) -> Dir {
        match self {
            FType::ConfirmedDataUp | FType::UnconfirmedDataUp | FType::JoinRequest | FType::RejoinRequest => Dir::Uplink,
            FType::ConfirmedDataDown | FType::UnconfirmedDataDown | FType::JoinAccept => Dir::Downlink,
        }
    }
}

#[test]
fn test_m_type() {
    let m_type = FType::from_value(0).unwrap();
    println!("{:?}, {:?}", m_type, m_type.get_dir());
}