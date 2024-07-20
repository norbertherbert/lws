use anyhow::{ Result as AnyResult, anyhow };

// ********************************
// * RejoinType
// ********************************

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum RejoinType {   
    RejoinType0 = 0x00,
    RejoinType1 = 0x01,
    RejoinType2 = 0x02,
}
impl RejoinType {
    pub fn from_value_no_check(value: u8) -> Self {
        match value {
            0 => RejoinType::RejoinType0,
            1 => RejoinType::RejoinType1,
            2 => RejoinType::RejoinType2,
            _ => panic!("invalid RejoinType value: {}", value),
        }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0..=2 => Ok(Self::from_value_no_check(value)),
            _ => Err(anyhow!("invalid RejoinType value: {}", value)),
        }
    }
}