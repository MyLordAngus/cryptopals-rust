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
