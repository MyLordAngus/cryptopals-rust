extern crate cryptopals;

use cryptopals::set1;

fn main()
{
	let buffer = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
	let key = "ICE";

	let xor = set1::repeating_xor(&buffer.as_bytes(), &key.as_bytes());
	for b in &xor {
		print!("{:02x}", b)
	}
	println!();
}
