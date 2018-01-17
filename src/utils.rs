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
