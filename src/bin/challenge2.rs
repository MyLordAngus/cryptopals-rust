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
	if env::args_os().count() != 3 {
		usage();
		std::process::exit(1);
	}

	let hex_os_str = env::args_os().nth(1).unwrap();
	let key_os_str = env::args_os().nth(2).unwrap();

	let hex_str = hex_os_str.to_str().unwrap();
	let hex_buf = utils::str_to_byte_array(hex_str).unwrap_or_else(|_: ()| {
		usage();
		std::process::exit(1);
	});

	let key_str = key_os_str.to_str().unwrap();
	let key_buf = utils::str_to_byte_array(key_str).unwrap_or_else(|_: ()| {
		usage();
		std::process::exit(1);
	});

	let xor = set1::fixed_xor(&hex_buf, &key_buf).unwrap_or_else(|_: ()| {
		usage();
		std::process::exit(1);
	});

	for b in xor {
		print!("{:x}", b)
	}
	println!();
}
