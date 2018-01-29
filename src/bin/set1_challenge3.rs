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

	let mut xor: Vec<u8> = Vec::new();
	let mut score = 0;
	for c in 0..255 {
		let xor_test = set1::single_char_xor(&hex_buf, c);

		if let Ok(s) = String::from_utf8(xor_test.clone()) {
			let new_score = utils::estimate_english_sentence(&s);

			if score < new_score {
				xor = xor_test;
				score = new_score;
			}
		}
	}

	for b in &xor {
		print!("{}", *b as char);
	}
	println!();
}
