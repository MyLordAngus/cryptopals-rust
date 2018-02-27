use std::fs::File;
use std::io;
use std::io::prelude::*;

use base64;

pub fn write_base64_fmt_file(buf: &[u8], filename: &str) -> io::Result<()>
{
	let base64_buf = base64::base64_encode(buf);
	let mut file = File::create(filename)?;

	let chunks = base64_buf.chunks(60);
	for chunk in chunks {
		file.write_all(chunk)?;
		writeln!(&mut file)?;
	}

	Ok(())
}
