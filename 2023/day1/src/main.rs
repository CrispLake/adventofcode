use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn find_num(s: &String, rev: bool) -> usize {
    let mut sub;

    for (i, _) in s.chars().enumerate() {
        if rev
        {
            sub = &s[(s.len() - i - 1)..];
        }
        else
        {
            sub = &s[..i];
        }
        if sub.contains("0") {
            return 0;
        } else if sub.contains("1") {
            return 1
        } else if sub.contains("2") {
            return 2
        } else if sub.contains("3") {
            return 3
        } else if sub.contains("4") {
            return 4
        } else if sub.contains("5") {
            return 5
        } else if sub.contains("6") {
            return 6
        } else if sub.contains("7") {
            return 7
        } else if sub.contains("8") {
            return 8
        } else if sub.contains("9") {
            return 9
        } else if sub.contains("zero") {
            return 0
        } else if sub.contains("one") {
            return 1
        } else if sub.contains("two") {
            return 2
        } else if sub.contains("three") {
            return 3
        } else if sub.contains("four") {
            return 4
        } else if sub.contains("five") {
            return 5
        } else if sub.contains("six") {
            return 6
        } else if sub.contains("seven") {
            return 7
        } else if sub.contains("eight") {
            return 8
        } else if sub.contains("nine") {
            return 9
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
        println!("first num: {}, second num: {}", find_num(&buffer, false),find_num(&buffer, true));
        num += find_num(&buffer, false) * 10 + find_num(&buffer, true);
        buffer.clear();
    }
    println!("{num}");
    Ok(())
}

