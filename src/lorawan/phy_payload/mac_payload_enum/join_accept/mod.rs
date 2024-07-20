pub mod jadl_settings;
pub mod cf_list;


use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };
use std::borrow::Cow;

use jadl_settings::JADLSettings;
use cf_list::CFList;

//
// JoinAccept{12|28}
//     JoinNonce{3}
//     Home_NetID{3}
//     DevAddr{4}
//     JADLSettings{1}
//     RXDelay{1}
//     CFList{16|0} - ( CFList{16} | None{0} )
//

// ********************************
// * JoinAcceptComps
// ********************************

#[derive(Debug)]
pub struct JoinAcceptComps<'a> {
    pub join_nonce: u32,             // 3 bytes
    pub home_netid: u32,             // 3 bytes
    pub dev_addr: u32,               // 4 bytes
    pub jadl_settings: JADLSettings, // 1 byte
    pub rx_delay: u8,                // 1 byte
    pub cf_list: Option<CFList<'a>>,     // 16 bytes
}
impl fmt::Display for JoinAcceptComps<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}JoinNonce:    {:06x}\n\
                {padding}Home_NetID:   {:06x}\n\
                {padding}DevAddr:      {:08x}\n\
                {padding}JADLSettings: {:4}\n\
                {padding}RXDelay:      {}\n\
                {padding}CFList:       {}\n\
            ", 
            self.join_nonce,
            self.home_netid,
            self.dev_addr,
            self.jadl_settings,
            self.rx_delay,
            match &self.cf_list {
                Some(v) => format!("{:?}", v), // TODO: to implement Display for CFList
                None => "".to_owned(),
            },
        )
    }
}



//********************************
//* JoinAccept
//********************************

#[derive(Debug)]
pub struct JoinAccept<'a> {
    pub bytes: Cow<'a, [u8]>,  // 12|28 bytes
}
impl<'a> JoinAccept<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) }
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        match bytes.len() {
            12 => { 
                Ok( Self::from_bytes_no_check(bytes) )
            },
            28 => {
                match bytes[27] {
                    0 | 1 | 255 => { 
                        Ok(JoinAccept{ bytes: Cow::Borrowed(bytes) })
                    },
                    other => {
                        Err(anyhow!("Invalid CFListType value: {}", other))
                    }
                }
            },
            other => { 
                Err(anyhow!("Invalid JoinAccept frame length: {}", other)) 
            }
        }
    }
    pub fn from_comps(comps: &JoinAcceptComps) -> JoinAccept<'a> {
        let mut bytes: Vec<u8> = Vec::with_capacity(28);
        bytes.extend_from_slice(&comps.join_nonce.to_le_bytes()[0..3]);
        bytes.extend_from_slice(&comps.home_netid.to_le_bytes()[0..3]);
        bytes.extend(comps.dev_addr.to_le_bytes());
        bytes.push(comps.jadl_settings.as_value());
        bytes.push(comps.rx_delay);
        if let Some(cf_list) = &comps.cf_list {
            bytes.extend_from_slice(cf_list.as_bytes());
        }
        JoinAccept{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_comps(&self) -> JoinAcceptComps {
        JoinAcceptComps {
            join_nonce: self.join_nonce(),
            home_netid: self.home_netid(),
            dev_addr: self.dev_addr(),
            jadl_settings: self.jadl_settings(),
            rx_delay: self.rx_delay(),
            cf_list: self.cf_list(),
        }
    }

    pub fn join_nonce(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..3].try_into().unwrap())
    }
    pub fn home_netid(&self) -> u32 {
        u32::from_le_bytes(self.bytes[3..6].try_into().unwrap())
    }
    pub fn dev_addr(&self) -> u32 {
        u32::from_le_bytes(self.bytes[6..10].try_into().unwrap())
    }
    pub fn jadl_settings(&self) -> JADLSettings {
        JADLSettings::from_value_no_check(self.bytes[10])
    }
    pub fn rx_delay(&self) -> u8 {
        self.bytes[11]
    }
    pub fn cf_list(&self) -> Option<CFList> {
        if self.bytes.len() == 28 {
            Some(CFList::from_bytes_no_check(&self.bytes[12..28]))
        } else {
            None
        }
    }
    pub fn mic(&self) -> [u8; 4] {
        let len = self.bytes.len();
        self.bytes[len-4..len].try_into().unwrap()
    }
}

impl fmt::Display for JoinAccept<'_> {
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


