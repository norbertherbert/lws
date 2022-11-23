use aes::{Aes128, cipher::InvalidLength};
use cmac::{Cmac, Mac};

use crate::Buf;

pub fn aes128_cmac(key: &[u8; 16], msg: &Buf) -> Result<[u8; 4], InvalidLength> {

    let mut mac = Cmac::<Aes128>::new_from_slice(key)?;
    mac.update(msg.to_slice());
    let mac_bytes = mac.finalize().into_bytes();

    Ok([mac_bytes[0],mac_bytes[1],mac_bytes[2],mac_bytes[3]] )

}
