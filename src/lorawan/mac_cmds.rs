
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MACCmdUL {

    // *******************************************
    // Class A commands (0x20 to 0x2F) 
    // *******************************************

    //                    0x00, // RFU
    ResetInd            = 0x01, // (1.1)   Used by an ABP end-device to indicate a reset and negotiate protocol version. 
    LinkCheckReq        = 0x02, // (1.0.0) Used by an end-device to validate its connectivity to a network.
    LinkADRAns          = 0x03, // (1.0)   Acknowledges LinkADRReq.
    DutyCycleAns        = 0x04, // (1.0)   Acknowledges DutyCycleReq.
    RXParamSetupAns     = 0x05, // (1.0)   Acknowledges RXParamSetupReq. 
    DevStatusAns        = 0x06, // (1.0)   Returns the status of the end-device, i.e., battery level and radio status.
    NewChannelAns       = 0x07, // (1.0)   Acknowledges NewChannelReq.
    RXTimingSetupAns    = 0x08, // (1.0)   Acknowledges RXTimingSetupReq.
    TXParamSetupAns     = 0x09, // (1.0.2) Acknowledges TXParamSetupReq.
    DlChannelAns        = 0x0a, // (1.0.2) Acknowledges DlChannelReq.
    RekeyInd            = 0x0b, // (1.1)   Used by an OTAA end-device to signal a session context update (rekeying)
    ADRParamSetupAns    = 0x0c, // (1.1)   Acknowledges ADRParamSetupReq.
    DeviceTimeReq       = 0x0d, // (1.0.3) Used by an end-device to request the current GPS time.
    //                    0x0e, //
    RejoinParamSetupAns = 0x0f, // (1.1)   Acknowledges RejoinParamSetupReq.

    // *******************************************
    // Class B commands (0x10 to 0x1F)
    // *******************************************

    PingSlotInfoReq     = 0x10, // (1.0.1) Used by the end-device to communicate the unicast ping-slot periodicity to the Network Server
    PingSlotChannelAns  = 0x11, // (1.0.1) Used by the end-device to acknowledge a PingSlotChannelReq command
    BeaconTimingReq     = 0x12, // (1.0.1) Deprecated
    BeaconFreqAns       = 0x13, // (1.0.1) Deprecated.
    BeaconSettingsReq   = 0x14, // (1.2.0) Used by the end-device to request BeaconSettingsInd
    BeaconSettingsConf  = 0x15, // (1.2.0) Used by the end-device to acknowledge a BeaconSettingsInd command
    //            0x16 to 0x1F, // RFU

    // *******************************************
    // Class C commands (0x20 to 0x2F) 
    // *******************************************

    DeviceModeInd       = 0x20, // (1.1) Used by the end-device to indicate its current operating mode (Class A, or Class C-enabled)
    //            0x21 to 0x2F, //  RFU

    // *******************************************
    // Other commands (0x30 to 0xFF) 
    // *******************************************

    DevMobilityInd      = 0x30, // (1.2.0) Used by the end-device to indicate its current mobility state.
    NegotiationInd      = 0x31, // (1.2.0) Used by the end-device to negotiate LoRaWAN revision and cipher-suite
    //            0x32 to 0x3F, // RFU
    //            0x40 to 0x47, // Reserved for Relay command extensions
    //            0x48 to 0x7F, // 
    //            0x80 to 0xFF, // Reserved for proprietary network command extensions.

}
impl MACCmdUL {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
        //  0x00
            0x01 => Ok(Self::ResetInd), 
            0x02 => Ok(Self::LinkCheckReq),
            0x03 => Ok(Self::LinkADRAns),
            0x04 => Ok(Self::DutyCycleAns),
            0x05 => Ok(Self::RXParamSetupAns), 
            0x06 => Ok(Self::DevStatusAns),
            0x07 => Ok(Self::NewChannelAns),
            0x08 => Ok(Self::RXTimingSetupAns),
            0x09 => Ok(Self::TXParamSetupAns),
            0x0a => Ok(Self::DlChannelAns),
            0x0b => Ok(Self::RekeyInd),
            0x0c => Ok(Self::ADRParamSetupAns),
            0x0d => Ok(Self::DeviceTimeReq),
        //  0x0e
            0x0f => Ok(Self::RejoinParamSetupAns),
            0x10 => Ok(Self::PingSlotInfoReq),
            0x11 => Ok(Self::PingSlotChannelAns),
            0x12 => Ok(Self::BeaconTimingReq),
            0x13 => Ok(Self::BeaconFreqAns),
            0x14 => Ok(Self::BeaconSettingsReq),
            0x15 => Ok(Self::BeaconSettingsConf),
        //  0x16 to 0x1F
            0x20 => Ok(Self::DeviceModeInd),
        //  0x21 to 0x2F
            0x30 => Ok(Self::DevMobilityInd),
            0x31 => Ok(Self::NegotiationInd),
        //  0x32 to 0x3F
        //  0x40 to 0x47
        //  0x48 to 0x7F
        //  0x80 to 0xFF
            _ => Err(anyhow!("invalid MACCmdUL value: {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MACCmdDL {

    // *******************************************
    // Class A commands (0x20 to 0x2F) 
    // *******************************************

    //                    0x00, // RFU
    ResetConf           = 0x01, // (1.1)   Acknowledges ResetInd. 
    LinkCheckAns        = 0x02, // (1.0.0) Answers LinkCheckReq, indicating link margin to the end-device.
    LinkADRReq          = 0x03, // (1.0.0) Requests end-device to change data rate, TX power, redundancy, or channel mask.
    DutyCycleReq        = 0x04, // (1.0)   Sets the maximum aggregated transmit duty cycle of an end-device.
    RXParamSetupReq     = 0x05, // (1.0)   Sets the reception slot parameters.
    DevStatusReq        = 0x06, // (1.0)   Requests the status of the end-device.
    NewChannelReq       = 0x07, // (1.0)   Creates or modifies the definition of one or several radio channels.
    RXTimingSetupReq    = 0x08, // (1.0)   Sets the timing of the reception slots.
    TXParamSetupReq     = 0x09, // (1.0.2) Sets the maximum allowed dwell time and MaxEIRP of the end-device.
    DlChannelReq        = 0x0a, // (1.0.2) Creates or modifies a downlink RX1 radio channel.
    RekeyConf           = 0x0b, // (1.1)   Acknowledges RekeyInd.
    ADRParamSetupReq    = 0x0c, // (1.1)   Sets ADR_ACK_LIMIT and ADR_ACK_DELAY of an end-device 
    DeviceTimeAns       = 0x0d, // (1.0.3) Answers DeviceTimeReq.
    ForceRejoinReq      = 0x0e, // (1.1)   Asks the end-device to Rejoin immediately, with optional periodic retries.
    RejoinParamSetupReq = 0x0f, // (1.1)   Asks the end-device to set periodic transmission of Rejoin-Request frames.

    // *******************************************
    // Class B commands (0x10 to 0x1F)
    // *******************************************

    PingSlotInfoAns     = 0x10, // (1.0.1) Used by the Network to acknowledge a PingSlotInfoReq command
    PingSlotChannelReq  = 0x11, // (1.0.1) Used by the Network Server to set the unicast ping channel frequency and data rate of an end-device2
    BeaconTimingAns     = 0x12, // (1.0.1) Deprecated
    BeaconFreqReq       = 0x13, // (1.0.1) Deprecated.
    //                    0x14, //
    BeaconSettingsInd   = 0x15, // (1.2.0) Used by the Network Server to configure the Beacon parameters in the end-device. Autonomously sent, or in response to BeaconSettingsReq. 
    //            0x16 to 0x1F, // RFU

    // *******************************************
    // Class C commands (0x20 to 0x2F) 
    // *******************************************

    DeviceModeConf      = 0x20, // (1.1) Used by the network to acknowledge a DeviceModeInd command
    //            0x21 to 0x2F, //  RFU

    // *******************************************
    // Other commands (0x20 to 0x2F) 
    // *******************************************

    DevMobilityResp     = 0x30, // (1.2.0) Acknowledges DevMobilityInd
    NegotiationConf     = 0x31, // (1.2.0) Used by the network to negotiate LoRaWAN revision and cipher-suite
    //            0x32 to 0x3F, // RFU
    //            0x40 to 0x47, // Reserved for Relay command extensions
    //            0x48 to 0x7F, // 
    //            0x80 to 0xFF, // Reserved for proprietary network command extensions.

}
impl MACCmdDL {
    pub fn from_value(value: u8) -> AnyResult<Self> {
        match value {
            0x01 => Ok(Self::ResetConf), //     = 0x01
            0x02 => Ok(Self::LinkCheckAns), //  = 0x02
            0x03 => Ok(Self::LinkADRReq), //    = 0x03
            0x04 => Ok(Self::DutyCycleReq), //        = 0x04
            0x05 => Ok(Self::RXParamSetupReq), //     = 0x05
            0x06 => Ok(Self::DevStatusReq), //        = 0x06
            0x07 => Ok(Self::NewChannelReq), //       = 0x07
            0x08 => Ok(Self::RXTimingSetupReq), //    = 0x08
            0x09 => Ok(Self::TXParamSetupReq), //     = 0x09
            0x0a => Ok(Self::DlChannelReq), //        = 0x0a
            0x0b => Ok(Self::RekeyConf), //           = 0x0b
            0x0c => Ok(Self::ADRParamSetupReq), //    = 0x0c
            0x0d => Ok(Self::DeviceTimeAns), //       = 0x0d
            0x0e => Ok(Self::ForceRejoinReq), //      = 0x0e
            0x0f => Ok(Self::RejoinParamSetupReq), // = 0x0f
            0x10 => Ok(Self::PingSlotInfoAns), //     = 0x10
            0x11 => Ok(Self::PingSlotChannelReq), //  = 0x11
            0x12 => Ok(Self::BeaconTimingAns), //     = 0x12
            0x13 => Ok(Self::BeaconFreqReq), //       = 0x13
        //  0x14
            0x15 => Ok(Self::BeaconSettingsInd), //   = 0x15
        //  0x16 to 0x1F
            0x20 => Ok(Self::DeviceModeConf), //      = 0x20
        //  0x21 to 0x2F
            0x30 => Ok(Self::DevMobilityResp), //     = 0x30
            0x31 => Ok(Self::NegotiationConf), //     = 0x31
        //  0x32 to 0x3F
        //  0x40 to 0x47
        //  0x48 to 0x7F 
        //  0x80 to 0xFF
            _ => Err(anyhow!("invalid MACCmdDL value: {}", value)),
        }
    }
}
