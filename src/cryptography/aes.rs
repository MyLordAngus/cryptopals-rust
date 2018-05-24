use cryptography::openssl_lib;
use cryptography::pkcs;
use cryptography::xor;

use rand;
use rand::Rng;

use random;

pub fn emulate_cbc_encrypt(key: &[u8], iv: &[u8], buf: &[u8]) ->
        Result<Vec<u8>, ()>
{
	let mut encrypted = vec![];

	let mut chunks = buf.chunks(key.len());
	let mut previous_block = iv.to_vec();

	while let Some(chunk) = chunks.next() {
		let mut pad_buffer = chunk.to_vec();
		if pad_buffer.len() < key.len() {
			pkcs::pkcs7_padding(&mut pad_buffer, key.len() as u8);
		}

		let block = xor::fixed_xor(&pad_buffer, &previous_block)?;
		let mut encrypted_block =
		  openssl_lib::aes_ecb_encrypt(key, &block).map_err(|_x| {
			  ()
		  })?;

		previous_block = encrypted_block.clone();
		encrypted.append(&mut encrypted_block);
	}

	Ok(encrypted)
}

pub fn generate_random_key() -> [u8; 16]
{
	rand::random::<[u8; 16]>()
}

#[derive(Rand, PartialEq, Debug)]
pub enum CipherMode
{
	ECB,
	CBC,
}

type EncOracleReturnType = (CipherMode, Vec<u8>);

pub fn encrypt_oracle(key: &[u8], buf: &[u8]) -> Result<EncOracleReturnType, ()>
{
	let mut rng = rand::thread_rng();
	let enc_buf;

	// generate a random buffer of size between 5 and 10
	let mut to_append_buf = Vec::with_capacity(rng.gen_range(5, 11));
	random::random_fill(&mut to_append_buf);

	// prepend and append random buffer before buffer to encrypt
	let mut to_encrypt_buf = vec![];
	to_encrypt_buf.extend_from_slice(&to_append_buf);
	to_encrypt_buf.extend_from_slice(buf);
	to_encrypt_buf.extend_from_slice(&to_append_buf);

	let algorithm_rand: CipherMode = rng.gen();
	match algorithm_rand {
		CipherMode::ECB => {
			// ecb encrypt
			enc_buf = openssl_lib::aes_ecb_encrypt(key, &to_encrypt_buf)
			  .map_err(|_err| ())?;
		},
		CipherMode::CBC => {
			// cbc encrypt
			// for iv, use random data (same size as key)
			let iv = generate_random_key();
			enc_buf = emulate_cbc_encrypt(key, &iv, &to_encrypt_buf)?;
		},
	}

	Ok((algorithm_rand, enc_buf))
}
