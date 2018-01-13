use std::env;

fn usage() {
    println!("wrong number of arguments/hex string format !")
}

fn str_to_byte_array(s: &str) -> Option<Vec<u8>> {
    let mut vec: Vec<u8> = vec![];
    let mut str_iter = s.chars();

    let mut i = 0;
    while i < s.len() {
        let mut byte_str = String::with_capacity(2);
        byte_str.push(str_iter.next()?);
        byte_str.push(str_iter.next()?);

        vec.push(u8::from_str_radix(byte_str.as_str(), 16).unwrap());

        i += 2
    }

    Some(vec)
}

fn byte_to_base64_char(byte: u8) -> u8 {
    match byte {
        0 ... 0b011001 => 'A' as u8 + byte,
        0b011010 ... 0b110011 => 'a' as u8 + (byte - 0b011010),
        0b110100 ... 0b111101 => '0' as u8 + (byte - 0b110100),
        0b111110 => '+' as u8,
        _ => '/' as u8
    }
}

fn convert(hex64 : &[u8]) -> Vec<u8> {
    let mut base64 = vec![];

    let mut hex64_chunks = hex64.chunks(3);
    loop {
        let hex64_slice = match hex64_chunks.next() {
            Some(s) => s,
            None => break,
        };

        match hex64_slice.len() {
            3 => {
                let hex64_16bits = ((hex64_slice[0] as u32) << 16) & 0xff0000
                                   | ((hex64_slice[1] as u32) << 8) & 0x00ff00
                                   | (hex64_slice[2] as u32) & 0x0000ff;

                for j in 0..4 {
                    let c = byte_to_base64_char((hex64_16bits >> 18 - (j * 6) & 0x3f) as u8);
                    base64.push(c);
                }
            },
            2 => {
                let hex64_16bits = (((hex64_slice[0] as u32) << 8) & 0x00ff00
                                   | ((hex64_slice[1] as u32)) & 0x0000ff)
                                   << 2;

                for j in 0..3 {
                    let c = byte_to_base64_char((hex64_16bits >> 12 - (j * 6) & 0x3f) as u8);
                    base64.push(c);
                }
                base64.push('=' as u8);
            },
            1 => {
                let hex64_16bits = (((hex64_slice[0] as u32)) & 0x0000ff)
                                   << 4;

                for j in 0..2 {
                    let c = byte_to_base64_char((hex64_16bits >> 6 - (j * 6) & 0x3f) as u8);
                    base64.push(c);
                }
                base64.push('=' as u8);
                base64.push('=' as u8);
            },
            _ => panic!("this case shall not happened !"),
        }
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

    let base64 = convert(hex64_buf.as_slice());
    for c in base64 {
        print!("{}", c as char)
    }

    println!();
}
