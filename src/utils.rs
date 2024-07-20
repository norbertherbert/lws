use std::fmt;

//********************************
//* Buf
//********************************

pub const MAX_BUF_SIZE: usize = 256;


struct VectorDisplay(Vec<u8>);
impl fmt::Display for VectorDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", hex::encode(&self.0[..]))
    }
}

pub fn vector_to_hex(v: &Vec<u8>) -> String {
	format!("{}", hex::encode(&v[..]))
}

#[test]
fn test_vector_display() {
	let v = vec![0x01, 0x02, 0x03, 0x04];
	println!("Encoded: {}", hex::encode(&v));
}


#[derive(Debug)]
pub struct Buf {
	pub bytes: [u8; MAX_BUF_SIZE],
	pub len: usize
}
impl Buf {
	pub fn new() -> Self {
		Buf { bytes: [0_u8; MAX_BUF_SIZE], len: 0 }
	}
	pub fn from_bytes(slice: &[u8]) -> Self {
		let mut len: usize = slice.len();
		if len > MAX_BUF_SIZE { len = MAX_BUF_SIZE; }
		let mut bytes = [0_u8; MAX_BUF_SIZE];
		bytes[..len].copy_from_slice(&slice[..len]);
		Buf { bytes, len }
	}
	pub fn as_bytes(&self) -> &[u8] {
		&self.bytes[..]
	}
}
impl fmt::Display for Buf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", hex::encode(&self.bytes[..]))
	}
}
