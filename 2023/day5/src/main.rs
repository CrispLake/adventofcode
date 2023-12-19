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

fn find_next_map(lines: &Vec<String>, mut i: usize) -> usize {
    i += 1;
    while !lines[i].chars().nth(0).unwrap().is_alphabetic() {
        if i + 1 == lines.len() {
            return 0;
        }
        i += 1;
        if lines[i].is_empty() {
            i += 1;
        }
    }
    i
}

fn get_next_value(lines: &Vec<String>, mut index: usize, mut value: i64) -> i64 {
    index += 1;
    while index < lines.len() && !lines[index].is_empty() {
        let mut nums = lines[index].split_whitespace();
        let dest = nums.next().unwrap().parse::<i64>().unwrap();
        let source = nums.next().unwrap().parse::<i64>().unwrap();
        let range = nums.next().unwrap().parse::<i64>().unwrap();
        if value >= source && value < source + range {
            // println!("source: {}", source);
            // println!("dest: {}", dest);
            // println!("range: {}", range);
            value = value - source + dest;
            // println!("new: {}", value);
            break;
        }
        index += 1;
    }
    value
}

fn part_one(lines: &Vec<String>) -> Result<(), String> {
    // println!("{}", lines[0]);
    let seeds = &lines[0][lines[0].find(":").ok_or("Colon not found")? + 2..];
    let mut lowest: i64 = 0;
    for seed in seeds.split_whitespace() {
        // println!("Seed: {}", seed);
        let mut index = 2;
        let mut value: i64 = seed.parse::<i64>().unwrap();
        while index != 0 {
            // println!("Map: {}", index);
            value = get_next_value(&lines, index, value);
            index = find_next_map(&lines, index);
            if index == 0 {
                if lowest == 0 {
                    lowest = value;
                }
                if lowest > value {
                    lowest = value;
                }
            }
        }
    }
    println!("lowest location {}", lowest);
    Ok(())
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = lines_from_file("./input");
    let _ = part_one(&lines);
    // part_two(&mut lines);
    Ok(())
}
