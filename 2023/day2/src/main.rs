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
    if space != 0 && colon != 0
    {
        let game_id = &buffer[space + 1..colon];
        return game_id.parse::<i32>().unwrap();
    }
    return -1;
}

fn is_valid_game(buffer: &String) -> bool {
    let colon;

    match buffer.find(':') {
        Some(index) => colon = index,
        None => return false,
    }
    let parsed = &buffer[colon + 2..];
    false
}

fn main() -> io::Result<()> {
    // let max_red = 12;
    // let max_green = 13;
    // let max_blue = 14;

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut game_id: i32;
    let mut total: i32 = 0;
    loop {
        reader.read_line(&mut buffer)?;
        if buffer.len() == 0
        {
            break;
        }
        game_id = get_game_id(&buffer);
        assert!(game_id != -1);
        if is_valid_game(&buffer)
        {
            total += game_id;
        }
        buffer.clear();
        // println!("{}", game_id);
    }
    Ok(())
}
