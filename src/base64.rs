fn u8_slices_to_u32(slice: &[u8]) -> u32
{
	assert!(slice.len() <= 4,
	        "cannot concatenate more than 4 bytes into 32 bits");

	let mut result = 0;

	let iter = slice.iter();
	for &s in iter {
		result = (result << 8) | (s as u32);
	}

	result
}

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

	if let Some((last_hex_chunk, hex_chunks)) =
		hex.chunks(3).collect::<Vec<_>>().split_last() {

		let mut hex_chunks_iter = hex_chunks.iter();
		while let Some(hex_slice) = hex_chunks_iter.next() {
			let hex_16bits = u8_slices_to_u32(hex_slice);

			for j in 0..4 {
				let c = byte_to_base64_char((hex_16bits >> 18 - (j * 6) & 0x3f) as u8);
				base64.push(c);
			}
		}

		if last_hex_chunk.len() == 3 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..4 {
				let c = byte_to_base64_char((hex_16bits >> 18 - (j * 6) & 0x3f) as u8);
				base64.push(c);
			}

		} else if last_hex_chunk.len() == 2 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..3 {
				let c = byte_to_base64_char((hex_16bits >> 12 - (j * 6) & 0x3f) as u8);
				base64.push(c);
			}
			base64.push('=' as u8);

		} else if last_hex_chunk.len() == 1 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..2 {
				let c = byte_to_base64_char((hex_16bits >> 6 - (j * 6) & 0x3f) as u8);
				base64.push(c);
			}
			base64.push('=' as u8);
			base64.push('=' as u8);
		}
	}

	base64
}

fn u32_to_u8_slice(four_bytes: u32) -> [u8; 4]
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

	// base64 buffer must be 4 characters-aligned
	if buf.len() % 4 != 0 {
		return Err("non 4-characters aligned base64 buffer");
	}

	let mut chunks = buf.as_bytes().chunks(4);
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn base64_encode_test()
	{
		assert_eq!(base64_encode(&[0x01, 0x02, 0x03]),
		           "AQID".as_bytes());
		assert_eq!(base64_encode(&[0x87, 0x22, 0x93, 0xaa, 0xf0, 0xb2, 0x00]),
		           "hyKTqvCyAA==".as_bytes());
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
}
