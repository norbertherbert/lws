use anyhow::Result as AnyResult;

use crate::lorawan::Dir;

use super::utils::{
	aes128_encrypt, aes128_cmac, 
	aes128_encrypt_in_place, aes128_decrypt_in_place,
};


pub enum FCntType {
	FCntUp,
	NFCntDown,
	AFCntDown,
}

#[repr(u8)]
pub enum IVHeader {
    Undefined = 0x00, // Added for LoRaWAN 1.0.x compatibility
    RX1       = 0x4a,
    RX2       = 0x4b,
    RXJ1      = 0x50,
    RXJ2      = 0x51,
}

#[repr(u8)]
pub enum JoinReqType {
    RejoinReqType0 = 0x00,
    RejoinReqType1 = 0x01,
    RejoinReqType2 = 0x02,
	JoinReq = 0xff,
}

#[repr(u8)]
pub enum KeyType {
    FNwkSIntKey         = 0x01,
    AppSKey             = 0x02,
    SNwkSIntKey         = 0x03,
    NwkSEncKey          = 0x04,
    JSEncKey            = 0x05,
    JSIntKey            = 0x06,
}

#[repr(u8)]
pub enum JSKeyType {
    JSEncKey            = 0x05,
    JSIntKey            = 0x06,
}

#[repr(u8)]
pub enum SKeyType {
    FNwkSIntKey         = 0x01,
    AppSKey             = 0x02,
    SNwkSIntKey         = 0x03,
    NwkSEncKey          = 0x04,
}

pub struct JSKeys {
	pub js_enc_key: [u8; 16],
	pub js_int_key: [u8; 16],
}

pub struct SKeys {
	pub app_s_key: [u8; 16],
	pub f_nwk_s_int_key: [u8; 16],
	pub s_nwk_s_int_key: [u8; 16],
	pub nwk_s_enc_key: [u8; 16],
}


/// Encrypts/Decrypts the `FRMPayload` field of a LoRaWAN Data Frame
/// 
/// # Arguments
/// 
/// * __`frm_payload`__\
///   Frame Payload (`FRMPayload`) \
/// * __`key`__\
///   If `FPort` = 0: `NwkSEncKey`; If `FPort` > 0: `AppSKey` \
/// * __`dev_addr`__\
///   Device Address (`DevAddr`) \
/// * __`f_cnt32`__\
///   `FCntUp` for uplink frame; \
///   `NFCntDown` for downlink frame with `FPort` = 0 or absent; \
///   `AFCntDown` for downlink frame with `FPort` > 0 \
/// 
/// # Specification
/// 
/// LoRaWAN 1.2.0 Draft 47 - line #1015        \
/// 4.3.3.1 Default PAYLOAD_ENCRYPTION_SCHEME  \
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


/// Encrypts/Decrypts the `FOpts` field in an UL/DL Data Frame
///
/// # Specification
/// 
/// LoRaWAN 1.2.0 Draft 47 - line #979          \
/// 4.3.1.7.1 Default FOPTS_ENCRYPTION_SCHEME   \
/// 
/// # Arguments
/// 
/// * __`f_opts`__\
///   Frame Options (`FOpts`)
/// * __`key`__\
///   `NwkSEncKey` \
/// * __`dev_addr`__\
///   Device Address (`DevAddr`) \
/// * __`f_cnt32`__\
///   `FCntUp` for uplink frame; \
///   `NFCntDown` for downlink frame with `FPort` = 0 or absent; \
///   `AFCntDown` for downlink frame with `FPort` > 0 \
/// * __`f_cnt_type`__\
///   `FCntUp` | `NFCntDown` | `AFCntDown` \
/// 

pub fn f_opts_crypt<'a>(
	f_opts: &'a mut [u8], 
	nwk_s_enc_key: &[u8; 16],
	dev_addr: u32,
	f_cnt32: u32,
	f_cnt_type: FCntType,
) -> AnyResult<()> {
	
	let f_opts_len = f_opts.len();
	if f_opts_len == 0 { return Ok(()) } 

	let dir: u8;                   // 0: if UL, 1: if DL
	let frame_context: u8;         // 1: if it is not AFCntDown, 2: if it is AFCntDown:
	match f_cnt_type {
		FCntType::FCntUp => { 
			dir = 0; 
			frame_context = 1;
		},
		FCntType::NFCntDown => { 
			dir = 1; 
			frame_context = 1;
		},
		FCntType::AFCntDown => { 
			dir = 1; 
			frame_context = 2;
		},
	};

	let mut block: [u8; 16] = [0; 16];
	block[0] = 0x01;
	block[1] = frame_context;
	block[5] = dir;
	block[6..10].copy_from_slice(&dev_addr.to_le_bytes());
	block[10..14].copy_from_slice(&f_cnt32.to_le_bytes());
	block[15] = 0x01;

	let s_block: [u8; 16] = aes128_encrypt(
		nwk_s_enc_key,
		&block
	);
	for i in 0..f_opts_len {
		f_opts[i] = f_opts[i] ^ s_block[i];
	}

	Ok(())

}


/// Calculates MIC for Class A downlink frames that contain the RekeyConf MAC command \
/// i.e., during session context confirmation \
///
/// # Argumants
/// 
/// * __`phy_payload`__\
///   `PHYPayload` \
/// * __`s_nwk_s_int_key`__\ 
///   Serving NS Network Session Integrity Key (`SNwkSIntKey`) \
/// * __`conf_f_cnt`__\
///   SHALL be the `FCnt` field of last uplink frame. (`ConfFCnt`) \
/// * __`dev_addr`__\
///   Device Address (DevAddr) \
/// * __`f_cnt32`__\
///   `NFCntDown` when FPort is absent or equals 0 or `AFCntDown` (`FPort` > 0) \
///
/// # Specification
/// 
/// LoRaWAN 1.2.0 Draaft 47 - line #1065
/// 4.4.1 Default DOWNLINK_MIC_SC_SCHEME
///
pub fn data_frame_dl_calculate_mic_sc<'a>(
	phy_payload: &'a [u8], 
	s_nwk_s_int_key: &'a [u8; 16], 
	conf_f_cnt: u16, 
	dev_addr: u32,
	f_cnt32: u32,
) -> [u8; 4] {

	let len = phy_payload.len()-4; // length without MIC

	// Creating Asc block
	// 0x49|conf_f_cnt|0x00|0x00|Dir|dev_addr|f_cnt|0x00|len
	let mut block: [u8; 16] = [0; 16];
	block[0] = 0x49;
	block[1..3].copy_from_slice(&conf_f_cnt.to_le_bytes());
	block[5] = Dir::Downlink as u8;
	block[6..10].copy_from_slice(&dev_addr.to_le_bytes());
	block[10..14].copy_from_slice(&f_cnt32.to_le_bytes());
	block[15] = (len & 0xff) as u8;

	aes128_cmac(
		s_nwk_s_int_key, 
		[&block, &phy_payload[..len]].concat().as_ref()
	)

}


// ***************************************************
// *** Data Frame: MIC for DL without RekeyConf
// ***************************************************

///
/// 4.4.2 Default DOWNLINK_MIC_A_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #1091
///
/// 
/// For a downlink frame which does not carry the RekeyConf MAC command!
/// 
/// data_frame_dl_calculate_mic_a
/// - phy_payload     PHY Payload
/// - s_nwk_s_int_key Serving NS Network Session Integrity Key (SNwkSIntKey)
/// - iv_header       depends on the RX window: RX1->0x4a, RX2->0x4b, RXJ1->0x50, RXJ2->0x51,
/// - tx_dr_up        It is the data rate used for the transmission of the uplink packet that initiated the Class A downlink. 
/// - tx_ch_up        It is the index of the channel used for the transmission of the uplink packet that initiated the Class A downlink. 
///                   It follows the same convention as ChIndex defined in section 5.7, with a maximum value of 127.
/// - cnt             It is the counter of the frame that initiated the Class A downlink: RX1/RX2->FCnt, RXJ1/RXJ2->RJCount02  
/// - dev_addr        Device Address (DevAddr)
/// - f_cnt32         NFCntDown when FPort is absent or equals 0; AFCntDown when FPort > 0
/// 
pub fn data_frame_dl_calculate_mic_a<'a>(
	phy_payload: &'a [u8], 
	s_nwk_s_int_key: &'a [u8; 16], 
	iv_header: IVHeader, 
	tx_dr_up: u8,
	tx_ch_up: u8,
	cnt: u16,
	dev_addr: u32,
	f_cnt32: u32,
) -> [u8; 4] {

	let len = phy_payload.len()-4; // length without MIC

	// Creating Aa block
	// iv_header|0x00|0x00|tx_dr_up|tx_ch_up|cnt|dev_addr|f_cnt|len
	let mut block: [u8; 16] = [0; 16];
	block[0] = iv_header as u8;
	block[3] = tx_dr_up;
	block[4] = tx_ch_up;
	block[5..7].copy_from_slice(&cnt.to_le_bytes());
	block[7..11].copy_from_slice(&dev_addr.to_le_bytes());
	block[11..15].copy_from_slice(&f_cnt32.to_le_bytes());
	block[15] = (len & 0xff) as u8;

	aes128_cmac(
		s_nwk_s_int_key, 
		[&block, &phy_payload[..len]].concat().as_ref()
	)

}


// ***************************************************
// *** Data Frame: MIC for UL
// ***************************************************

///
/// 4.4.3 Default UPLINK_MIC_SCHEME
///     LoRaWAN 1.2.0 Draft 47 - line #1130
///  
/// 
/// For upnlink frames
/// 
/// data_frame_ul_calculate_mic
/// - phy_payload       PHY Payload
/// - s_nwk_s_int_key   Serving NS Network Session Integrity Key (SNwkSIntKey)
/// - f_nwk_s_int_key   Forwarding NS Network Session Integrity Key (FNwkSIntKey)
/// - conf_f_cnt        If the ACK bit of the uplink frame is set, meaning this frame is acknowledging a Confirmed-Data-Downlink frame, 
///                     then ConfFCnt SHALL be the FCnt field of the Confirmed-Data-Downlink frame. 
///                     If the ACK bit of the uplink frame is not set, ConfFCnt SHALL value 0x0000.
/// - tx_dr_up          It is the data rate used for the transmission of the uplink frame. 
///                     It follows the same convention as the LinkADRReq command and SHALL be defined for each physical layer and regional channel plan.
/// - tx_ch_up          It is the index of the channel used for the transmission. 
///                     It follows the same convention as ChIndex defined in section 5.7, with a maximum value of 127.
/// - dev_addr          Device Address (DevAddr)
/// - f_cnt32           Frame Counter (FCntUp)
/// 
pub fn data_frame_ul_calculate_mic<'a>(
	phy_payload: &'a [u8], 
	s_nwk_s_int_key: &'a [u8; 16], 
	f_nwk_s_int_key: &'a [u8; 16], 
	conf_f_cnt: u16,
	tx_dr_up: u8,
	tx_ch_up: u8,
	dev_addr: u32,
	f_cnt32: u32,
) -> [u8; 4] {

	let len = phy_payload.len()-4; // length without MIC

	// Creating B0 block
	// 0x49|0x00|0x00|0x00|0x00|Dir|dev_addr|f_cnt|0x00|len
	let mut block: [u8; 16] = [0; 16];
	block[0] = 0x49;
	block[5] = Dir::Uplink as u8;
	block[6..10].copy_from_slice(&dev_addr.to_le_bytes());
	block[10..14].copy_from_slice(&f_cnt32.to_le_bytes());
	block[15] = (len & 0xff) as u8;

	let cmacf = aes128_cmac(
		f_nwk_s_int_key, 
		[&block, &phy_payload[..len]].concat().as_ref()
	);

	// Turning B0 block to B1 block
	// 0x49|conf_f_cnt|tx_dr_up|tx_ch_up|Dir|dev_addr|f_cnt|0x00|len
	block[1..3].copy_from_slice(&conf_f_cnt.to_le_bytes());
	block[3] = tx_dr_up;
	block[4] = tx_ch_up;

	let cmacs = aes128_cmac(
		s_nwk_s_int_key, 
		[&block, &phy_payload[..len]].concat().as_ref()
	);

	[cmacs[0], cmacs[1], cmacf[0], cmacf[1]]

}



// ***************************************************
// *** Join Request MIC
// ***************************************************

///
/// 6.2.2.1 Default JOIN_REQUEST_MIC_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2442
///     CMAC = aes128_cmac(
///       NwkKey, 
///       MHDR|JoinEUI|DevEUI|DevNonce
///     )
///
/// 6.2.4.1.1 Default REJOIN_REQUEST_02_MIC_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2822
///     CMAC = aes128_cmac(
///       SNwkSIntKey, 
///       MHDR|RejoinType|NetID|DevEUI|RJcount02
///     )
/// 
/// 6.2.4.2.1 Default REJOIN_REQUEST_1_MIC_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2876
///     CMAC = aes128_cmac(
///       JSIntKey, 
///       MHDR|RejoinType|JoinEUI|DevEUI|RJcount1
///     )
/// 
/// 6.2.5 Join-Request frame
///     LoRaWAN L2 1.0.4 - Line #1427
///     CMAC = aes128_cmac(
///       AppKey, 
///       MHDR|JoinEUI|DevEUI|DevNonce
///     ) 
/// 
/// 
/// join_request_calculate_mic
/// - key           Depends on the Join Frame Type:
///                   Join Request          -> NwkKey
///                   Rejoin Request 0 or 2 -> SNwkSIntKey
///                   Rejoin Request 1      -> JSIntKey
/// - phy_payload   PHYPayload (MHDR|Request|MIC)
/// 

pub fn join_request_calculate_mic(
	key: &[u8; 16],
	phy_payload: &[u8], 
) -> [u8; 4] {
	aes128_cmac(
		key, &phy_payload[..phy_payload.len()-4]
	)
}



// ***************************************************
// *** Join Accept MIC
// ***************************************************

///
/// 6.2.3.2 Default JOIN_ACCEPT_MIC_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2687
///     CMAC = aes128_cmac(
///       JSIntKey, 
///       JoinReqType|JoinEUI|DevJCount      |MHDR|JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList 
///     )
/// 
/// 
/// join_accept_calculate_mic
/// - js_int_key                JSIntKey
/// - join_req_type             JoinReqType
/// - join_eui                  JoinEUI
/// - dev_j_count               DevJCount:
///                               Join-Request          -> DevNonce
///                               Rejoin-Request 0 or 2 -> RJCount02
///                               Rejoin-Request 1      -> RJCount02
/// - clear_text_phy_payload    MHDR|JoinAccept|MIC)
///

pub fn join_accept_calculate_mic(
	js_int_key: &[u8; 16],
	join_req_type: JoinReqType,
	join_eui: u64,
	dev_j_count: u16,
	clear_text_phy_payload: &[u8], 
) -> [u8; 4] {

    // let mut prefix: Vec<u8> = vec![ join_req_type as u8 ];
	// prefix.append(&mut join_eui.to_le_bytes().to_vec());
	// prefix.append(&mut dev_j_count.to_le_bytes().to_vec());
	// prefix.append(&mut phy_payload[..phy_payload.len()-4].to_vec());

	// aes128_cmac(
	// 	js_int_key, &prefix
	// )

	let len = clear_text_phy_payload.len()-4; // length without MIC

	let mut prefix: [u8; 11] = [0; 11];
	prefix[0] = join_req_type as u8;
	prefix[1..9].copy_from_slice(&join_eui.to_le_bytes()[..8]);
	prefix[9..11].copy_from_slice(&dev_j_count.to_le_bytes()[..2]);

	aes128_cmac(
		js_int_key, 
		[&prefix, &clear_text_phy_payload[..len]].concat().as_ref()
	)

}



// ***************************************************
// *** Join Accept Encrypt
// ***************************************************


///
/// 6.2.3.3 Default JOIN_ACCEPT_ENCRYPTION_SCHEME
/// LoRaWAN 1.2.0 Draaft 45 - line #2704
///
/// If CFList is absent:
///   JAPayload = aes128_ecb_decrypt(JoinAcceptEncKey, JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|MIC)
/// If CFList is present: 2716
///   JAPayload = aes128_ecb_decrypt(JoinAcceptEncKey, JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList|MIC)
/// 
/// 
/// join_accept_encrypt
/// - join_accept_enc_key      NwkKey if triggered by Join-Request; 
///                            JSEncKey if triggered by Rejoin-Request type 0 or 1 or 2
///                            **** LORAWAN 1.0.x **** 
///                              AppKey
/// - clear_text_ja_payload    JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList|MIC
/// 

pub fn join_accept_encrypt<'a>(
	join_accept_enc_key: &[u8; 16],
	clear_text_ja_payload: &'a mut [u8],
) {
	aes128_decrypt_in_place(join_accept_enc_key, &mut clear_text_ja_payload[1..17]);
	if clear_text_ja_payload.len() == 33 {
		aes128_decrypt_in_place(join_accept_enc_key, &mut clear_text_ja_payload[17..33]);
	}
}




// ***************************************************
// *** Join Accept Decrypt
// ***************************************************

///
/// join_accept_decrypt
/// - join_accept_enc_key      NwkKey if triggered by Join-Request; 
///                            JSEncKey if triggered by Rejoin-Request type 0 or 1 or 2
///                            **** LORAWAN 1.0.x **** 
///                              AppKey
/// - ja_payload               Encrypted (JoinNonce|NetID|DevAddr|JADLSettings|RxDelay|CFList|MIC)
/// 
pub fn join_accept_decrypt<'a>(
	join_accept_enc_key: &[u8; 16],
	ja_payload: &'a mut [u8], 
) {
	aes128_encrypt_in_place(join_accept_enc_key, &mut ja_payload[1..17]);
	if ja_payload.len() == 33 {
		aes128_encrypt_in_place(join_accept_enc_key, &mut ja_payload[17..33]);
	}
}



// ***********************
// *** JS Key Derivation
// ***********************

///
/// 6.1.1.4.1 Default JOIN_SERVER_LIFETIME_KEYS_DERIVATION_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2199
/// 
/// derive_js_key
/// - nwk_key           Network Key (NwkKey)
/// - target_key_type   JSEncKey | JSIntKey
/// - dev_eui           Device EUI (DevEUI)
/// 
pub fn derive_js_key(
	nwk_key: &[u8; 16], 
	target_key_type: JSKeyType, 
	dev_eui: u64,
) -> [u8; 16] {
	let mut block: [u8; 16] = [0; 16];
	block[0] = target_key_type as u8;
	block[1..9].copy_from_slice(&dev_eui.to_le_bytes()[..8]);
	aes128_encrypt( nwk_key, &block )
}


pub fn derive_js_keys(
	nwk_key: &[u8; 16], 
	dev_eui: u64,
) -> JSKeys {
	let mut block: [u8; 16] = [0; 16];
	block[1..9].copy_from_slice(&dev_eui.to_le_bytes()[..8]);

	block[0] = JSKeyType::JSEncKey as u8;
	let js_enc_key = aes128_encrypt( nwk_key, &block );

	block[0] = JSKeyType::JSIntKey as u8;
	let js_int_key = aes128_encrypt( nwk_key, &block );

	JSKeys { js_enc_key, js_int_key }
}




// ****************************
// *** Session Key Derivation
// ****************************

///
/// 6.2.3.1 Default SESSION_KEYS_DERIVATION_SCHEME
///     LoRaWAN 1.2.0 Draaft 47 - line #2663
///
/// derive_s_key
/// - master_key       AppKey if target_key_type=AppSKey, otherwise NwkKey
/// - target_key_type  AppSKey | FNwkSIntKey | SNwkSIntKey | NwkSEncKey
/// - join_nonce       JoinNonce
/// - join_eui         JoinEUI
/// - dev_j_count      Depends on the frame type of the request that triggers the Join-Accept frame.
///                      Join-Request ->               DevNonce
///                      Rejoin-Request type 0 or 2 -> RJcount02
///                      Rejoin-Request type 1 ->      RJcount1
/// 
pub fn derive_s_key(
	master_key: &[u8; 16], // = app_key for KeyType::AppSKey, otherwise nwk_key
	target_key_type: SKeyType, 
	join_nonce: u32, // 3 bytes
	join_eui: u64,   // 8 bytes
	dev_j_count: u16 // 2 bytes
) -> [u8; 16] {
	let mut block: [u8; 16] = [0; 16];
	block[0] = target_key_type as u8;
	block[1..4].copy_from_slice(&join_nonce.to_le_bytes()[..3]);
	block[4..12].copy_from_slice(&join_eui.to_le_bytes());
	block[12..14].copy_from_slice(&dev_j_count.to_le_bytes());
	aes128_encrypt( master_key, &block )
}



///
/// derive_s_keys
/// - app_key       AppKey
/// - nwk_key       NwkKey
/// - join_nonce    JoinNonce
/// - join_eui      JoinEUI
/// - dev_j_count   Depends on the frame type of the request that triggers the Join-Accept frame.
///                   Join-Request               -> DevNonce
///                   Rejoin-Request type 0 or 2 -> RJcount02
///                   Rejoin-Request type 1      -> RJcount1
/// 

pub fn derive_s_keys(
	app_key: &[u8; 16],
	nwk_key: &[u8; 16],
	join_nonce: u32,  // 3 bytes
	join_eui: u64,    // 8 bytes
	dev_j_count: u16  // 2 bytes
) -> SKeys {

	let mut block: [u8; 16] = [0; 16];

	block[1..4].copy_from_slice(&join_nonce.to_le_bytes()[..3]);
	block[4..12].copy_from_slice(&join_eui.to_le_bytes());
	block[12..14].copy_from_slice(&dev_j_count.to_le_bytes());

	block[0] = SKeyType::AppSKey as u8;
	let app_s_key = aes128_encrypt( 
		app_key, 
		&block 
	).try_into().unwrap();

	block[0] = SKeyType::FNwkSIntKey as u8;
	let f_nwk_s_int_key = aes128_encrypt(
		nwk_key,
		&block
	).try_into().unwrap();

	block[0] = SKeyType::SNwkSIntKey as u8;
	let s_nwk_s_int_key = aes128_encrypt(
		nwk_key,
		&block
	).try_into().unwrap();

	block[0] = SKeyType::NwkSEncKey as u8;
	let nwk_s_enc_key = aes128_encrypt(
		nwk_key,
		&block
	).try_into().unwrap();

	SKeys { 
		app_s_key, 
		f_nwk_s_int_key, 
		s_nwk_s_int_key, 
		nwk_s_enc_key 
	}

}

















#[derive(Default, PartialEq, Debug)]
pub struct DerivedKeyOptions {
	pub js_int_key: Option<[u8; 16]>,
	pub js_enc_key: Option<[u8; 16]>,
	pub app_s_key: Option<[u8; 16]>,
	pub f_nwk_s_int_key: Option<[u8; 16]>,
	pub s_nwk_s_int_key: Option<[u8; 16]>,
	pub nwk_s_enc_key: Option<[u8; 16]>,
}

#[derive(Default)]
pub struct MasterKeyOptions<'a> {
	pub app_key: Option<&'a [u8; 16]>,
	pub nwk_key: Option<&'a [u8; 16]>,
}

#[derive(Default)]
pub struct KeyDerivationVectorOptions {
	pub dev_eui: Option<u64>,
	pub join_nonce: Option<u32>,  // 3 bytes
	pub join_eui: Option<u64>,    // 8 bytes
	pub dev_j_count: Option<u16>  // 2 bytes
}

pub fn derive_key_options(
	derived_key_options: &mut DerivedKeyOptions,
	master_key_options: &MasterKeyOptions,
	derivation_vec_options: &KeyDerivationVectorOptions
) {

	let mut input_block: [u8; 16] = [0; 16];


	if let (Some(dev_eui), Some(nwk_key)) = (derivation_vec_options.dev_eui, master_key_options.nwk_key) {

		input_block[1..9].copy_from_slice(&dev_eui.to_le_bytes());

		input_block[0] = KeyType::JSIntKey as u8;
		derived_key_options.js_int_key = Some(aes128_encrypt( nwk_key, &input_block ).try_into().unwrap());
	
		input_block[0] = KeyType::JSEncKey as u8;
		derived_key_options.js_enc_key = Some(aes128_encrypt( nwk_key, &input_block ).try_into().unwrap());
	
	}


	if let (Some(join_nonce), Some(join_eui), Some(dev_j_count)) = (derivation_vec_options.join_nonce, derivation_vec_options.join_eui, derivation_vec_options.dev_j_count) {

		input_block[1..4].copy_from_slice(&join_nonce.to_le_bytes()[..3]);
		input_block[4..12].copy_from_slice(&join_eui.to_le_bytes());
		input_block[12..14].copy_from_slice(&dev_j_count.to_le_bytes());

		if let Some(app_key) = master_key_options.app_key {

			input_block[0] = KeyType::AppSKey as u8;
			derived_key_options.app_s_key = Some(aes128_encrypt( app_key, &input_block ).try_into().unwrap());

		}
	
		if let Some(nwk_key) = master_key_options.nwk_key {
	
			input_block[0] = KeyType::FNwkSIntKey as u8;
			derived_key_options.f_nwk_s_int_key = Some(aes128_encrypt( nwk_key, &input_block ).try_into().unwrap());
	
			input_block[0] = KeyType::SNwkSIntKey as u8;
			derived_key_options.s_nwk_s_int_key = Some(aes128_encrypt( nwk_key, &input_block ).try_into().unwrap());
	
			input_block[0] = KeyType::NwkSEncKey as u8;
			derived_key_options.nwk_s_enc_key = Some(aes128_encrypt( nwk_key, &input_block ).try_into().unwrap());
	
		}

	}

}


#[cfg(test)]
mod tests {

	use super::*;
	use crate::lorawan::crypto::utils::key_from_string;


// SESSION_KEYS_DERIVATION_SCHEME
	#[test]
	fn test_derive_session_keys() {
		
		let join_nonce: u32 = 0x13f; // 3 bytes only
		let join_eui: u64 = 0xc60680edce607b69;
		let dev_nonce: u16 = 0x905;

		let app_key = key_from_string("3ab224a63b2ace637bd0f61bf524078e").unwrap();
		let nwk_key = key_from_string("3ab224a63b2ace637bd0f61bf524078e").unwrap();

		let app_s_key = key_from_string("ccc8109a7cfb3ef234dd81ac959ca55a").unwrap();
		let f_nwk_s_int_key = key_from_string("bb5256bcd251a4fd471193eadcde7dc3").unwrap();
		let s_nwk_s_int_key = key_from_string("d3495206102789165829a4b689be525c").unwrap();
		let nwk_s_enc_key = key_from_string("1dddd79e081442554d61c9a9e5ea8cd5").unwrap();


		let mut derived_key_options = DerivedKeyOptions::default();
		let master_key_options = MasterKeyOptions{
			app_key: Some(&app_key),
			nwk_key: Some(&nwk_key),
		};
		let derivation_vec_options = KeyDerivationVectorOptions{
			dev_eui: None,
			join_nonce: Some(join_nonce),
			join_eui: Some(join_eui),
			dev_j_count: Some(dev_nonce),
		};
		derive_key_options(&mut derived_key_options, &master_key_options, &derivation_vec_options);


		let reference_key_options = DerivedKeyOptions{
			js_int_key: None,
			js_enc_key: None,
			app_s_key: Some(app_s_key),
			f_nwk_s_int_key: Some(f_nwk_s_int_key),
			s_nwk_s_int_key: Some(s_nwk_s_int_key),
			nwk_s_enc_key: Some(nwk_s_enc_key),
		};
		assert_eq!(reference_key_options, derived_key_options);

	}

	// JOIN_SERVER_LIFETIME_KEYS_DERIVATION_SCHEME
	#[test]
	fn test_derive_js_keys() {

		let dev_eui: u64 = 0xa88a15525dcb5e07;
		let nwk_key = key_from_string("3ab224a63b2ace637bd0f61bf524078e").unwrap();
		let js_int_key = key_from_string("0d72c86f3c2f05d91ff76531c7fedb04").unwrap();
		let js_enc_key = key_from_string("993e584f283ad769e7cf4eade655a7f8").unwrap();


		let mut derived_key_options = DerivedKeyOptions::default();
		let master_key_options = MasterKeyOptions{
			app_key: None,
			nwk_key: Some(&nwk_key),
		};
		let derivation_vec_options = KeyDerivationVectorOptions{
			dev_eui: Some(dev_eui),
			join_nonce: None,
			join_eui: None,
			dev_j_count: None,
		};
		derive_key_options(&mut derived_key_options, &master_key_options, &derivation_vec_options);

		let reference_key_options = DerivedKeyOptions{
			js_int_key: Some(js_int_key),
			js_enc_key: Some(js_enc_key),
			app_s_key: None,
			f_nwk_s_int_key: None,
			s_nwk_s_int_key: None,
			nwk_s_enc_key: None,
		};
		assert_eq!(reference_key_options, derived_key_options);

	}

}


