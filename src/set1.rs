fn byte_to_base64_char(byte: u8) -> u8
{
	match byte {
		0 ... 0b011001 => 'A' as u8 + byte,
		0b011010 ... 0b110011 => 'a' as u8 + (byte - 0b011010),
		0b110100 ... 0b111101 => '0' as u8 + (byte - 0b110100),
		0b111110 => '+' as u8,
		_ => '/' as u8
	}
}

pub fn base64_encode(hex: &[u8]) -> Vec<u8>
{
	let mut base64 = vec![];

	let mut hex_chunks = hex.chunks(3);
	loop {
		let hex_slice = match hex_chunks.next() {
			Some(s) => s,
			None => break,
		};

		match hex_slice.len() {
			3 => {
				let hex_16bits = ((hex_slice[0] as u32) << 16) & 0xff0000
					| ((hex_slice[1] as u32) << 8) & 0x00ff00
					| (hex_slice[2] as u32) & 0x0000ff;

				for j in 0..4 {
					let c = byte_to_base64_char((hex_16bits >> 18 - (j * 6) & 0x3f) as u8);
					base64.push(c);
				}
			},
			2 => {
				let hex_16bits = (((hex_slice[0] as u32) << 8) & 0x00ff00
						    | ((hex_slice[1] as u32)) & 0x0000ff)
					<< 2;

				for j in 0..3 {
					let c = byte_to_base64_char((hex_16bits >> 12 - (j * 6) & 0x3f) as u8);
					base64.push(c);
				}
				base64.push('=' as u8);
			},
			1 => {
				let hex_16bits = (((hex_slice[0] as u32)) & 0x0000ff)
					<< 4;

				for j in 0..2 {
					let c = byte_to_base64_char((hex_16bits >> 6 - (j * 6) & 0x3f) as u8);
					base64.push(c);
				}
				base64.push('=' as u8);
				base64.push('=' as u8);
			},
			_ => panic!("this case shall not happened !"),
		}
	}

	base64
}

pub fn u32_to_u8_slice(four_bytes: u32) -> [u8; 4]
{
	let mut result: [u8; 4] = [0, 0, 0, 0];

	for i in 0..4 {
		result[i] = ((((four_bytes << (i * 8)) & 0xff000000) >> 24)
		             & 0xff) as u8;
	}

	result
}

pub fn base64_decode(buf: &str) -> Result<Vec<u8>, &'static str>
{
	let mut result: Vec<u8> = vec![];
	let mut chunks = buf.as_bytes().chunks(4);

	// base64 buffer must be 4 characters-aligned
	if buf.len() % 4 != 0 {
		return Err("non 4-characters aligned base64 buffer");
	}

	while let Some(four_chars) = chunks.next() {
		if four_chars[0] == '=' as u8 || four_chars[1] == '=' as u8 {
			return Err("use of '=' in a non-allowed position");
		}

		let mut three_bytes: u32 = 0;
		let mut bytes_count: usize = 1;

		let mut four_chars_iter = four_chars.iter();
		while let Some(c) = four_chars_iter.next() {
			if *c >= 'A' as u8 && *c <= 'Z' as u8 {
				let byte: u8 = 0b000000 + (c - 'A' as u8);
				three_bytes = (three_bytes << 6) | byte as u32;
			} else if *c >= 'a' as u8 && *c <= 'z' as u8 {
				let byte: u8 = 0b011010 + (c - 'a' as u8);
				three_bytes = (three_bytes << 6) | byte as u32;
			} else if *c >= '0' as u8 && *c <= '9' as u8 {
				let byte: u8 = 0b110100 + (c - '0' as u8);
				three_bytes = (three_bytes << 6) | byte as u32;
			} else if *c == '+' as u8 {
				let byte: u8 = 0b111110;
				three_bytes = (three_bytes << 6) | byte as u32;
			} else if *c == '/' as u8 {
				let byte: u8 = 0b111111;
				three_bytes = (three_bytes << 6) | byte as u32;

			// special alignment character
			} else if *c == '=' as u8 {
				three_bytes = three_bytes >> 2;
				bytes_count += 1;

			// no other character is allowed in base64 encoding
			} else {
				return Err("unparsable character in base64 buffer");
			}
		}

		let three_bytes = u32_to_u8_slice(three_bytes);
		result.extend_from_slice(&three_bytes[bytes_count..]);
	}

	Ok(result)
}

pub fn fixed_xor(hex: &[u8], key: &[u8]) -> Result<Vec<u8>, ()>
{
	if hex.len() != key.len() {
		return Err(());
	}

	let mut xor: Vec<u8> = Vec::new();
	let mut iter = hex.iter().zip(key);

	while let Some(tuple) = iter.next() {
		xor.push(tuple.0 ^ tuple.1);
	}

	Ok(xor)
}

pub fn single_char_xor(hex: &[u8], key: u8) -> Vec<u8>
{
	let mut xor: Vec<u8> = Vec::new();
	let mut iter = hex.iter();

	while let Some(value) = iter.next() {
		xor.push(value ^ key);
	}

	xor
}

pub fn repeating_xor(buf: &[u8], key: &[u8]) -> Vec<u8>
{
	let cycle = key.iter().cycle();
	let mut zip = buf.iter().zip(cycle);

	let mut result :Vec<u8> = vec![];

	while let Some(v) = zip.next() {
		result.push(v.0 ^ v.1);
	}

	result
}

pub fn hamming_distance(u1: &[u8], u2: &[u8]) -> u8
{
	let mut result: u8 = 0;
	let mut zip = u1.iter().zip(u2);

	while let Some(t) = zip.next() {
		for i in 0..8 {
			if (t.0 >> i) & 0x01u8 != (t.1 >> i) & 0x01u8 {
				result += 1;
			}
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn repeating_xor_test()
	{
		let buf = vec![0x01, 0x02, 0xa0, 0xff];
		let key = vec![0x01, 0xff, 0x01];

		assert_eq!(repeating_xor(&buf, &key),
		           vec![0x00, 0xfd, 0xa1, 0xfe]);
	}

	#[test]
	fn u32_to_u8_slice_test()
	{
		let slice = [0x12, 0x34, 0x56, 0x78];
		assert_eq!(u32_to_u8_slice(0x12345678), slice);
	}

	#[test]
	fn base64_decode_test()
	{
		assert_eq!(base64_decode("abcd0123"),
		           Ok(vec![0x69, 0xb7, 0x1d, 0xd3, 0x5d, 0xb7]));
		assert_eq!(base64_decode("abcd012="),
		           Ok(vec![0x69, 0xb7, 0x1d, 0xd3, 0x5d]));
		assert_eq!(base64_decode("abcd01=="),
		           Ok(vec![0x69, 0xb7, 0x1d, 0xd3]));

		// wrong usage of '=' character
		assert!(base64_decode("abcd0===").is_err());

		// non 4-characters aligned string
		assert!(base64_decode("abcd0").is_err());
	}

	#[test]
	fn hamming_distance_test()
	{
		assert_eq!(hamming_distance(&[0x00], &[0xff]), 8);
		assert_eq!(hamming_distance(&[0x0f], &[0xf0]), 8);
		assert_eq!(hamming_distance(&[0x0f], &[0xff]), 4);
		assert_eq!(hamming_distance(&[0xf0], &[0xff]), 4);

		assert_eq!(hamming_distance(&[0xf0, 0xff], &[0xff, 0xf0]), 8);

		assert_eq!(hamming_distance("this is a test".as_bytes(),
		                            "wokka wokka!!!".as_bytes()), 37);
	}

}
