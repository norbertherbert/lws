use anyhow::{ Result as AnyResult, anyhow };

//********************************
//* Major
//********************************

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Major {
	LoRaWANR1 = 0,
	Rfu1      = 1,
    Rfu2      = 2,
	Rfu3      = 3,

}
impl Major {
    pub fn from_value_no_check(value: u8) -> Self {
        match value {
            0 => Major::LoRaWANR1,
            1 => Major::Rfu1,
            2 => Major::Rfu1,
            3 => Major::Rfu1,
            _ => panic!("invalid Major value: {}", value),
        }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0..=3 => Ok(Self::from_value_no_check(value)),
            _ => Err(anyhow!("invalid Major value: {}", value)),
        }
    }
}
