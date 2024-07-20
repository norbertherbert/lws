pub mod fhdr;

use std::fmt;
use anyhow::{
    Result as AnyResult,
    anyhow
};
use std::borrow::Cow;

use fhdr::FHDR;

//
// Data{7..M}
//     FHDR{7..22}
//     FPort{1|0} - ( FPort{1} | None{0} )
//     FRMPayload{0..(M-8)}
//

// ********************************
// * DataComps
// ********************************

pub struct DataComps<'a> {
    fhdr: FHDR<'a>,                 // 7..M (M for SF7 = 230)
    f_port: Option<u8>,             // 0 | 1
    frm_payload: Option<Vec<u8>>,   // 0..(M-8)
}
impl fmt::Display for DataComps<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}FHDR:       {:4}\n\
                {padding}FPort:      {}\n\
                {padding}FRMPayload: {}\
            ", 
            self.fhdr,
            match self.f_port {
                Some(v) => format!("{}", v),
                None => "".to_string(),
            },
            match &self.frm_payload {
                Some(v) => hex::encode(v),
                None => "".to_string(),
            },
        )
    }
}


// ********************************
// * Data
// ********************************
#[derive(Debug)]
pub struct Data<'a> {
    bytes: Cow<'a, [u8]>,    // 7..M (M for SF7 = 230)
}
impl<'a> Data<'a> {

    pub fn from_bytes_no_check(bytes: &'a [u8]) -> Self {
        Self{ bytes: Cow::Borrowed(bytes) } 
    }
    pub fn from_bytes(bytes: &'a [u8]) -> AnyResult<Self> {
        let len = bytes.len();
        match len {
            7..=230 => {

                let fhdr_len = 7 + (bytes[5] >> 4) as usize;

                if fhdr_len > len || fhdr_len + 1 == len {
                    return Err(anyhow!("Invalid Data frame length: {}", len));
                }

                Ok( Self::from_bytes_no_check(bytes) ) 
            },
            other => { 
                Err(anyhow!("Invalid Data frame length: {}", other)) 
            }
        }
    }

    pub fn from_comps(comps: &DataComps) -> Data<'a> {
        let mut bytes = Vec::with_capacity(230);
        bytes.extend_from_slice(comps.fhdr.as_bytes());
        if let Some(v) = comps.f_port {
            bytes.push(v);
        }
        if let Some(v) = &comps.frm_payload {
            bytes.extend(v);
        }
        Data{ bytes: Cow::Owned(bytes) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_comps(&self) -> DataComps {
        DataComps {
            // TODO: to be optimized so that l is not fetched multiple times
            fhdr: self.fhdr(),
            f_port: self.f_port(),
            frm_payload: self.frm_payload(),
        }
    }

    pub fn fhdr(&self) -> FHDR {
        let fhdr_len = (7 + (self.bytes[4] >> 4)) as usize;
        FHDR::from_bytes_no_check( &self.bytes[..fhdr_len] )
    }

    pub fn f_port(&self) -> Option<u8> {
        let fhdr_len = (7 + (self.bytes[4] >> 4)) as usize;
        if self.bytes.len() == fhdr_len { 
            return None;
        } else {
            return Some(self.bytes[fhdr_len]);
        }
    }

    pub fn frm_payload(&self) -> Option<Vec<u8>> {
        let fhdr_len = (7 + (self.bytes[4] >> 4)) as usize;
        if self.bytes.len() == fhdr_len { 
            return None;
        } else {
            return Some(self.bytes[fhdr_len..].to_vec());
        }
    }

}
impl fmt::Display for Data<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(
            f, 
            "\
                {padding}Bytes:     {}\n\
                {:width$}\
            ",
            hex::encode(&self.bytes),
            self.to_comps(),
            width = width
        )
    }
}


// ********************************
// M Values:
//     SF12 59
//     SF11 59
//     SF10 59
//     SF9  123
//     SF8  230
//     SF7  230
// ********************************
