pub mod f_type;
pub mod dir;
pub mod major;

use std::fmt;
use anyhow::{ Result as AnyResult, anyhow };

use f_type::FType;
use major::Major;

//
// MHDR{1}
//     FType{.3}
//     RFU{.3}
//     Mayor{.2}
//

//********************************
//* MHDRComps
//********************************

#[derive(Debug)]
pub struct MHDRComps {
    pub f_type: FType,
    pub rfu: u8,
    pub major: Major,
}
impl fmt::Display for MHDRComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}FType: {:?}\n\
                {padding}RFU:   {:?}\n\
                {padding}Major: {:?}\
            ", 
            self.f_type,
            self.rfu,
            self.major,
        )
    }
}


//********************************
//* MHDR
//********************************

pub struct MHDR {
    value: u8,
}
impl MHDR {


    pub fn from_value_no_check(value: u8) -> Self {
        Self{ value }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        let f_type_value = (value & 0b11100000) >> 5;
        if f_type_value > 6 {
            return Err(anyhow!("Invalid FType value: {}", f_type_value));
        }
        let major_value = value & 0b00000011;
        if major_value > 0 {
            return Err(anyhow!("Invalid Major value: {}", major_value));
        }
        Ok(Self::from_value_no_check(value))
    }


    pub fn from_comps(comps: MHDRComps) -> MHDR {
        let value = ((comps.f_type as u8) << 7) & ((comps.rfu as u8) << 6) & ((comps.major as u8) << 5);
        MHDR{ value }
    }

    pub fn as_value(&self) -> u8 {
        self.value
    }
    pub fn to_comps(&self) -> MHDRComps {
        let mhdr_comps = MHDRComps {
            f_type: self.f_type(),
            rfu: self.rfu(),
            major: self.major(),
        };
        mhdr_comps
    }

    pub fn f_type(&self) -> FType {
        FType::from_value_no_check((self.value & 0b11100000) >> 5)
    }
    pub fn rfu(&self) -> u8 {
        (self.value & 0b00011100) >> 2
    }
    pub fn major(&self) -> Major {
        Major::from_value_no_check(self.value & 0b00000011)
    }

}
impl fmt::Display for MHDR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(
            f, 
            "\
                {padding}Value:     0x{:02x}\n\
                {:width$}\
            ",
            self.value,
            self.to_comps(),
            width = width
        )
    }
}