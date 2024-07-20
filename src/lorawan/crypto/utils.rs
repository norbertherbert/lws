use aes::{Aes128, Block};
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
	// InvalidLength
};
use cmac::{Cmac, Mac};
use anyhow::Result as AnyResult;

use crate::lorawan::MType;

// ***********************
// *** Crypto Utils
// ***********************

pub fn aes128_cmac(key: &[u8; 16], buffer: &[u8]) -> [u8; 4] {
	let mut mac = <Cmac::<Aes128> as Mac>::new_from_slice(key).unwrap();
    mac.update(buffer);
    let mac_bytes = mac.finalize().into_bytes();

	mac_bytes[..4].try_into().unwrap() 
}

// https://docs.rs/aes/latest/aes/
pub fn aes128_encrypt<'a>(key: &'a [u8; 16], block: &'a [u8; 16]) -> [u8; 16] {
	let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut encrypted_block: Block = GenericArray::clone_from_slice(block);
	cipher.encrypt_block(&mut encrypted_block);
	encrypted_block.into()
}

pub fn aes128_decrypt<'a>(key: &'a [u8; 16], block: &'a [u8; 16]) -> [u8; 16] {
	let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut decrypted_block: Block = GenericArray::clone_from_slice(block);
	cipher.encrypt_block(&mut decrypted_block);
	decrypted_block.into()
}

pub fn aes128_encrypt_in_place<'a>(
	key: &[u8; 16],
	msg: &'a mut [u8], 
) {
	let cipher = Aes128::new(GenericArray::from_slice(key));
	cipher.encrypt_block( GenericArray::from_mut_slice(&mut msg[1..17]) );
	if msg.len() == 33 {
		cipher.encrypt_block( GenericArray::from_mut_slice(&mut msg[17..33]) );
	}
}

pub fn aes128_decrypt_in_place<'a>(
	key: &[u8; 16],
	msg: &'a mut [u8], 
) {
	let cipher = Aes128::new(GenericArray::from_slice(key));
	cipher.decrypt_block( GenericArray::from_mut_slice(&mut msg[1..17]) );
	if msg.len() == 33 {
		cipher.decrypt_block( GenericArray::from_mut_slice(&mut msg[17..33]) );
	}
}


pub fn get_m_type(phy_payload: &[u8]) -> AnyResult<MType> {
	MType::from_value((phy_payload[0] & 0b11100000) >> 5)
}

pub fn key_from_string(str: &str) -> AnyResult<[u8; 16]> {
	let mut key: [u8; 16] = [0; 16];
	hex::decode_to_slice(str, &mut key)?;
	Ok(key)
}
