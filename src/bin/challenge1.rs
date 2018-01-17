extern crate cryptopals;

use std::env;
use cryptopals::set1;
use cryptopals::utils;

fn usage()
{
	println!("wrong number of arguments/hex string format !")
}

fn main()
{
	if env::args_os().count() != 2 {
		usage();
		std::process::exit(1);
	}

	let hex_os_str = env::args_os().nth(1).unwrap();
	let hex_str = hex_os_str.to_str().unwrap();
	let hex_buf = utils::str_to_byte_array(hex_str).unwrap_or_else(|_: ()| {
		usage();
		std::process::exit(1);
	});

	let base64 = set1::base64_encode(hex_buf.as_slice());
	for c in base64 {
		print!("{}", c as char)
	}
	println!();
}
