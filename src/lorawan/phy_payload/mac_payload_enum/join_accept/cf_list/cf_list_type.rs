use anyhow::{ Result as AnyResult, anyhow };

// ********************************
// * CFListType
// ********************************

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CFListType {   
    DynamicChannelList = 0,
    FixedChannelMask = 1,
    NewJoinEUIandJSCookie = 255,
}
impl CFListType {
    pub fn from_value_no_check(value: u8) -> Self {
        match value {
            0 => CFListType::DynamicChannelList,
            1 => CFListType::FixedChannelMask,
            255 => CFListType::NewJoinEUIandJSCookie,
            _ => panic!("invalid CFListType value: {}", value),
        }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0 | 1 | 255 => Ok(Self::from_value_no_check(value)),
            _ => Err(anyhow!("invalid CFListType value: {}", value)),
        }
    }
}
