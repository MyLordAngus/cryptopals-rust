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

#[test]
fn run_challenge2()
{
	let output = Command::new("./target/debug/set1_challenge2")
	                     .arg("1c0111001f010100061a024b53535009181c")
	                     .arg("686974207468652062756c6c277320657965")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "746865206b696420646f6e277420706c6179\n".as_bytes());
}

#[test]
fn run_challenge3()
{
	let output = Command::new("./target/debug/set1_challenge3")
	                     .arg("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "Cooking MC's like a pound of bacon\n".as_bytes());
}

#[test]
fn run_challenge4()
{
	let output = Command::new("./target/debug/set1_challenge4")
	                     .arg("assets/set1_challenge4.txt")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "Now that the party is jumping\n\n".as_bytes());
}

#[test]
fn run_challenge5()
{
	let output = Command::new("./target/debug/set1_challenge5")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20690a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f\n".as_bytes());
}

#[test]
fn run_challenge6()
{
	let output = Command::new("./target/debug/set1_challenge6")
	                     .arg("assets/set1_challenge6.txt")
	                     .output()
	                     .expect("failed to run integration test");

	let text = "Best keysize = 29\nTerminator X: Bring the noise\nI'm back and I'm ringin' the bell \nA rockin' \n";
	assert_eq!(output.stdout, text.as_bytes());
}
