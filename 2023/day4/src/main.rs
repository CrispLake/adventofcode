use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_nums(slice: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    // println!("{}", slice);
    for num in slice.split(' ') {
        if !num.is_empty() {
            // println!("{}", num);
            vec.push(num.to_string());
        }
    }
    vec
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = lines_from_file("./input");
    let mut total = 0;
    for line in lines {
        let mut card_total = 0;
        println!("{}", line);
        let colon;
        let pipe;
        match line.find(':') {
            Some(index) => colon = index + 2,
            None => return Ok(()),
        }
        match line.find('|') {
            Some(index) => pipe = index + 2,
            None => return Ok(()),
        }
        let win_nums: Vec<String> = get_nums(&line[colon..pipe - 2]);
        let my_nums: Vec<String> = get_nums(&line[pipe..]);
        for num in my_nums {
            if win_nums.contains(&num) {
                println!("contains: {}", num);
                card_total *= 2;
                if card_total == 0 {
                    card_total = 1;
                }
            }
        }
        total += card_total;
    }
    println!("{}", total);
    Ok(())
}