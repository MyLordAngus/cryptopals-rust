fn u8_slices_to_u32(slice: &[u8]) -> u32
{
	assert!(slice.len() <= 4,
	        "cannot concatenate more than 4 bytes into 32 bits");

	let mut result = 0;

	let iter = slice.iter();
	for &s in iter {
		result = (result << 8) | (u32::from(s));
	}

	result
}

fn byte_to_base64_char(byte: u8) -> u8
{
	match byte {
		0 ... 0b01_1001 => b'A' + byte,
		0b01_1010 ... 0b11_0011 => b'a' + (byte - 0b01_1010),
		0b11_0100 ... 0b11_1101 => b'0' + (byte - 0b11_0100),
		0b11_1110 => b'+',
		_ => b'/'
	}
}

pub fn base64_encode(hex: &[u8]) -> Vec<u8>
{
	let mut base64 = vec![];

	if let Some((last_hex_chunk, hex_chunks)) =
		hex.chunks(3).collect::<Vec<_>>().split_last() {

		let hex_chunks_iter = hex_chunks.iter();
		for hex_slice in hex_chunks_iter {
			let hex_16bits = u8_slices_to_u32(hex_slice);

			for j in 0..4 {
				let c = byte_to_base64_char((hex_16bits >> (18 - (j * 6)) & 0x3f) as u8);
				base64.push(c);
			}
		}

		if last_hex_chunk.len() == 3 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..4 {
				let c = byte_to_base64_char((hex_16bits >> (18 - (j * 6)) & 0x3f) as u8);
				base64.push(c);
			}

		} else if last_hex_chunk.len() == 2 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..3 {
				let c = byte_to_base64_char((hex_16bits >> (12 - (j * 6)) & 0x3f) as u8);
				base64.push(c);
			}
			base64.push(b'=');

		} else if last_hex_chunk.len() == 1 {
			let hex_16bits = u8_slices_to_u32(last_hex_chunk);
			for j in 0..2 {
				let c = byte_to_base64_char((hex_16bits >> (6 - (j * 6)) & 0x3f) as u8);
				base64.push(c);
			}
			base64.push(b'=');
			base64.push(b'=');
		}
	}

	base64
}

fn u32_to_u8_slice(four_bytes: u32) -> [u8; 4]
{
	let mut result: [u8; 4] = [0, 0, 0, 0];

	for (i, item) in result.iter_mut().take(4).enumerate() {
		*item = ((((four_bytes << (i * 8)) & 0xff00_0000) >> 24)
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

	let chunks = buf.as_bytes().chunks(4);
	for four_chars in chunks {
		if four_chars[0] == b'=' || four_chars[1] == b'=' {
			return Err("use of '=' in a non-allowed position");
		}

		let mut three_bytes: u32 = 0;
		let mut bytes_count: usize = 1;

		let mut four_chars_iter = four_chars.iter();
		while let Some(c) = four_chars_iter.next() {
			if *c >= b'A' && *c <= b'Z' {
				let byte: u8 = c - b'A';
				three_bytes = (three_bytes << 6) | u32::from(byte);
			} else if *c >= b'a' && *c <= b'z' {
				let byte: u8 = 0b01_1010 + (c - b'a');
				three_bytes = (three_bytes << 6) | u32::from(byte);
			} else if *c >= b'0' && *c <= b'9' {
				let byte: u8 = 0b11_0100 + (c - b'0');
				three_bytes = (three_bytes << 6) | u32::from(byte);
			} else if *c == b'+' {
				let byte: u8 = 0b11_1110;
				three_bytes = (three_bytes << 6) | u32::from(byte);
			} else if *c == b'/' {
				let byte: u8 = 0b11_1111;
				three_bytes = (three_bytes << 6) | u32::from(byte);

			// special alignment character
			} else if *c == b'=' {
				three_bytes >>= 2;
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
