use std::process::Command;

#[test]
fn run_challenge1()
{
	let output = Command::new("./target/debug/set1_challenge1")
	                     .arg("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t\n".as_bytes());
}
