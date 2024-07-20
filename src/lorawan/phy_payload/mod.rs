
pub mod mhdr;
pub mod mac_payload_enum;
pub mod mic;

use std::fmt;
use anyhow::{
    Result as AnyResult, 
    anyhow
};
use std::borrow::Cow;

use mhdr::MHDR;
use mic::MIC;
use mac_payload_enum::{
    MACPayloadEnum, 
    join_request::JoinRequest,
    rejoin_request::RejoinRequest,
    join_accept::JoinAccept,
    data::Data,
};

//
// PhyPayload{12..M+5}
//     MHDR{1}
//     MACPayload{7..M} - ( JoinRequest{18} | RejoinRequest02{14} | RejoinRequest1{19} | JoinAccept{12|28} | Data{7..M} )  
//     MIC{4}
//

//********************************
//* PhyPayloadComps
//********************************

pub struct PhyPayloadComps<'a> {
    pub mhdr: MHDR,                    // 1 byte
    pub mac_payload: MACPayloadEnum<'a>,   // 7..M
    pub mic: MIC,                      // 4 bytes
}
impl fmt::Display for PhyPayloadComps<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}MHDR:       {:4}\n\
                {padding}MACPayload: {:4}\
                {padding}MIC:        {}\
            ", 
            self.mhdr,
            self.mac_payload,
            hex::encode(self.mic),
        )
    }
}


// ********************************
// * PhyPayload
// ********************************

#[derive(Debug)]
pub struct PhyPayload<'a> {
    bytes: Cow<'a, [u8]>,    // 12..M+5
}
impl<'a> PhyPayload<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) } 
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            12..=295 => { 
                Ok(Self::from_bytes_no_check(bytes)) 
            },
            other => { 
                Err(anyhow!("Invalid length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: &PhyPayloadComps) -> PhyPayload<'a> {
        let mut bytes: Vec<u8> = Vec::with_capacity(295);
        bytes.push(comps.mhdr.as_value());
        bytes.extend_from_slice(comps.mac_payload.as_bytes());
        bytes.extend(comps.mic);
        PhyPayload{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_comps(&self) -> PhyPayloadComps {
        let phy_payload_comps = PhyPayloadComps {
            mhdr: self.mhdr(),
            mac_payload: self.mac_payload(),
            mic: self.mic(),
        };
        phy_payload_comps
    }

    pub fn mhdr(&self) -> MHDR {
        MHDR::from_value_no_check(self.bytes[0])
    }

    pub fn mac_payload(&self) -> MACPayloadEnum {
        let l = self.bytes.len();
        let f_type = MHDR::from_value_no_check(self.bytes[0]).f_type();
        MACPayloadEnum::from_bytes_no_check(&self.bytes[1..l-4], f_type)
    }

    pub fn join_request(&self) -> JoinRequest { 
        let l = self.bytes.len();
        JoinRequest::from_bytes_no_check(&self.bytes[1..l-4])
    }

    pub fn rejoin_request(&self) -> RejoinRequest {
        let l = self.bytes.len();
        RejoinRequest::from_bytes_no_check(&self.bytes[1..l-4])
    }

    pub fn join_accept(&self) -> JoinAccept {
        let l = self.bytes.len();
        JoinAccept::from_bytes_no_check(&self.bytes[1..l-4])
    }
    pub fn data(&self) -> Data {
        let l = self.bytes.len();
        Data::from_bytes_no_check(&self.bytes[1..l-4])
    }

    pub fn mic(&self) -> MIC {
        let l = self.bytes.len();
        self.bytes[l-4..].try_into().unwrap()
    }

}
impl fmt::Display for PhyPayload<'_> {
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