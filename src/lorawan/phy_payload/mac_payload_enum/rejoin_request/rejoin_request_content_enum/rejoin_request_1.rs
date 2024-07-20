
use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

//
// RejoinRequest1{18}
//     JoinEUI{8}
//     DevEUI{8}
//     RJCount1{2}
//

// ********************************
// * RejoinRequest1Comps
// ********************************

pub struct RejoinRequest1Comps {
    pub join_eui: u64,            // 8 bytes
    pub dev_eui: u64,             // 8 bytes
    pub rj_count1: u16,           // 2 bytes
}
impl fmt::Display for RejoinRequest1Comps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}JoinEUI:      {:016x}\n\
                {padding}DevEUI:     {:016x}\
                {padding}RJCount1:   {:04x}\
            ",
            self.join_eui,
            self.dev_eui,
            self.rj_count1,
        )
    }
}

// ********************************
// * RejoinRequest1
// ********************************

#[derive(Debug)]
pub struct RejoinRequest1<'a> {
    bytes: Cow<'a, [u8]>,    // 19 bytes
}
impl<'a> RejoinRequest1<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) }
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            18 => { 
                match bytes[0] {
                    1 => {
                        Ok( Self::from_bytes_no_check(bytes) )
                    },
                    other => {
                        Err(anyhow!("Invalid RejoinType Value: {:?}", other))
                    }
                }
            },
            other => { 
                Err(anyhow!("Invalid RejoinRequest1 Frame length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: &RejoinRequest1Comps) -> RejoinRequest1 {
        let mut bytes: Vec<u8> = Vec::with_capacity(18);
        bytes.extend(comps.join_eui.to_le_bytes());
        bytes.extend(comps.dev_eui.to_le_bytes());
        bytes.extend(comps.rj_count1.to_le_bytes());
        RejoinRequest1{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub fn to_comps(&self) -> RejoinRequest1Comps {
        RejoinRequest1Comps {
            join_eui: self.join_eui(),
            dev_eui: self.dev_eui(),
            rj_count1: self.rj_count1(),
        }
    }

    pub fn join_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[0..8].try_into().unwrap())
    }
    pub fn dev_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[8..16].try_into().unwrap())
    }
    pub fn rj_count1(&self) -> u16 {
        u16::from_le_bytes(self.bytes[16..18].try_into().unwrap())
    }

}
impl fmt::Display for RejoinRequest1<'_> {
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