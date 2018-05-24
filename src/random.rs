use rand;
use rand::Rng;

pub fn random_fill(buf: &mut [u8]) -> ()
{
	let mut rng = rand::thread_rng();

	for b in buf.iter_mut() {
		*b = rng.gen();
	}
}
