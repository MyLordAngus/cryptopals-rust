extern crate openssl;

use self::openssl::error;
use self::openssl::symm::{Cipher,decrypt};

pub fn aes_decrypt(key: &[u8], iv: Option<&[u8]>, buffer: &[u8]) ->
        Result<Vec<u8>, error::ErrorStack>
{
	let decoded_buffer: Vec<u8>;
	let cipher = Cipher::aes_128_ecb();

	decoded_buffer = decrypt(cipher, key, iv, &buffer)?;

	Ok(decoded_buffer)
}
