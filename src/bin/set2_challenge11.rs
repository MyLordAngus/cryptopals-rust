// An ECB/CBC detection oracle
// ===========================
//
// Now that you have ECB and CBC working:
//
// Write a function to generate a random AES key; that's just 16 random bytes.
//
// Write a function that encrypts data under an unknown key --- that is, a function that generates
// a random key and encrypts under it.
//
// The function should look like:
//
// encryption_oracle(your-input)
// => [MEANINGLESS JIBBER JABBER]
//
// Under the hood, have the function append 5-10 bytes (count chosen randomly) before the plaintext
// and 5-10 bytes after the plaintext.
//
// Now, have the function choose to encrypt under ECB 1/2 the time, and under CBC the other half
// (just use random IVs each time for CBC). Use rand(2) to decide which to use.
//
// Detect the block cipher mode the function is using each time. You should end up with a piece of
// code that, pointed at a block box that might be encrypting ECB or CBC, tells you which one is
// happening.
//

extern crate cryptopals;

use std::fs::File;
use std::io::prelude::*;
use std::process;

use cryptopals::cryptography::aes;
use cryptopals::cryptography::pkcs;

/// try to detect if a block has been encrypted using aes ecb
/// in order to be meaningful, encrypted data size should be at least 32 bytes
/// (2 times 16 bytes aes key)
pub fn detect_aes_ecb(buf: &[u8]) -> bool
{
	buf.windows(32).any(|win| win[0..16] == win[16..32])
}

fn main()
{
	let mut file = File::open("./assets/set2_challenge10.txt")
	  .expect("cannot open file to encrypt");

	let mut buffer = String::new();
	file.read_to_string(&mut buffer).expect("cannot read file to encrypt");

	for line in buffer.lines() {
		let mut pad_buffer = b"YELLOW SUBMARINEYELLOW SUBMARINE".to_vec();
		pad_buffer.extend_from_slice(line.as_bytes());
		pkcs::pkcs7_padding(&mut pad_buffer, 16);

		let rand_key = aes::generate_random_key();
		let oracle_result =
			aes::encrypt_oracle(&rand_key, &pad_buffer)
			  .unwrap_or_else(|_| { process::exit(1); });

		if detect_aes_ecb(&oracle_result.1) {
			println!("ECB detected");
			assert_eq!(oracle_result.0, aes::CipherMode::ECB);
		} else {
			println!("CBC detected");
			assert_eq!(oracle_result.0, aes::CipherMode::CBC);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn detect_aes_ecb_test()
	{
		let buf = b"YELLOW SUBMARINEYELLOW SUBMARINE";
		assert!(detect_aes_ecb(buf));

		let buf = b"YELLOW SUBMARINEOTHER";
		assert_eq!(detect_aes_ecb(buf), false);

		let buf = b"";
		assert_eq!(detect_aes_ecb(buf), false);
	}
}
