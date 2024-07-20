pub mod rejoin_type;
pub mod rejoin_request_content_enum;

// use std::fmt;
// use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

use rejoin_type::RejoinType;
use rejoin_request_content_enum::{
    rejoin_request_02::RejoinRequest02,
    rejoin_request_1::RejoinRequest1,
};


// ********************************
// * RejoinRequest
// ********************************

#[derive(Debug)]
pub struct RejoinRequest<'a> {
    bytes: Cow<'a, [u8]>,    // 19 bytes
}
impl<'a> RejoinRequest<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) }
    }
    // pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
    //     match bytes.len() {
    //         19 => { 
    //             match bytes[0] {
    //                 1 => {
    //                     Ok( Self::from_bytes_no_check(bytes) )
    //                 },
    //                 other => {
    //                     Err(anyhow!("Invalid RejoinType Value: {:?}", other))
    //                 }
    //             }
    //         },
    //         other => { 
    //             Err(anyhow!("Invalid RejoinRequest1 Frame length: {}", other)) 
    //         }
    //     }
    // }

    // pub fn from_comps(comps: &RejoinRequest1Comps) -> RejoinRequest1 {
    //     let mut bytes: Vec<u8> = Vec::with_capacity(19);
    //     bytes.push(comps.rejoin_type as u8);
    //     bytes.extend(comps.join_eui.to_le_bytes());
    //     bytes.extend(comps.dev_eui.to_le_bytes());
    //     bytes.extend(comps.rj_count1.to_le_bytes());
    //     RejoinRequest1{ bytes: Cow::Owned(bytes) }
    // }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    // pub fn to_comps(&self) -> RejoinRequest1Comps {
    //     RejoinRequest1Comps {
    //         rejoin_type: self.rejoin_type(),
    //         join_eui: self.join_eui(),
    //         dev_eui: self.dev_eui(),
    //         rj_count1: self.rj_count1(),
    //     }
    // }

    pub fn rejoin_type(&self) -> RejoinType {
        RejoinType::from_value_no_check(self.bytes[0])
    }

    // pub fn rejoin_request_content_enum(&self) -> RejoinRequestContentEnum {
    //     RejoinRequestContentEnum::from_bytes_no_check(&self.bytes[1..])
    // }

    pub fn rejoin_request_02(&self) -> RejoinRequest02 {
        RejoinRequest02::from_bytes_no_check(&self.bytes[1..])
    }
    pub fn rejoin_request_1(&self) -> RejoinRequest1 {
        RejoinRequest1::from_bytes_no_check(&self.bytes[1..])
    }

}
// impl fmt::Display for RejoinRequest<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let width = f.width().unwrap_or(0);
//         let padding = " ".repeat(width);
//         write!(
//             f, 
//             "\
//                 {padding}Value:    {}\n\
//                 {:width$}\
//             ",
//             hex::encode(&self.bytes),
//             self.to_comps(),
//             width = width
//         )
//     }
// }