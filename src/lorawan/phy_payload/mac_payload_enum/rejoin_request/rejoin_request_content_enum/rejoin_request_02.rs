use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

//
// RejoinRequest02{13}
//     NetID{3}
//     DevEUI{8}
//     RJCount02{2}
//

// ********************************
// * RejoinRequest02Comps
// ********************************

pub struct RejoinRequest02Comps {
    pub net_id: u32,              // 3 bytes
    pub dev_eui: u64,             // 8 bytes
    pub rj_count02: u16,          // 2 bytes
}
impl fmt::Display for RejoinRequest02Comps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}NetID:      {:06x}\n\
                {padding}DevEUI:     {:016x}\
                {padding}RJCount02:  {:04x}\
            ",
            self.net_id,
            self.dev_eui,
            self.rj_count02,
        )
    }
}

// ********************************
// * RejoinRequest02
// ********************************

#[derive(Debug)]
pub struct RejoinRequest02<'a> {
    bytes: Cow<'a, [u8]>,    // 14 bytes
}
impl<'a> RejoinRequest02<'a> {
    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) }
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            13 => { 
                match bytes[0] {
                    0 | 2 => {
                        Ok( Self::from_bytes_no_check(bytes) )
                    },
                    other => {
                        Err(anyhow!("Invalid RejoinType Value: {:?}", other))
                    }
                }
            },
            other => { 
                Err(anyhow!("Invalid RejoinRequest02 Frame length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: &RejoinRequest02Comps) -> RejoinRequest02 {
        let mut bytes: Vec<u8> = Vec::with_capacity(13);
        bytes.extend_from_slice(&comps.net_id.to_le_bytes()[..3]);
        bytes.extend(&comps.dev_eui.to_le_bytes());
        bytes.extend(&comps.rj_count02.to_le_bytes());
        RejoinRequest02{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub fn to_comps(&self) -> RejoinRequest02Comps {
        RejoinRequest02Comps {
            net_id: self.net_id(),
            dev_eui: self.dev_eui(),
            rj_count02: self.rj_count02(),
        }
    }

    pub fn net_id(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..3].try_into().unwrap())
    }
    pub fn dev_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[3..11].try_into().unwrap())
    }
    pub fn rj_count02(&self) -> u16 {
        u16::from_le_bytes(self.bytes[11..13].try_into().unwrap())
    }

}
impl fmt::Display for RejoinRequest02<'_> {
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
