use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

// ********************************
// * FixedChannelMaskComps
// ********************************

#[derive(Debug)]
pub struct FixedChannelMaskComps {
    pub ch4freq: u32,             // 3 bytes
    pub ch5freq: u32,             // 3 bytes
    pub ch6freq: u32,             // 3 bytes
    pub ch7freq: u32,             // 3 bytes
    pub ch8freq: u32,             // 3 bytes
}
impl fmt::Display for FixedChannelMaskComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}ch4freq:      {}\n\
                {padding}ch5freq:      {}\n\
                {padding}ch6freq:      {}\n\
                {padding}ch7freq:      {}\n\
                {padding}ch8freq:      {}\n\
            ", 
            self.ch4freq,
            self.ch5freq,
            self.ch6freq,
            self.ch7freq,
            self.ch8freq,
        )
    }
}

// ********************************
// * FixedChannelMask
// ********************************

#[derive(Debug)]
pub struct FixedChannelMask<'a> {
    pub bytes: Cow<'a, [u8]>,  // 15 bytes
}

impl<'a> FixedChannelMask<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) } 
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            15 => { 
                Ok( Self::from_bytes_no_check(bytes) ) 
            },
            other => { 
                Err(anyhow!("Invalid length: {}", other)) 
            }
        }
    }
    pub fn from_comps(comps: &FixedChannelMaskComps) -> FixedChannelMask {
        let mut bytes: Vec<u8> = Vec::with_capacity(15);
        bytes.extend_from_slice(&comps.ch4freq.to_le_bytes()[0..3]);
        bytes.extend_from_slice(&comps.ch5freq.to_le_bytes()[0..3]);
        bytes.extend_from_slice(&comps.ch6freq.to_le_bytes()[0..3]);
        bytes.extend_from_slice(&comps.ch7freq.to_le_bytes()[0..3]);
        bytes.extend_from_slice(&comps.ch8freq.to_le_bytes()[0..3]);
        FixedChannelMask{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_comps(&self) -> FixedChannelMaskComps {
        FixedChannelMaskComps {
            ch4freq: self.ch4freq(),
            ch5freq: self.ch5freq(),
            ch6freq: self.ch6freq(),
            ch7freq: self.ch7freq(),
            ch8freq: self.ch8freq(),
        }
    }

    pub fn ch4freq(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..3].try_into().unwrap())
    }
    pub fn ch5freq(&self) -> u32 {
        u32::from_le_bytes(self.bytes[3..6].try_into().unwrap())
    }
    pub fn ch6freq(&self) -> u32 {
        u32::from_le_bytes(self.bytes[6..9].try_into().unwrap())
    }
    pub fn ch7freq(&self) -> u32 {
        u32::from_le_bytes(self.bytes[9..12].try_into().unwrap())
    }
    pub fn ch8freq(&self) -> u32 {
        u32::from_le_bytes(self.bytes[12..15].try_into().unwrap())
    }

}


impl fmt::Display for FixedChannelMask<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(
            f, 
            "\
                {padding}Value:    {}\n\
                {:width$}\
            ",
            hex::encode(&self.bytes),
            self.to_comps(),
            width = width
        )
    }
}




