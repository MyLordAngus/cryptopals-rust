
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

#[test]
fn run_challenge10()
{
	let output = Command::new("./target/debug/set2_challenge10")
	                     .output()
	                     .expect("failed to run integration test");

	let md5 = Command::new("/usr/bin/md5sum")
	                   .arg("/tmp/output.txt")
	                   .output()
	                   .expect("failed to run md5sum");
	assert_eq!(md5.stdout,
	           "9524d907f1ac0bcb0dc92604ebcc0fc8  /tmp/output.txt\n".as_bytes());
}
