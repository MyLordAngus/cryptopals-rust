use std::env;

fn usage() {
    println!("wrong number of arguments/hex string format !")
}

fn str_to_byte_array(s: &str) -> Option<Vec<u32>> {
    let mut vec: Vec<u32> = vec![];
    let mut str_iter = s.chars();

    let mut i = 0;
    while i < s.len() {
        let mut byte_str = String::with_capacity(2);
        //byte_str.push(str_iter.next().unwrap());
        byte_str.push(str_iter.next()?);
        byte_str.push(str_iter.next()?);

        vec.push(u32::from_str_radix(byte_str.as_str(), 16).unwrap());

        i += 2
    }

    Some(vec)
}

fn convert(hex64 : &[u32]) -> String {
    let mut base64 = String::new();

    let hex64_size = hex64.len();
    let mut hex64_iter = hex64.into_iter();
    let mut hex64_iter_array: [&u32; 3] = [&0; 3];

    let mut i = 0;
    while i < hex64_size {
        hex64_iter_array[0] = hex64_iter.next().unwrap();
        hex64_iter_array[1] = hex64_iter.next().unwrap();
        hex64_iter_array[2] = hex64_iter.next().unwrap();

        println!("{:x}:{:x}:{:x}", hex64_iter_array[0],
                                   hex64_iter_array[1],
                                   hex64_iter_array[2]);
        i += 3;
    }

    base64
}

fn main() {
    if env::args_os().count() != 2 {
        usage();
        std::process::exit(1);
    }

    let hex64_os_str = env::args_os().nth(1).unwrap();
    let hex64_str = hex64_os_str.to_str().unwrap();
    let hex64_buf = str_to_byte_array(hex64_str).unwrap_or_else(|| {
        usage();
        std::process::exit(1);
    });

    convert(hex64_buf.as_slice());
}
