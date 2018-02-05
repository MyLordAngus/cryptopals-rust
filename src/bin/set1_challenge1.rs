extern crate cryptopals;

use std::env;
use std::process;

use cryptopals::ascii;
use cryptopals::base64;

fn usage()
{
	println!("wrong number of arguments/hex string format !")
}

fn main()
{
	if env::args_os().count() != 2 {
		usage();
		process::exit(1);
	}

	let hex_os_str = env::args_os().nth(1).unwrap();
	let hex_str = hex_os_str.to_str().unwrap();
	let hex_buf = ascii::str_to_byte_array(hex_str).unwrap_or_else(|_: ()| {
		usage();
		process::exit(1);
	});

	let base64 = base64::base64_encode(hex_buf.as_slice());
	for c in base64 {
		print!("{}", c as char)
	}
	println!();
}
