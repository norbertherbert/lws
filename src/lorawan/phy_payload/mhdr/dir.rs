use anyhow::{ Result as AnyResult, anyhow };

//********************************
//* Dir
//********************************

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Dir {
	Uplink   = 0,
	Downlink = 1,
}
impl Dir {
    pub fn from_value_no_check(value: u8) -> Self {
        match value {
            0 => Dir::Uplink,
            1 => Dir::Downlink,
            _ => panic!("invalid Dir value: {}", value),
        }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 | 1 => Ok(Self::from_value_no_check(value)),
            _ => Err(anyhow!("invalid Dir value: {}", value)),
        }
    }
}
