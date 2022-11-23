use aes::Aes128;
use aes::cipher::{
	typenum::U16,
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

use crate::lorawan_pdu::{Buf, Dir, MType};
use crate::lorawan_cmac::aes128_cmac;





/*
use aes::cipher::InvalidLength;
use cmac::{Cmac, Mac};

pub fn aes128_cmac(key: &[u8; 16], msg: &Buf) -> Result<[u8; 4], InvalidLength> {

    let mut mac = Cmac::<Aes128>::new_from_slice(key)?;
    mac.update(msg.to_slice());
    let mac_bytes = mac.finalize().into_bytes();

    Ok([mac_bytes[0],mac_bytes[1],mac_bytes[2],mac_bytes[3]] )

}
*/





// ***********************
// Utils

pub fn get_dir(m_type: MType) -> Result<Dir, &'static str> {
	match m_type {
        MType::ConfirmedDataUp | MType::UnconfirmedDataUp => {
            return Ok(Dir::Uplink);
        },
        MType::ConfirmedDataDown | MType::UnconfirmedDataDown => {
            return Ok(Dir::Uplink);
        },
        _ => {
            return Err("GetDir() error: Incorrect MType")
        }
    }
}

// https://docs.rs/aes/latest/aes/
pub fn aes128_decrypt<'a>(key: &'a [u8; 16], input_block: &'a [u8; 16]) -> Result<GenericArray<u8, U16>, &'static str> {
    let cipher = Aes128::new(&GenericArray::from(*key));
    let mut output_block = GenericArray::from(*input_block);
	cipher.decrypt_block(&mut output_block);
	Ok(output_block)
}

// https://docs.rs/aes/latest/aes/
pub fn aes128_encrypt<'a>(key: &'a [u8; 16], input_block: &'a [u8; 16]) -> Result<GenericArray<u8, U16>, &'static str> {
    let cipher = Aes128::new(&GenericArray::from(*key));
    let mut output_block = GenericArray::from(*input_block);
	cipher.encrypt_block(&mut output_block);
	Ok(output_block)
}



pub fn phy_data_crypt<'a>(key: &[u8; 16], x: &'a Buf, phy_payload: &'a Buf, f_cnt32_last_known: u32) -> Result<Buf, &'static str> {

    //TODO!: use GetMType function
    let m_type = MType::from_u8((phy_payload.buf[0] & 0b11100000) >> 5);
	let dir = get_dir(m_type).unwrap();

	let mut _a_: [u8; 16] = [
		0x01,
		0x00, 0x00, 0x00, 0x00,
		dir as u8,
		phy_payload.buf[1], phy_payload.buf[2], phy_payload.buf[3], phy_payload.buf[4], // DevAddr
		phy_payload.buf[6], phy_payload.buf[7], // FCnt
		((f_cnt32_last_known >> 16) & 0xff) as u8, ((f_cnt32_last_known >> 24) & 0xff) as u8,
		0x00, 0x00
    ];
        
	let l = x.len;


	let mut y = Buf::new();
	y.len = l & 0xff;


    let mut num_of_blocks = l >> 4;  // l modulo 16
	if (l & 0xf) > 0 {                      // l reminder 16
		num_of_blocks += 1;
	}

    for k in 0..num_of_blocks {
		_a_[15] = (k + 1) as u8;

		
		let _s_ = aes128_encrypt(key, &_a_).unwrap();


        for i in 0..16 {
            if 16*k+i >= l { break; };
            y.buf[16*k+i] = x.buf[16*k+i] ^ _s_[i]
        }
	}

	return Ok(y)

}

pub fn phy_data_calculate_mic<'a>(key: &'a [u8; 16], phy_data_without_mic: &'a Buf, f_cnt32_last_known: u32) -> Result<[u8; 4], &'static str> {

	let l = phy_data_without_mic.len;

    //TODO!: use GetMType function
    let m_type = MType::from_u8((phy_data_without_mic.buf[0] & 0b11100000) >> 5);
	let dir = get_dir(m_type).unwrap();

	let _b_: [u8; 16] = [
		0x49,
		0x00, 0x00, // byte(confFCnt & 0xff), byte((confFCnt >> 8) & 0xff), // for LoRaWAN 1.1
		0x00, 0x00,
		dir as u8,
		phy_data_without_mic.buf[1], phy_data_without_mic.buf[2], phy_data_without_mic.buf[3], phy_data_without_mic.buf[4], // DevAddr
		phy_data_without_mic.buf[6], phy_data_without_mic.buf[7], // FCnt
		((f_cnt32_last_known >> 16) & 0xff) as u8, ((f_cnt32_last_known >> 24) & 0xff) as u8,
		0x00,
		(l & 0xff) as u8
    ];


	// concatenate _b_ and phy_data_without_mic.to_slice()
	let mut msg = Buf::new_from_slice(&_b_);
	msg.buf[msg.len..msg.len+phy_data_without_mic.len].copy_from_slice(phy_data_without_mic.to_slice());
	msg.len = msg.len + phy_data_without_mic.len;
	// let msg = &[&_b_, phy_data_without_mic.to_slice()].concat();

	let mic = aes128_cmac(key, &msg).unwrap();

	return Ok(mic);

}