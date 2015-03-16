use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::ASCII;

#[allow(dead_code)]
pub fn ascii_encoding(str: &String) -> Vec<u8> {
			ASCII.encode(&str, EncoderTrap::Strict).unwrap()
}

#[allow(dead_code)]
pub fn ascii_decoding(en: &Vec<u8>) -> String {
	match ASCII.decode(&en, DecoderTrap::Strict) {
			Ok(str) => str.to_string(),
			Err(why) => why.to_string(),
		}
}
