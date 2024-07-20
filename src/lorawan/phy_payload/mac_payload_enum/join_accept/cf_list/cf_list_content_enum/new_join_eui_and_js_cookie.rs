use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

// ********************************
// * NewJoinEUIandJSCookieComps
// ********************************

#[derive(Debug)]
pub struct NewJoinEUIandJSCookieComps {
    pub new_join_eui:    u64,   // 8 bytes
    pub security_cookie: u64,   // 7 bytes
}
impl fmt::Display for NewJoinEUIandJSCookieComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}NewJoinEUI:     {}\n\
                {padding}SecurityCookie: {}\
            ", 
            self.new_join_eui,
            self.security_cookie,
        )
    }
}

// ********************************
// * NewJoinEUIandJSCookie
// ********************************

#[derive(Debug)]
pub struct NewJoinEUIandJSCookie<'a> {
    pub bytes: Cow<'a, [u8]>,  // 15 bytes
}
impl<'a> NewJoinEUIandJSCookie<'a> {

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
    pub fn from_comps(comps: &NewJoinEUIandJSCookieComps) -> NewJoinEUIandJSCookie {
        let mut bytes: Vec<u8> = Vec::with_capacity(15);
        bytes.extend(&comps.new_join_eui.to_le_bytes());
        bytes.copy_from_slice(&comps.security_cookie.to_le_bytes()[..7]);
        NewJoinEUIandJSCookie{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub fn to_comps(&self) -> NewJoinEUIandJSCookieComps {
        NewJoinEUIandJSCookieComps {
            new_join_eui: self.new_join_eui(),
            security_cookie: self.security_cookie(),
        }
    }

    pub fn new_join_eui(&self) -> u64 {
        u64::from_le_bytes(self.bytes[0..8].try_into().unwrap())
    }
    pub fn security_cookie(&self) -> u64 {
        u64::from_le_bytes(self.bytes[8..15].try_into().unwrap())
    }

}
impl fmt::Display for NewJoinEUIandJSCookie<'_> {
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
