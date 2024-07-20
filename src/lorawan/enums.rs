use anyhow::{ Result as AnyResult, anyhow };

//********************************
//* MType
//********************************

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 => Ok(MType::JoinRequest),
            1 => Ok(MType::JoinAccept),
            2 => Ok(MType::UnconfirmedDataUp),
            3 => Ok(MType::UnconfirmedDataDown),
            4 => Ok(MType::ConfirmedDataUp),
            5 => Ok(MType::ConfirmedDataDown),
            6 => Ok(MType::RejoinRequest),
            _ => Err(anyhow!("invalid MType value: {}", value)),
        }
    }
    pub fn get_dir(&self) -> Dir {
        match self {
            MType::ConfirmedDataUp | MType::UnconfirmedDataUp | MType::JoinRequest | MType::RejoinRequest => Dir::Uplink,
            MType::ConfirmedDataDown | MType::UnconfirmedDataDown | MType::JoinAccept => Dir::Downlink,
        }
    }
    pub fn valid_value(value: u8) -> bool {
        value < 7
    } 
}

#[test]
fn test_m_type() {
    let m_type = MType::from_value(0).unwrap();
    println!("{:?}, {:?}", m_type, m_type.get_dir());
}


//********************************
//* Major
//********************************

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Major {
	LoRaWanR1 = 0, // LoRaWAN R1
	Rfu1      = 1,
    Rfu2      = 2,
	Rfu3      = 3,

}
impl Major {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 => Ok(Major::LoRaWanR1),
            1 => Ok(Major::Rfu1),
            2 => Ok(Major::Rfu1),
            3 => Ok(Major::Rfu1),
            _ => Err(anyhow!("invalid Major value: {}", value)),
        }
    }
    pub fn valid_value(value: u8) -> bool {
        value == 0
    } 
}


//********************************
//* Dir
//********************************

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Dir {
	Uplink   = 0,
	Downlink = 1,
}
impl Dir {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 => Ok(Dir::Uplink),
            1 => Ok(Dir::Downlink),
            _ => Err(anyhow!("invalid Dir value: {}", value)),
        }
    }
}

//********************************
//* RJType
//********************************

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RJType {
	Type0 = 0,
	Type1 = 1,
	Type2 = 2,
}
impl RJType {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 => Ok(RJType::Type0),
            1 => Ok(RJType::Type1),
            2 => Ok(RJType::Type2),
            _ => Err(anyhow!("invalid Dir value: {}", value)),
        }
    }
}