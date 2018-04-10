// Implement PKCS#7 padding
// ========================
//
// A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into
// ciphertext. But we almost never want to transform a single block; we encrypt irregularly-sized
// messages.
//
// One way we account for irregularly-sized messages is by padding, creating a plaintext that is an
// even multiple of the blocksize. The most popular padding scheme is called PKCS#7.
//
// So: pad any block to a specific block length, by appending the number of bytes of padding to the
// end of the block. For instance,
//
// "YELLOW SUBMARINE"
//
// ... padded to 20 bytes would be:
//
// "YELLOW SUBMARINE\x04\x04\x04\x04"
//

extern crate cryptopals;

use std::env;
use std::num;
use std::process;

use cryptopals::cryptography::pkcs;

fn main()
{
	let buffer_os_str = env::args_os().nth(1).unwrap_or_else(|| {
		println!("wrong number of arguments");
		process::exit(1);
	});
	let buffer = buffer_os_str.to_str().unwrap_or_else(|| {
		println!("wrong number of arguments");
		process::exit(1);
	});

	let block_size_os_str = env::args_os().nth(2).unwrap_or_else(|| {
		println!("wrong number of arguments");
		process::exit(1);
	});
	let block_size = block_size_os_str.to_str().unwrap_or_else(|| {
		println!("wrong number of arguments");
		process::exit(1);
	});
	let block_size = u8::from_str_radix(block_size, 10).unwrap_or_else(|_err: num::ParseIntError| {
		println!("wrong block size");
		process::exit(1);
	});

	let mut pad_buffer = buffer.as_bytes().to_vec();
	pkcs::pkcs7_padding(&mut pad_buffer, block_size);
	for b in &pad_buffer {
		let c = *b as char;
		if c.is_alphanumeric() || c.is_whitespace() {
			print!("{}", c);
		} else {
			print!("\\x{:02x}", b);
		}
	}
	println!();
}
