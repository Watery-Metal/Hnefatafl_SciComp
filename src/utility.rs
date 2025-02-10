//Utilities for interfacing with human players
use std::io;
use std::fmt::Display;
use std::path::PathBuf;
use std::{fs, io::Write, error::Error};
use std::collections::HashMap;
use crate::{MoveRequest, Direction, GameState, VictoryCondition, Piece};

pub fn get_player_move(board_size: u8) -> Option<MoveRequest> {
    //Accept movement argument from the command line.
    let mut request = String::new();
    io::stdin().read_line(&mut request).expect("Failed to receive player input during get_player_move function.");
    //add confirmation handling here
    let components: Vec<&str> = request.split_whitespace().collect();
    
    if components.len() != 3 {
        println!("Invalid input! Please enter board position, direction, and distance of desired move.");
        return None
    }

    let component_test: Vec<char> = components[0].chars().collect();//separate line; compiler fighting
    if component_test.len() > 3 {println!("Debugging Print: Piece argument too large"); return None}

    let component_test: Vec<char> = components[1].chars().collect();
    if component_test.len() > 1 {println!("Debugging Print: Direction argument too large"); return None}

    let component_test: Vec<char> = components[2].chars().collect();
    if component_test.len() > 2 {println!("Debugging Print: Distance argument too large"); return None}
    
    let direction: Direction;
        match components[1].to_uppercase().chars().next() {//if type error, there's also .to_uppercase() for String
            Some('R') => {direction = Direction::R;}
            Some('U') => {direction = Direction::U;}
            Some('D') => {direction = Direction::D;}
            Some('L') => {direction = Direction::L;}
            _ => {println!("Debugging Print: Direction provided not understood."); return None}
        }
    
    let magnitude: u8;
    let mag_parse = components[2].parse::<u8>();
    if mag_parse.is_err() {println!("Debugging Print: Couldn't parse move magnitude as u8"); return None} else {magnitude = mag_parse.unwrap();}

    let position: u8;
    let position_option = to_index(components[0], board_size);
    match position_option {
        None => {println!("Debugging Print: to_index function didn't return a board position"); return None}
        _ => {position = position_option.unwrap();}
    }
    Some(MoveRequest{position, direction, magnitude})
}

pub fn to_index(x: &str, sizen: u8) -> Option<u8> {
    //Mapping from human-legible coord position to absolute board index
    let letter = x[0..1].to_uppercase().chars().next();//How to slice in rust? Also, needs to be char
    if letter.is_none() {println!("Debugging Print: to_index funtion could not find letter component of provided command."); return None}
    let letnum: u8;
    let letnum_op = c2n(letter.unwrap());
    if letnum_op.is_none() {
        println!("Debugging Print: c2n Function did not find appropriate letter, to_index funtion ceasing.");
        return None} else {letnum = letnum_op.unwrap();}

    let numbs_op = x[1..].parse();
    let numbs: u8;
    if numbs_op.is_err() {
        println!("Debugging Print: to_index function could not find number component.");
        return None
    } else {numbs = numbs_op.unwrap();}

    Some((sizen * (numbs - 1)) + letnum)
}

pub fn to_coord(board_location: &u8, board_size: &u8) -> String {
    //Mapping from absolute board index to human-legible coord
    let row = board_location / board_size;
    let col = n2c(board_location % board_size).unwrap();
    format!("{}{}", row+1, col)
}

pub fn confirm<T: Display>(input: &T) -> bool {
    let input_string = format!("{}", input);
    println!("Is \"{}\" correct? (Y/N)", input_string.trim_end());
    loop{
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        let user_answer: char = match response.trim().parse() {
            Ok(good) => good,
            Err(_) => {println!("Invalid. Please enter Y or N."); continue},
        };
        
        match user_answer {
            'Y' => return true,
            'y' => return true,
            'N' => return false,
            'n' => return false,
            _ => {println!("I didn't understand. Please confirm (Y), or deny (N)."); continue}
        }
    }
}

pub fn confirm_blank() -> bool {
    //Gets a bool from a player confirmation
    loop{
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        let user_answer: char = match response.trim().parse() {
            Ok(good) => good,
            Err(_) => {println!("Invalid. Please enter Y or N."); continue},
        };
        
        match user_answer {
            'Y' => return true,
            'y' => return true,
            'N' => return false,
            'n' => return false,
            _ => {println!("I didn't understand. Please confirm (Y), or deny (N)."); continue}
        }
    }
}

pub fn c2n(input: char) -> Option<u8> {
    //Pseudo-Hashmap for board indicies
    match input {
        'A' => {Some(0)},
        'B' => {Some(1)},
        'C' => {Some(2)},
        'D' => {Some(3)},
        'E' => {Some(4)},
        'F' => {Some(5)},
        'G' => {Some(6)},
        'H' => {Some(7)},
        'I' => {Some(8)},
        'J' => {Some(9)},
        'K' => {Some(10)},
        'L' => {Some(11)},
        'M' => {Some(12)},
        _ => {None},
    }
}

pub fn n2c(input: u8) -> Option<char> {
    match input {
        0 => {Some('A')},
        1 => {Some('B')},
        2 => {Some('C')},
        3 => {Some('D')},
        4 => {Some('E')},
        5 => {Some('F')},
        6 => {Some('G')},
        7 => {Some('H')},
        8 => {Some('I')},
        9 => {Some('J')},
        10 => {Some('K')},
        11 => {Some('L')},
        12 => {Some('M')},
        _ => {None},
    }
}
pub fn get_board_size() -> u8 {
    let board_size: u8;
    loop {
        let mut request = String::new();
        io::stdin().read_line(&mut request)
            .expect("Failed to read line.");

        let input_size: u8 = match request.trim().parse() {
            Ok(good) => good,
            Err(_) => {println!("Invalid. Please enter a u8"); continue},
        };

        match input_size % 2 {
            1 => {
                board_size = input_size;
                println!("Great! The board will be {}x{}", board_size, board_size);
                break
            }
            _ =>{
                println!("Invalid board size (for now). Please enter an odd number.");
                continue
            }
        }
    }
    board_size
}

pub fn get_name(role: String) -> String {
    println!("{}:", role);
    let player_name: String;
    loop{
        let mut request = String::new();
        io::stdin().read_line(&mut request)
            .expect("Failed to read line.");
        let success = confirm(&request);
        if success {
            player_name = request.to_string().trim_end().to_string();
            break
        } else {
            println!("{} Name:", role);
            continue
        }
    }
    player_name
}

pub fn get_no(prompt: String) -> u8 {
    println!("{}", prompt);
    let player_name: u8;
    loop{
        let mut request = String::new();
        io::stdin().read_line(&mut request)
            .expect("Failed to read line.");
        let success = confirm(&request);
        if success {
            let trial = request.to_string().trim_end().parse::<u8>();
            match  trial {
                Ok(value) => {player_name = value; break}
                Err(_) => {println!("Number couldn't be understood. Please try again:"); continue}
            }
        } else {
            println!("Enter another number:");
            continue
        }
    }
    player_name
}

pub fn get_no_16(prompt: String) -> u16 {
    println!("{}", prompt);
    let player_name: u16;
    loop{
        let mut request = String::new();
        io::stdin().read_line(&mut request)
            .expect("Failed to read line.");
        let success = confirm(&request);
        if success {
            let trial = request.to_string().trim_end().parse::<u16>();
            match  trial {
                Ok(value) => {player_name = value; break}
                Err(_) => {println!("Number couldn't be understood. Please try again:"); continue}
            }
        } else {
            println!("Enter another number:");
            continue
        }
    }
    player_name
}


pub fn say_direction(input: &Direction) -> String {
    match input {
        Direction::U => {"up".to_string()}
        Direction::D => {"down".to_string()}
        Direction::L => {"left".to_string()}
        Direction::R => {"right".to_string()}
    }
}

pub fn store_vc(victory: &Option<VictoryCondition>) -> String {
    match victory {
        None => {"N".to_string()}
        &Some(VictoryCondition::KingInCorner) => {"K".to_string()}
        &Some(VictoryCondition::KingCaptured) => {"C".to_string()}
        &Some(VictoryCondition::AttackerExtinction) => {"A".to_string()}
        &Some(VictoryCondition::DefenderExtinction) => {"D".to_string()}
    }
}

fn store_piece(piece: &Piece) -> String {
    match *piece {
        Piece::Attacker => {"A".to_string()}
        Piece::Defender => {"D".to_string()}
        Piece::King => {"K".to_string()}
    }
}

fn read_piece(info: &str) -> Piece {
    match info {
        "A" => {Piece::Attacker}
        "D" => {Piece::Defender}
        "K" => {Piece::King}
        &_ => {panic!("Strings not sliced properly; attempted to read an invalid piece.");}
    }
}

fn read_victory(info: &str) -> Option<VictoryCondition> {
    match info {
        "N" => {None}
        "K" => {Some(VictoryCondition::KingInCorner)}
        "C" => {Some(VictoryCondition::KingCaptured)}
        "A" => {Some(VictoryCondition::AttackerExtinction)}
        "D" => {Some(VictoryCondition::DefenderExtinction)}
        &_ => {None}
    }
}

pub fn save_state_to_file(game: &GameState, game_id: String) -> Result<(), Box<dyn Error>> {
    //Converts a GameState to a legible and loadable test file
    let mut file = fs::File::create(game_id)?;
    let header = format!("{} {} {}\n", game.sizen, game.turn, store_vc(&game.victory));
    file.write_all(header.as_bytes())?;

    for i in 0..(game.sizen * game.sizen) {
        let candidate = game.board.get(&i);
        if candidate.is_none() {continue}
        let entry = format!("{} {}\n", i, store_piece(candidate.unwrap()));
        file.write_all(entry.as_bytes())?;
    }
    Ok(())
}

pub fn read_state_from_file(file_path: &PathBuf) -> Option<GameState> {
    //Attempts to read a GameState from a file, returning None if unsuccessful
    let file_content = fs::read_to_string(file_path).expect("No File Present");
    let mut game_data = file_content.lines().collect::<Vec<&str>>();
    let header = game_data[0];
    game_data.drain(..1);

    let additional_info: Vec<&str> = header.split_whitespace().collect();
    let sizen: u8 = additional_info[0].parse().expect("Couldn't parse sizen in header");
    let turn: u32 = additional_info[1].parse().expect("Couldn't parse turn in header");
    let victory: Option<VictoryCondition> = read_victory(additional_info[2]);
    
    let mut board: HashMap<u8, Piece> = HashMap::new();

    let corners: Vec<u8> = vec![0, sizen - 1, (sizen * sizen) - 1, (sizen * sizen) - sizen];
    let throne: u8 = ((sizen * sizen) - 1) / 2;

    for line in game_data {
        let split: Vec<&str> = line.split_whitespace().collect();
        assert!(split.len() == 2);
        let location: u8 = split[0].parse().expect("Couldn't parse location in gamestate data");
        let piece: Piece = read_piece(split[1]);
        board.insert(location, piece);
    }
    Some(GameState{sizen, turn, victory, board, corners, throne})
}

pub fn write_test_results() {

}