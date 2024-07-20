use std::fmt;
use anyhow::Result as AnyResult;

//
// FCtrlUL{1}
//     ADR{.1}
//     ADRACKReq{.1}
//     ACK{.1}
//     ClassB{.1}
//     FOptsLen{.4}
//

//********************************
//* FCtrlULComps
//********************************

#[derive(Debug)]
pub struct FCtrlULComps {
    pub adr: bool,          // 1 bit
    pub adr_ack_req: bool,  // 1 bit
    pub ack: bool,          // 1 bit
    pub class_b: bool,      // 1 bit
    pub f_opts_len: u8,     // 4 bits
}
impl fmt::Display for FCtrlULComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}ADR:       {}\n\
                {padding}AdrAckReq: {}\n\
                {padding}ACK:       {}\n\
                {padding}ClassB:    {}\n\
                {padding}FOptsLen:  {}\
            ", 
            self.adr,
            self.adr_ack_req,
            self.ack,
            self.class_b,
            self.f_opts_len,
        )
    }
}

#[test]
fn test_f_ctrl_ul() {
    let f_ctrl_ul = FCtrlULComps {
        adr: true,
        adr_ack_req: true,
        ack: true,
        class_b: false,
        f_opts_len: 0b1010,
    };
    println!("DISPLAY FCtrlUL:\n{:4}", f_ctrl_ul);
}


//********************************
//* FCtrlUL
//********************************

pub struct FCtrlUL {
    value: u8,
}
impl FCtrlUL {

    pub fn from_value_no_check(value: u8) -> Self {
        Self{ value }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {
        // TODO: Check if needed!
        Ok( Self::from_value_no_check(value) )
    }
    pub fn from_comps(comps: &FCtrlULComps) -> FCtrlUL {
        let value = ((comps.adr as u8) << 7) & ((comps.adr_ack_req as u8) << 6) & ((comps.ack as u8) << 5) & ((comps.class_b as u8) << 4) & (comps.f_opts_len & 0b00001111);
        FCtrlUL{ value }
    }

    pub fn as_value(&self) -> u8 {
        self.value
    }
    pub fn to_comps(&self) -> FCtrlULComps {
        FCtrlULComps {
            adr: self.adr(),
            adr_ack_req: self.adr_ack_req(),
            ack: self.ack(),
            class_b: self.class_b(),
            f_opts_len: self.f_opts_len(),
        }
    }

    pub fn adr(&self) -> bool {
        (self.value & 0b10000000) == 0b10000000
    }
    pub fn adr_ack_req(&self) -> bool {
        (self.value & 0b01000000) == 0b01000000
    }
    pub fn ack(&self) -> bool {
        (self.value & 0b00100000) == 0b00100000
    }
    pub fn class_b(&self) -> bool {
        (self.value & 0b00010000) == 0b00010000
    }
    pub fn f_opts_len(&self) -> u8 {
        self.value & 0b00001111
    }

}
impl fmt::Display for FCtrlUL {
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
fn test_f_ctrl_ul1() {
    let f_ctrl_ul = FCtrlUL{ value: 0x36 };
    println!("DISPLAY FCtrlUL:\n{:4}", f_ctrl_ul);
}
