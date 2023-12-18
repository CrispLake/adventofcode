use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn get_game_id(buffer: &String) -> i32 {
    let space;
    let colon;

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

fn get_power(buffer: &String) -> i32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let colon;
    let mut reds;
    let mut greens;
    let mut blues;

    match buffer.find(':') {
        Some(index) => colon = index,
        None => return 0,
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
            // println!("{}", cube_parsed);
            let space;
            match cube_parsed.find(' ') {
                Some(index) => space = index,
                None => return 0,
            }
            let cube_count = cube_parsed[..space].parse::<i32>().unwrap();
            match cube_parsed.find('\n') {
                Some(index) => cube_parsed = &cube_parsed[..index],
                None => (),
            }
            // println!("|{}|", &cube_parsed[space + 1..]);
            match &cube_parsed[space + 1..] {
                "red" => reds += cube_count,
                "green" => greens += cube_count,
                "blue" => blues += cube_count,
                _ => (),
            }
        }
        if reds > max_red {max_red = reds}
        if greens > max_green {max_green = greens}
        if blues > max_blue {max_blue = blues}
    }
    max_red * max_green * max_blue
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
            // println!("{}", cube_parsed);
            let space;
            match cube_parsed.find(' ') {
                Some(index) => space = index,
                None => return false,
            }
            let cube_count = cube_parsed[..space].parse::<i32>().unwrap();
            match cube_parsed.find('\n') {
                Some(index) => cube_parsed = &cube_parsed[..index],
                None => (),
            }
            println!("|{}|", &cube_parsed[space + 1..]);
            match &cube_parsed[space + 1..] {
                "red" => reds += cube_count,
                "green" => greens += cube_count,
                "blue" => blues += cube_count,
                _ => (),
            }
        }
        if reds > max_red || greens > max_green || blues > max_blue {
            return false;
        }
        // println!("{}, {}, {}", reds, greens, blues);
    }
    true
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut game_id: i32;
    let mut total: i32 = 0;
    let mut total_power: i32 = 0;
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
        total_power += get_power(&buffer);
        buffer.clear();
        // println!("{}", game_id);
    }
    println!("{}", total);
    println!("{}", total_power);
    Ok(())
}
