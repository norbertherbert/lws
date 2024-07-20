use std::fmt;
use anyhow::Result as AnyResult;

//
// JADLSettings{1}
//     OptNeg{.1}
//     RX1DROffset{.3}
//     RX2DataRate{.4}
//

// ********************************
// * JADLSettingsComps
// ********************************

#[derive(Debug)]
pub struct JADLSettingsComps {
    pub opt_neg: bool,      // 1 bit
    pub rx1_dr_offset: u8,  // 3 bits
    pub rx2_data_rate: u8,  // 4 bits
}
impl fmt::Display for JADLSettingsComps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let padding = " ".repeat(width);
        write!(f, 
            "\
                {padding}OptNeg:      {}\n\
                {padding}RX1DROffset: {}\n\
                {padding}RX2DataRate: {}\
            ", 
            self.opt_neg,
            self.rx1_dr_offset,
            self.rx2_data_rate,
        )
    }
}


//********************************
//* JADLSettings
//********************************

#[derive(Debug)]
pub struct JADLSettings {
    value: u8,
}
impl JADLSettings {

    pub fn from_value_no_check(value: u8) -> Self {
        Self{ value }
    }
    pub fn from_value(value: u8) -> AnyResult<Self> {

        // TODO: to verify if necessary!

        Ok( Self{ value } )
    }
    pub fn from_comps(comps: JADLSettingsComps) -> JADLSettings {
        let value = ((comps.opt_neg as u8) << 7) & ((comps.rx1_dr_offset as u8) << 4) & (comps.rx2_data_rate as u8);
        JADLSettings{ value }
    }

    pub fn as_value(&self) -> u8 {
        self.value
    }
    pub fn to_comps(&self) -> JADLSettingsComps {
        JADLSettingsComps {
            opt_neg: self.opt_neg(),
            rx1_dr_offset: self.rx1_dr_offset(),
            rx2_data_rate: self.rx2_data_rate(),
        }
    }

    pub fn opt_neg(&self) -> bool {
        (self.value & 0b10000000) == 0b10000000
    }
    pub fn rx1_dr_offset(&self) -> u8 {
        (self.value & 0b01110000) >> 4
    }
    pub fn rx2_data_rate(&self) -> u8 {
        self.value & 0b00001111
    }

}
impl fmt::Display for JADLSettings {
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