extern crate shCrypto;

#[test]
fn ascii_encoding_test() {
	let o_str: String = "Maidsafe".to_string();
	let en_str = vec![77, 97, 105, 100, 115, 97, 102, 101];
	assert_eq!(en_str, shCrypto::ascii::ascii_encoding(&o_str));
}

#[test]
fn ascii_decoding_test() {
	let o_str: String = "Maidsafe".to_string();
	let en_str = vec![77, 97, 105, 100, 115, 97, 102, 101];
	assert_eq!(o_str, shCrypto::ascii::ascii_decoding(&en_str));
}
