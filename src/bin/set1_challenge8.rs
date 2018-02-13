// Detect AES in ECB mode
// ======================
//
// In this file are a bunch of hex-encoded ciphertexts.
//
// One of them has been encrypted with ECB.
//
// Detect it.
//
// Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte
// plaintext block will always produce the same 16 byte ciphertext.
//

extern crate cryptopals;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::process;

use cryptopals::base64;

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

	let mut set_size = std::usize::MAX;
	let mut best_buffer: &str = "";
	let mut best_buffer_idx: usize = 0;
	for (idx, line) in base64_buffer.lines().enumerate() {
		//let line = line.trim();
		if let Ok(line_buffer) = base64::base64_decode(line.trim()) {
			let set: HashSet<&u8> = HashSet::from_iter(
			                                  line_buffer.iter());
			if set.len() < set_size {
				best_buffer = line;
				best_buffer_idx = idx + 1;
				set_size = set.len();
			}
		}
	}

	if best_buffer_idx > 0 {
		println!("Best buffer at line {}", best_buffer_idx);
		for c in best_buffer.chars() {
			print!("{}", c);
		}
		println!();
	}
}
