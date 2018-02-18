pub fn pkcs7_padding(buffer: &mut Vec<u8>, block_size: u8)
{
	let to_pad = (block_size as usize)
	             - (buffer.len() % block_size as usize);

	let len = buffer.len();
	buffer.resize(len + to_pad, to_pad as u8);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pkcs7_padding_test()
	{
		let string = "YELLOW SUBMARINE".as_bytes().to_vec();

		let mut string_pad_4 = string.clone();
		string_pad_4.extend_from_slice(&[0x04; 4]);
		let mut string_pad_16 = string.clone();
		string_pad_16.extend_from_slice(&[0x10; 16]);

		let mut to_pad_buffer = string.clone();
		pkcs7_padding(&mut to_pad_buffer, 20);
		assert_eq!(to_pad_buffer, string_pad_4);

		to_pad_buffer = "".as_bytes().to_vec();
		pkcs7_padding(&mut to_pad_buffer, 16);
		assert_eq!(to_pad_buffer, [0x10; 16].to_vec());

		let mut to_pad_buffer = string.clone();
		pkcs7_padding(&mut to_pad_buffer, 16);
		assert_eq!(to_pad_buffer, string_pad_16);
	}
}
