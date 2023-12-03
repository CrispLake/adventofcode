use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn second_num(s: &String) -> usize {
    let bytes = s.as_bytes();

    let mut x = 0;
    for (i, &item) in bytes.iter().enumerate() {
        match item {
            b'0' => x = 0,
            b'1' => x = 1,
            b'2' => x = 2,
            b'3' => x = 3,
            b'4' => x = 4,
            b'5' => x = 5,
            b'6' => x = 6,
            b'7' => x = 7,
            b'8' => x = 8,
            b'9' => x = 9,
            _ => continue,
        }
    }
    return x;
}

fn first_num(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        match item {
            b'0' => return 0,
            b'1' => return 1,
            b'2' => return 2,
            b'3' => return 3,
            b'4' => return 4,
            b'5' => return 5,
            b'6' => return 6,
            b'7' => return 7,
            b'8' => return 8,
            b'9' => return 9,
            _ => continue,
        }
    }
    return 0;
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut num = 0;

    loop {
        reader.read_line(&mut buffer)?;
        if buffer.len() == 0
        {
            break;
        }
        num += first_num(&buffer) * 10 + second_num(&buffer);
        buffer.clear();
    }
    println!("{num}");
    Ok(())
}

