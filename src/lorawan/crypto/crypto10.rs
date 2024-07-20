use anyhow::Result as AnyResult;

use crate::lorawan::Dir;

use super::utils::{
	aes128_encrypt, aes128_cmac, 
	aes128_encrypt_in_place, aes128_decrypt_in_place,
};


/// Encrypts/Decrypts the FRMPayload of a LoRaWAN Data Frame
///
/// # Arguments
/// 
/// * __`frm_payload`__\
///   Frame Payload (`FRMPayload`)
/// * __`key`__\
///   `NwkSKey` if FPort=0; `AppSKey` if `FPort`>0
/// * __`dir`__\
///   The direction of the message (UL/DL)
/// * __`dev_addr`__\
///   Device Address (`DevAddr`)
/// * __`f_cnt32`__\
///   `FCntUp` for uplink frame; `FCntDown` for downlink frame
/// 
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - Line #767                     \
/// 4.3.3 MAC frame payload encryption (FRMPayload)  \
/// 
pub fn frm_payload_crypt<'a>(
	frm_payload: &'a mut [u8], 
	key: &[u8; 16],
	dir: Dir,
	dev_addr: u32,
	f_cnt32: u32,
) -> AnyResult<()> {
	
	let frm_payload_len = frm_payload.len();
	if frm_payload_len == 0 { return Ok(()) } 

	// Ceating an Ai block where i=0
	let mut block: [u8; 16] = [0; 16];
	block[0] = 0x01;
	block[5] = dir as u8;
	block[6..10].copy_from_slice(&dev_addr.to_le_bytes());
	block[10..14].copy_from_slice(&f_cnt32.to_le_bytes());

	let mut num_of_blocks = frm_payload_len >> 4;  // l modulus 16
	if (frm_payload_len & 0xf) > 0 {                      // l reminder 16
		num_of_blocks += 1;
	}

	for k in 0..num_of_blocks {
		// Updating i in Ai
		block[15] = (k + 1) as u8;
		let s_block: [u8; 16] = aes128_encrypt(
			key,
			&block
		);
		let block_start_index = k << 4; // 16*k
		for i in 0..16 {
			if block_start_index+i >= frm_payload_len { break; };
			frm_payload[block_start_index+i] = frm_payload[block_start_index+i] ^ s_block[i]
		}
	}

	Ok(())

}


/// Calculates the MIC of a LoRaWAN Data Frame
/// 
/// # Arguments
/// 
/// * __`phy_payload`__\
/// PHY Payload
/// * __`nwk_s_key`__\
///   Network Session Key (`NwkSKey`)
/// * __`dev_addr`__\
///   Device Address (`DevAddr`)
/// * __`f_cnt32`__\
///   `FCntUp` or `FCntDown` 
/// 
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - Line #795      \
/// 4.4 Message Integrity Code (MIC)  \
/// 
pub fn data_frame_calculate_mic<'a>(
	phy_payload: &'a [u8], 
	nwk_s_key: &'a [u8; 16], 
	dev_addr: u32,
	f_cnt32: u32,
) -> [u8; 4] {

	let len = phy_payload.len()-4; // length without MIC

	// Creating Asc block
	// 0x49|0x00|0x00|0x00|0x00|Dir|dev_addr|f_cnt|0x00|len
	let mut block: [u8; 16] = [0; 16];
	block[0] = 0x49;
	block[5] = Dir::Downlink as u8;
	block[6..10].copy_from_slice(&dev_addr.to_le_bytes());
	block[10..14].copy_from_slice(&f_cnt32.to_le_bytes());
	block[15] = (len & 0xff) as u8;

	aes128_cmac(
		nwk_s_key, 
		[&block, &phy_payload[..len]].concat().as_ref()
	)

}


/// Calculates the MIC of a LoRaWAN JoinRequest/JoinAccept Frame
///
/// # Arguments
/// 
/// * __`app_key`__\
///   `AppKey`
/// * __`clear_text_phy_payload`__\
///   `PHYPayload   (= MHDR|JoinRequest/JoinAccept|MIC`)
///
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - Line #1427 \
/// 6.2.5 Join-Request frame      \
/// 
/// `CMAC = aes128_cmac(AppKey, MHDR|JoinEUI|DevEUI|DevNonce)` 
/// 
/// LoRaWAN L2 1.0.4 - Line #1457 \
/// 6.2.6 Join-Accept frame       \
/// 
/// `CMAC = aes128_cmac(AppKey, MHDR|JoinNonce|NetID|DevAddr|DLSettings|RXDelay|CFList)`
/// 
pub fn join_frame_calculate_mic(
	app_key: &[u8; 16],
	clear_text_phy_payload: &[u8], 
) -> [u8; 4] {
	let len = clear_text_phy_payload.len()-4; // length without MIC
	aes128_cmac(
		app_key, &clear_text_phy_payload[..len]
	)
}


/// Encrypts a LoRaWAN JoinAccept Frame 
/// 
/// # Arguments
/// 
/// * __`app_key`__\
/// `AppKey` 
/// * __`clear_text_ja_payload`__\
/// The unencrypted (clear text) version of the Join Accept Payload: \
/// `JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList|MIC`
///  
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - Line #1457 \
/// 6.2.6 Join-Accept frame       \
///
/// `JAPayload = aes128_decrypt(JoinAcceptEncKey, JoinNonce|NetID|DevAddr|DLSettings|RxDelay|CFList|MIC)`
///
pub fn join_accept_encrypt<'a>(
	app_key: &[u8; 16],
	clear_text_ja_payload: &'a mut [u8],
) {
	aes128_decrypt_in_place(app_key, &mut clear_text_ja_payload[1..17]);
	if clear_text_ja_payload.len() == 33 {
		aes128_decrypt_in_place(app_key, &mut clear_text_ja_payload[17..33]);
	}
}


/// Decrypts a LoRaWAN JoinAccept Frame 
///
/// # Arguments
/// 
/// * __`app_key`__\
///   `AppKey` 
/// * __`ja_payload`__\
///   Encrypted (`JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList|MIC`)
///
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - Line #1457 \
/// 6.2.6 Join-Accept frame       \
/// 
pub fn join_accept_decrypt<'a>(
	app_key: &[u8; 16],
	ja_payload: &'a mut [u8], 
) {
	aes128_encrypt_in_place(app_key, &mut ja_payload[1..17]);
	if ja_payload.len() == 33 {
		aes128_encrypt_in_place(app_key, &mut ja_payload[17..33]);
	}
}


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    NwkSKey         = 0x01,
    AppSKey         = 0x02,
}


/// Derives a LoRaWAN Sessioom Key (AppSKey or NwkSKey)
/// 
/// # Arguments
/// 
/// * __`app_key`__\
///   `AppKey`
/// * __`target_key_type`__\
///   `AppSKey` | `FNwkSIntKey`
/// * __`join_nonce`__\
///   `JoinNonce`
/// * __`net_id`__\
///   `NetID`
/// * __`dev_nonce`__\
///   `DevNonce`
/// 
/// # Specification
/// 
/// LoRaWAN L2 1.0.4 - line #1457 \
/// 6.2.6 Join-Accept frame
///
pub fn derive_s_key(
	app_key: &[u8; 16], // 16 bytes
	target_key_type: KeyType, 
	join_nonce: u32,    // 3 bytes
	net_id: u32,        // 3 bytes
	dev_nonce: u16      // 2 bytes
) -> [u8; 16] {
	let mut block: [u8; 16] = [0; 16];
	block[0] = target_key_type as u8;
	block[1..4].copy_from_slice(&join_nonce.to_le_bytes()[..3]);
	block[4..7].copy_from_slice(&net_id.to_le_bytes()[..3]);
	block[7..9].copy_from_slice(&dev_nonce.to_le_bytes());
	aes128_encrypt( app_key, &block )
}



pub struct SKeys {
	pub app_s_key: [u8; 16],
	pub nwk_s_key: [u8; 16],
}


pub fn derive_s_keys(
	app_key: &[u8; 16], // 16 bytes
	join_nonce: u32,    // 3 bytes
	net_id: u32,        // 3 bytes
	dev_nonce: u16      // 2 bytes
) -> SKeys {

	let mut block: [u8; 16] = [0; 16];

	block[1..4].copy_from_slice(&join_nonce.to_le_bytes()[..3]);
	block[4..7].copy_from_slice(&net_id.to_le_bytes()[..3]);
	block[7..9].copy_from_slice(&dev_nonce.to_le_bytes());
	
	block[0] = KeyType::NwkSKey as u8;
	let nwk_s_key = aes128_encrypt( app_key, &block );

	block[0] = KeyType::AppSKey as u8;
	let app_s_key = aes128_encrypt( app_key, &block );

	SKeys{nwk_s_key, app_s_key}

}

