mod settings;
pub use settings::{Settings, get_settings_once};

mod logger;
pub use logger::init_logger_once;

pub mod lorawan_pdu;
pub use lorawan_pdu::{Buf, PktfMType, MType, RXPacket, Stat, PushData, PHYDataComps};

pub mod lorawan_crypto;
pub use lorawan_crypto::{aes128_encrypt, aes128_decrypt, phy_data_crypt, phy_data_calculate_mic};

pub mod lorawan_cmac;
pub use lorawan_cmac::aes128_cmac;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
