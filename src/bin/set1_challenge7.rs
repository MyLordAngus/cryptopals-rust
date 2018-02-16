extern crate cryptopals;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;

use cryptopals::base64;
use cryptopals::cryptography::openssl_lib;

fn main()
{
	let filename_args_os = env::args_os().nth(1).unwrap_or_else(|| {
		println!("wrong number of arguments/hex string format in file!");
		process::exit(1);
	});
	let filename = filename_args_os.to_str().unwrap_or_else(|| {
		println!("wrong number of arguments/hex string format in file!");
		process::exit(1);
	});

	let mut file = File::open(filename).unwrap_or_else(|err: std::io::Error| {
		println!("cannot open file {:?}: {}", filename, err.description());
		process::exit(1);
	});

	let mut base64_buffer = String::new();
	if let Err(s) = file.read_to_string(&mut base64_buffer) {
		println!("error while reading file: {}", s.description());
		process::exit(1);
	}
	base64_buffer = base64_buffer.replace("\n", "");

	let buffer = base64::base64_decode(&base64_buffer).unwrap_or_else(|err: &str| {
		println!("error during base64 parsing: {}", err);
		process::exit(1);
	});

	let key = "YELLOW SUBMARINE".as_bytes();

	let decoded = openssl_lib::aes_decrypt(key, None, &buffer).unwrap_or_else(|_| {
		println!("error while decoding buffer");
		process::exit(1);
	});

	for c in &decoded {
		print!("{}", *c as char);
	}
	println!();
}
