extern crate cryptopals;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;

use cryptopals::ascii;
use cryptopals::base64;
use cryptopals::bit_utils;
use cryptopals::cryptography::xor;

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

	// --------------------------------------------------------------------
	// estimate key size
	// --------------------------------------------------------------------

	let mut keysize_difference: (usize, f32) = (0, std::f32::MAX);
	for keysize in 2..41 {
		let mut distances: Vec<f32> = Vec::new();
		let chunks = buffer.chunks(keysize).collect::<Vec<_>>();

		// hamming distance between first and all other blocks
		// normalize it by dividing by size of block
		for i in 0..chunks.len() {
			let mut distance =
			    bit_utils::hamming_distance(chunks[0], chunks[i]) as f32;
			distances.push(distance / keysize as f32);
		}

		// calculate average hamming distance
		// save keysize if average hamming distance is smaller
		let avg_distance = distances.iter().sum::<f32>()
		                   / chunks.len() as f32;
		if avg_distance < keysize_difference.1 {
			keysize_difference.0 = keysize;
			keysize_difference.1 = avg_distance;
		}
	}

	let best_keysize = keysize_difference.0;
	println!("Best keysize = {}", best_keysize);

	// --------------------------------------------------------------------
	// estimate the key
	// --------------------------------------------------------------------

	// single-char xor each blocks group and estimate probability of being
	// the key
	let mut key: Vec<u8> = Vec::with_capacity(best_keysize);
	for i in 0..best_keysize {
		let key_byte_buffer = buffer.iter().enumerate()
		.filter(|elem| {
			elem.0 % best_keysize == i
		}).map(|elem| {
			*elem.1
		}).collect::<Vec<_>>();

		let mut best_char: char = ' ';
		let mut best_estimation = 0;
		for c in 0..255 {
			let decoded = xor::single_char_xor(&key_byte_buffer, c);
			if let Ok(string) = String::from_utf8(decoded) {
				let estimation = ascii::estimate_english_sentence(&string);
				if estimation > best_estimation {
					best_char = c as char;
					best_estimation = estimation;
				}
			}
		}

		key.push(best_char as u8);
	}

	key.iter().for_each(|&c| print!("{}", c as char));
	println!();

	// --------------------------------------------------------------------
	// decode the buffer with the key
	// --------------------------------------------------------------------

	let decoded = xor::repeating_xor(&buffer, &key);
	for c in 0..45 {
		print!("{}", decoded[c] as char);
	}
	println!();
}
