extern crate openssl;

use self::openssl::error;
use self::openssl::symm::{Cipher,Crypter,Mode,decrypt};

pub fn aes_ecb_decrypt(key: &[u8], buffer: &[u8]) ->
        Result<Vec<u8>, error::ErrorStack>
{
	let decoded_buffer: Vec<u8>;
	let cipher = Cipher::aes_128_ecb();

	decoded_buffer = decrypt(cipher, key, None, &buffer)?;

	Ok(decoded_buffer)
}

pub fn aes_ecb_encrypt(key: &[u8], buffer: &[u8]) ->
        Result<Vec<u8>, error::ErrorStack>
{
	let cipher = Cipher::aes_128_ecb();
	let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None)?;
	crypter.pad(false);

	let mut encrypted = vec![0x00; buffer.len() + cipher.block_size()];
	let mut encrypted_size;

	encrypted_size = crypter.update(buffer, &mut encrypted)?;
	encrypted_size += crypter.finalize(&mut encrypted)?;
	encrypted.truncate(encrypted_size);

	Ok(encrypted)
}
