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
