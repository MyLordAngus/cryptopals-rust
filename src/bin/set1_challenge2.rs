extern crate cryptopals;

use std::env;
use std::process;

use cryptopals::ascii;
use cryptopals::xor;

fn usage()
{
	println!("wrong number of arguments/hex string format !")
}

fn main()
{
	if env::args_os().count() != 3 {
		usage();
		process::exit(1);
	}

	let hex_os_str = env::args_os().nth(1).unwrap();
	let key_os_str = env::args_os().nth(2).unwrap();

	let hex_str = hex_os_str.to_str().unwrap();
	let hex_buf = ascii::str_to_byte_array(hex_str).unwrap_or_else(|_: ()| {
		usage();
		process::exit(1);
	});

	let key_str = key_os_str.to_str().unwrap();
	let key_buf = ascii::str_to_byte_array(key_str).unwrap_or_else(|_: ()| {
		usage();
		process::exit(1);
	});

	let xor = xor::fixed_xor(&hex_buf, &key_buf).unwrap_or_else(|_: ()| {
		usage();
		process::exit(1);
	});

	for b in xor {
		print!("{:x}", b)
	}
	println!();
}
