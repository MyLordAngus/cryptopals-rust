fn xor(a: u8, b: u8) -> u8
{
	a ^ b
}

pub fn fixed_xor(hex: &[u8], key: &[u8]) -> Result<Vec<u8>, ()>
{
	if hex.len() != key.len() {
		return Err(());
	}

	let mut xor_vec: Vec<u8> = Vec::new();
	let mut iter = hex.iter().zip(key);

	while let Some(tuple) = iter.next() {
		xor_vec.push(xor(*tuple.0, *tuple.1));
	}

	Ok(xor_vec)
}

pub fn single_char_xor(hex: &[u8], key: u8) -> Vec<u8>
{
	let mut xor_vec: Vec<u8> = Vec::new();
	let mut iter = hex.iter();

	while let Some(value) = iter.next() {
		xor_vec.push(xor(*value, key));
	}

	xor_vec
}

pub fn repeating_xor(buf: &[u8], key: &[u8]) -> Vec<u8>
{
	let cycle = key.iter().cycle();
	let mut zip = buf.iter().zip(cycle);

	let mut xor_vec :Vec<u8> = vec![];

	while let Some(v) = zip.next() {
		xor_vec.push(xor(*v.0, *v.1));
	}

	xor_vec
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
}
