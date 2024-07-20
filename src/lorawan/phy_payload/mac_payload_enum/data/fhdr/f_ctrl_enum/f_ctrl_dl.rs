use std::fmt;
use anyhow::Result as AnyResult;

//
// FCtrlDL{1}
//     ADR{.1}
//     RFU{.1}
//     ACK{.1}
//     FPending{.1}
//     FOptsLen{.4}
//

//********************************
//* FCtrlDLComps
//********************************

#[derive(Debug)]
pub struct FCtrlDLComps {
    pub adr: bool,        // 1 bit
    pub rfu: bool,        // 1 bit
    pub ack: bool,        // 1 bit
    pub f_pending: bool,  // 1 bit  
    pub f_opts_len: u8,   // 4 bits
}
impl fmt::Display for FCtrlDLComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}ADR:       {}\n\
                {padding}RFU:       {}\n\
                {padding}ACK:       {}\n\
                {padding}FPending:  {}\n\
                {padding}FOptsLen:  {}\
            ", 
            self.adr,
            self.rfu,
            self.ack,
            self.f_pending,
            self.f_opts_len,
        )
    }
}

#[test]
fn test_f_ctrl_dl() {
    let f_ctrl_dl = FCtrlDLComps{
        adr: true,
        rfu: true,
        ack: true,
        f_pending: false,
        f_opts_len: 0b1010,
    };
    println!("DISPLAY FCtrlDL:\n{:4}", f_ctrl_dl);
}

//********************************
//* FCtrlDL
//********************************

pub struct FCtrlDL {
    value: u8,
}
impl FCtrlDL {

    pub fn from_value_no_check(value: u8) -> Self {
        Self{ value }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        // TODO: Check if needed!
        Ok( Self::from_value_no_check(value) )
    }

    pub fn from_comps(comps: &FCtrlDLComps) -> FCtrlDL {
        let value = ((comps.adr as u8) << 7) & ((comps.rfu as u8) << 6) & ((comps.ack as u8) << 5) & ((comps.f_pending as u8) << 4) & (comps.f_opts_len & 0b00001111);
        FCtrlDL{ value }
    }

    pub fn as_value(&self) -> u8 {
        self.value
    }
    pub fn to_comps(&self) -> FCtrlDLComps {
        FCtrlDLComps {
            adr: self.adr(),
            rfu: self.rfu(),
            ack: self.ack(),
            f_pending: self.f_pending(),
            f_opts_len: self.f_opts_len(),
        }
    }

    pub fn adr(&self) -> bool {
        (self.value & 0b10000000) == 0b10000000
    }
    pub fn rfu(&self) -> bool {
        (self.value & 0b01000000) == 0b01000000
    }
    pub fn ack(&self) -> bool {
        (self.value & 0b00100000) == 0b00100000
    }
    pub fn f_pending(&self) -> bool {
        (self.value & 0b00010000) == 0b00010000
    }
    pub fn f_opts_len(&self) -> u8 {
        self.value & 0b00001111
    }

}
impl fmt::Display for FCtrlDL {
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

#[test]
fn test_f_ctrl_dl1() {
    let f_ctrl_dl = FCtrlDL{ value: 0x36 };
    println!("DISPLAY FCtrlDL:\n{:4}", f_ctrl_dl);
}
