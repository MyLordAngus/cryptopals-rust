use cryptography::openssl_lib;
use cryptography::pkcs;
use cryptography::xor;

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
