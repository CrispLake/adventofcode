use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn get_game_id(buffer: &String) -> i32 {
    let mut space = 0;
    let mut colon = 0;

    match buffer.find(' ') {
        Some(index) => space = index,
        None => return -1,
    }
    match buffer.find(':') {
        Some(index) => colon = index,
        None => return -1,
    }
    if space != 0 && colon != 0 {
        let game_id = &buffer[space + 1..colon];
        return game_id.parse::<i32>().unwrap();
    }
    return -1;
}

fn is_valid_game(buffer: &String) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let colon;
    let mut reds;
    let mut greens;
    let mut blues;

    match buffer.find(':') {
        Some(index) => colon = index,
        None => return false,
    }
    let parsed = &buffer[colon + 2..];
    let parts = parsed.split(';');
    // println!("{}", parsed);
    for part in parts {
        reds = 0;
        greens = 0;
        blues = 0;
        // println!("{}", part);
        let cubes = part.split(',');
        for cube in cubes {
            let mut cube_parsed = &cube[..];
            if cube.chars().nth(0).unwrap().is_whitespace() {
                cube_parsed = &cube[1..];
            }
            println!("{}", cube_parsed);
            match buffer.find(':') {
                Some(index) => colon = index,
                None => return false,
            }
            let cube_count = cube_parsed.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            println!("{}", cube_count);
            match cube_parsed[2..].to_string() {
                f if f == "red".to_string() => reds += cube_count,
                f if f == "green".to_string() => greens += cube_count,
                f if f == "blue".to_string() => blues += cube_count,
                _ => (),
            }
        }
        println!("{}, {}, {}", reds, greens, blues);
        if reds > max_red || greens > max_green || blues > max_blue {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let f = File::open("testinput")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut game_id: i32;
    let mut total: i32 = 0;
    loop {
        reader.read_line(&mut buffer)?;
        if buffer.len() == 0 {
            break;
        }
        game_id = get_game_id(&buffer);
        assert!(game_id != -1);
        if is_valid_game(&buffer) {
            total += game_id;
        }
        buffer.clear();
        // println!("{}", game_id);
    }
    println!("{}", total);
    Ok(())
}
