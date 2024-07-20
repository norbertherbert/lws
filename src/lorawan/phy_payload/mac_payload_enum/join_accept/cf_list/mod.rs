
pub mod cf_list_content_enum;
pub mod cf_list_type;

use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

use cf_list_type::CFListType;
use cf_list_content_enum::{
    CFListContentEnum, 
    dynamic_channel_list::DynamicChannelList,
    fixed_channel_mask::FixedChannelMask, 
    new_join_eui_and_js_cookie::NewJoinEUIandJSCookie
};

//
// CFList{16}
//     CFListContent{15}
//     CFListType{1}
//

// ********************************
// * CFListComps
// ********************************

#[derive(Debug)]
pub struct CFListComps<'a> {
    pub cf_list_content: CFListContentEnum<'a>, // 15 bytes
    pub cf_list_type: CFListType,       // 1 byte
}
impl fmt::Display for CFListComps<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}CFListContentEnum: {}\n\
                {padding}CFListType:    {:?}\
            ", 
            self.cf_list_content,
            self.cf_list_type,
        )
    }
}


// ********************************
// * CFList
// ********************************

#[derive(Debug)]
pub struct CFList<'a> {
    pub bytes: Cow<'a, [u8]>,  // 16 bytes
}
impl<'a> CFList<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) } 
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            16 => { 
                Ok( Self::from_bytes_no_check(bytes) ) 
            },
            other => { 
                Err(anyhow!("Invalid length: {}", other)) 
            }
        }
    }
    pub fn from_comps(comps: &CFListComps) -> CFList<'a> {
        let mut bytes: Vec<u8> = Vec::with_capacity(16);
        let b = match &comps.cf_list_content {
            CFListContentEnum::DynamicChannelList(v) => v,
            CFListContentEnum::FixedChannelMask(v) => v.as_bytes(),
            CFListContentEnum::NewJoinEUIandJSCookie(v) => &v.as_bytes(),
        };
        bytes.extend(b);
        bytes.push(comps.cf_list_type as u8);
        CFList{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_comps(&self) -> CFListComps {
        CFListComps {
            cf_list_content: self.cf_list_content(),
            cf_list_type: self.cf_list_type(),
        }
    }

    pub fn cf_list_content(&self) -> CFListContentEnum {

        match CFListType::from_value_no_check(self.bytes[15]) {
            CFListType::DynamicChannelList => {
                CFListContentEnum::DynamicChannelList(
                    self.bytes[0..15].try_into().unwrap()
                ) 
            },
            CFListType::FixedChannelMask => { 
                CFListContentEnum::FixedChannelMask(
                    FixedChannelMask::from_bytes_no_check(&self.bytes[0..15])
                )
            },
            CFListType::NewJoinEUIandJSCookie => {
                CFListContentEnum::NewJoinEUIandJSCookie(
                    NewJoinEUIandJSCookie::from_bytes_no_check(&self.bytes[0..15])
                )
            },
        }
    }

    pub fn dynamic_channel_list(&self) -> DynamicChannelList {
        self.bytes[0..15].try_into().unwrap()
    }
    pub fn fixed_channel_mask(&self) -> FixedChannelMask {
        FixedChannelMask::from_bytes_no_check(&self.bytes[0..15])
    }
    pub fn new_join_eui_and_js_cookie(&self) -> NewJoinEUIandJSCookie {
        NewJoinEUIandJSCookie::from_bytes_no_check(&self.bytes[0..15])
    }

    pub fn cf_list_type(&self) -> CFListType {
        CFListType::from_value_no_check(self.bytes[15])
    }

}
impl fmt::Display for CFList<'_> {
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

