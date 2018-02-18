
use std::process::Command;

#[test]
fn run_challenge9()
{
	let output = Command::new("./target/debug/set2_challenge9")
	                     .arg("YELLOW SUBMARINE")
	                     .arg("4")
	                     .output()
	                     .expect("failed to run integration test");

	assert_eq!(output.stdout,
	           "YELLOW SUBMARINE\\x04\\x04\\x04\\x04\n".as_bytes());
}
