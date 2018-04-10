// Implement CBC mode
// ==================
//
// CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite the fact that a block cipher natively only transforms individual blocks.
//
// In CBC mode, each ciphertext block is added to the next plaintext block before the next call to the cipher core.
//
// The first plaintext block, which has no associated previous ciphertext block, is added to a "fake 0th ciphertext block" called the initialization vector, or IV.
//
// Implement CBC mode by hand by taking the ECB function you wrote earlier, making it encrypt instead of decrypt (verify this by decrypting whatever you encrypt to test), and using your XOR function from the previous exercise to combine them. 

extern crate cryptopals;

use std::fs::File;
use std::io::prelude::*;
use std::process;

use cryptopals::file;
use cryptopals::cryptography::aes;

fn main()
{
	let mut file = File::open("assets/set2_challenge10_2.txt").unwrap_or_else(|_err| {
		println!("cannot open input file");
		process::exit(1);
	});

	let mut contents = Vec::new();
	file.read_to_end(&mut contents).unwrap_or_else(|_err| {
		println!("error while reading input file");
		process::exit(1);
	});

	let encrypted = aes::emulate_cbc_encrypt(
	  b"YELLOW SUBMARINE", &[0x00; 16], &contents).unwrap();

	file::write_base64_fmt_file(&encrypted, "/tmp/output.txt")
	  .unwrap_or_else(|_err| {
		  println!("error while writing result to output file");
		  process::exit(1);
	  });
}
