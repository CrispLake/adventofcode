use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn get_num_from_pos(line: &String, mut index: usize) -> i32 {
    while index > 0 && line.as_bytes()[index].is_ascii_digit() {
        index -= 1;
    }
    if !line.as_bytes()[index].is_ascii_digit() {
        index += 1;
    }
    let num_start = index;
    while index != line.len() && line.as_bytes()[index].is_ascii_digit() {
        index += 1;
    }
    let num: i32 = line[num_start..index].parse::<i32>().unwrap();
    // println!("{}", num);
    num
}

fn find_nums(line: &String, i: usize) -> i32 {
    let mut total: i32 = 0;
    if !line.is_empty() {
        if i > 0 && line.as_bytes()[i - 1].is_ascii_digit() {
            total += get_num_from_pos(line, i - 1);
        }
        else if line.as_bytes()[i].is_ascii_digit() {
            total += get_num_from_pos(line, i);
        }
        if !line.as_bytes()[i].is_ascii_digit() && line.as_bytes()[i + 1].is_ascii_digit() {
            total += get_num_from_pos(line, i + 1);
        }
    }
    total
}

fn get_total_from_line(current_line: &String, prev_line: &String, next_line: &String) -> i32 {
    let mut total: i32 = 0;

    // println!("{}", prev_line);
    // println!("{}", current_line);
    // println!("{}", next_line);
    for (i, c) in current_line.chars().enumerate() {
        if c != '.' && !c.is_whitespace() && !c.is_ascii_digit() {
            total += find_nums(&prev_line, i);
            total += find_nums(&current_line, i);
            total += find_nums(&next_line, i);
        }
    }
    total
}

fn multiply_nums(line: &String, i: usize) -> i32 {
    let mut total: i32 = 0;
    if !line.is_empty() {
        total = 1;
        let mut num;
        if i > 0 && line.as_bytes()[i - 1].is_ascii_digit() {
            num = get_num_from_pos(line, i - 1);
            if num != 0 {
                println!("{}", num);
                total *= num;
            }
        }
        else if line.as_bytes()[i].is_ascii_digit() {
            num = get_num_from_pos(line, i);
            if num != 0 {
                println!("{}", num);
                total *= num;
            }
        }
        if !line.as_bytes()[i].is_ascii_digit() && line.as_bytes()[i + 1].is_ascii_digit() {
            num = get_num_from_pos(line, i + 1);
            if num != 0 {
                println!("{}", num);
                total *= num;
            }
        }
    }
    total
}

fn count_nums(line: &String, i: usize) -> i32 {
    let mut total: i32 = 0;
    if !line.is_empty() {
        if i > 0 && line.as_bytes()[i - 1].is_ascii_digit() {
            total += 1;
        }
        else if line.as_bytes()[i].is_ascii_digit() {
            total += 1;
        }
        if !line.as_bytes()[i].is_ascii_digit() && line.as_bytes()[i + 1].is_ascii_digit() {
            total += 1;
        }
    }
    total
}

fn get_ratio_from_line(current_line: &String, prev_line: &String, next_line: &String) -> i32 {
    let mut count: i32 = 0;
    let mut total: i32 = 0;
    for (i, c) in current_line.chars().enumerate() {
        if c == '*' {
            count += count_nums(&prev_line, i);
            count += count_nums(&current_line, i);
            count += count_nums(&next_line, i);
            // println!("count: {}", count);
            let mut tmp = 0;
            if count == 2 {
                println!("{}", prev_line);
                println!("{}", current_line);
                println!("{}", next_line);
                tmp = 1;
                let mut num = multiply_nums(&prev_line, i);
                if num != 0 {
                    // println!("{}", num);
                    tmp *= num;
                }
                num = multiply_nums(&current_line, i);
                if num != 0 {
                    // println!("{}", num);
                    tmp *= num;
                }
                num = multiply_nums(&next_line, i);
                if num != 0 {
                    // println!("{}", num);
                    tmp *= num;
                }
            }
            count = 0;
            total += tmp;
        }
    }
    // println!("ratio: {}", total);
    total
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = lines_from_file("./input");
    let mut prev_line: String = String::new();
    let mut current_line: String = String::new();
    let mut total: i32 = 0;
    let mut ratio: i32 = 0;
    for next_line in lines {
        if !current_line.is_empty() {
            total += get_total_from_line(&current_line, &prev_line, &next_line);
            ratio += get_ratio_from_line(&current_line, &prev_line, &next_line);
        }
        prev_line = current_line;
        current_line = next_line;
    }
    let next_line: String = String::new();
    total += get_total_from_line(&current_line, &prev_line, &next_line);
    ratio += get_ratio_from_line(&current_line, &prev_line, &next_line);
    println!("{}", total);
    println!("{}", ratio);
    Ok(())
}