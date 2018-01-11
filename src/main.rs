use std::env;

fn usage() {
    println!("wrong number of arguments !")
}

fn convert(hex64 : &str) -> String {
    let mut base64 = String::new();

    base64
}

fn main() {
    if env::args_os().count() != 2 {
        usage();
        std::process::exit(1);
    }

    let hex64_os_str = env::args_os().nth(1).unwrap();
    let hex64_str = hex64_os_str.to_str().unwrap();
}
