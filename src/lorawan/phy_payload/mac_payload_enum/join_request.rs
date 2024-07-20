use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

//
// JoinRequest{18}
//     JoinEUI{8}
//     DevEUI{8}
//     DevNonce{2}
//

// ********************************
// * JoinRequestComps
// ********************************

#[derive(Debug)]
pub struct JoinRequestComps {
    pub join_eui: u64,   // 8 bytes
    pub dev_eui: u64,    // 8 bytes
    pub dev_nonce: u16,  // 2 bytes
}
impl fmt::Display for JoinRequestComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}JoinEUI:  {:016x}\n\
                {padding}DevEUI:   {:016x}\
                {padding}DevNonce: {:04x}\
            ", 
            self.join_eui,
            self.dev_eui,
            self.dev_nonce,
        )
    }
}


// ********************************
// * JoinRequest
// ********************************

#[derive(Debug)]
pub struct JoinRequest<'a> {
    bytes: Cow<'a, [u8]>,    // 18 bytes
}
impl<'a> JoinRequest<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) } 
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            18 => { 
                Ok( Self::from_bytes_no_check(bytes) ) 
            },
            other => { 
                Err(anyhow!("Invalid length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: &JoinRequestComps) -> JoinRequest {
        let mut bytes: Vec<u8> = Vec::with_capacity(18);
        bytes.extend(comps.join_eui.to_le_bytes());
        bytes.extend(comps.dev_eui.to_le_bytes());
        bytes.extend(comps.dev_nonce.to_le_bytes());
        JoinRequest{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_comps(&self) -> JoinRequestComps {
        JoinRequestComps {
            join_eui: self.join_eui(),
            dev_eui: self.dev_eui(),
            dev_nonce: self.dev_nonce(),
        }
    }

    pub fn join_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[0..8].try_into().unwrap())
    }
    pub fn dev_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[8..16].try_into().unwrap())
    }
    pub fn dev_nonce(&self) -> u16 {
        u16::from_le_bytes(self.bytes[16..18].try_into().unwrap())
    }

}


impl fmt::Display for JoinRequest<'_> {
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