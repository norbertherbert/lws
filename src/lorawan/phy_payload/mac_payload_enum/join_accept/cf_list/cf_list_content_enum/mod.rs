pub mod fixed_channel_mask;
pub mod dynamic_channel_list;
pub mod new_join_eui_and_js_cookie;

use::std::fmt;

use fixed_channel_mask::{
    FixedChannelMaskComps, 
    FixedChannelMask
};
use dynamic_channel_list::DynamicChannelList;
use new_join_eui_and_js_cookie::{
    NewJoinEUIandJSCookieComps, 
    NewJoinEUIandJSCookie
};

// ********************************
// * CFListContentEnumComps
// ********************************

#[derive(Debug)]
pub enum CFListContentEnumComps {
    DynamicChannelList(DynamicChannelList),
    FixedChannelMask(FixedChannelMaskComps),
    NewJoinEUIandJSCookie(NewJoinEUIandJSCookieComps),
}
impl fmt::Display for CFListContentEnumComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            CFListContentEnumComps::DynamicChannelList(v) => {
                // let padding = " ".repeat(width);
                write!(f, "{}", hex::encode(v))
            }
            CFListContentEnumComps::FixedChannelMask(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            CFListContentEnumComps::NewJoinEUIandJSCookie(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
        }
    }
}


// ********************************
// * CFListContentEnum
// ********************************

#[derive(Debug)]
pub enum CFListContentEnum<'a> {
    DynamicChannelList(DynamicChannelList),
    FixedChannelMask(FixedChannelMask<'a>),
    NewJoinEUIandJSCookie(NewJoinEUIandJSCookie<'a>),
}
impl fmt::Display for CFListContentEnum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        match self {
            CFListContentEnum::DynamicChannelList(v) => {
                // let padding = " ".repeat(width);
                write!(f, "{}", hex::encode(v))
            }
            CFListContentEnum::FixedChannelMask(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
            CFListContentEnum::NewJoinEUIandJSCookie(v) => { 
                write!(f, "{:width$}", v, width=width) 
            }
        }
    }
}
