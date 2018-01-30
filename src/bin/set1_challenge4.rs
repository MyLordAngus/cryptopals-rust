extern crate cryptopals;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
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

	let filename = env::args_os().nth(1).unwrap();
	let f = File::open(&filename).unwrap_or_else(|_: std::io::Error| {
		println!("error opening file {:?}", filename);
		std::process::exit(1);
	});
	let reader = BufReader::new(f);

	let mut xor: Vec<u8> = Vec::new();
	let mut score = 0;

	for line in reader.lines() {
		let hex_str = match line {
			Ok(hex_str) => hex_str,
			Err(_) => continue,
		};
		let hex_buf = match utils::str_to_byte_array(&hex_str) {
			Ok(hex_buf) => hex_buf,
			Err(_) => continue,
		};

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
	}

	for b in &xor {
		print!("{}", *b as char);
	}
	println!();
}
