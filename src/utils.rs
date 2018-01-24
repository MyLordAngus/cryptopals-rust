use std::collections::HashMap;
use std::ascii::AsciiExt;

pub fn str_to_byte_array(s: &str) -> Result<Vec<u8>, ()>
{
	let mut vec: Vec<u8> = vec![];
	let mut str_iter = s.chars();

	let mut i = 0;
	while i < s.len() {
		let mut byte_str = String::with_capacity(2);
		byte_str.push(str_iter.next().ok_or(())?);
		byte_str.push(str_iter.next().ok_or(())?);

		vec.push(u8::from_str_radix(byte_str.as_str(), 16).unwrap());

		i += 2
	}

	Ok(vec)
}

pub fn estimate_english_sentence(s: &str) -> u32
{
	let mut score: u32 = 0;
	let mut chars = s.chars();

	let probabilities: HashMap<char, u32> =
	[('a', 43),
	 ('b', 10),
	 ('c', 23),
	 ('d', 3),
	 ('e', 56),
	 ('f', 9),
	 ('g', 12),
	 ('h', 15),
	 ('i', 38),
	 ('j', 1),
	 ('k', 5),
	 ('l', 27),
	 ('m', 15),
	 ('n', 33),
	 ('o', 36),
	 ('p', 16),
	 ('q', 0),
	 ('r', 38),
	 ('s', 29),
	 ('t', 35),
	 ('u', 18),
	 ('v', 5),
	 ('w', 6),
	 ('x', 1),
	 ('y', 9),
	 ('z', 1),
	].iter().cloned().collect();

	while let Some(c) = chars.next() {
		// eliminate string with a non-ascii character
		if ! c.is_ascii() {
			return 0;
		}

		// do not count non alphabetic character
		if ! c.is_alphabetic() {
			continue;
		}

		// convert character to lowercase if needed and get score
		if let Some(lowercase) = c.to_lowercase().next() {
			if probabilities.contains_key(&lowercase) {
				score += probabilities[&lowercase];
			}
		}
	}

	score
}
