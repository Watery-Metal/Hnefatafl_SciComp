//Matthew Passage , Nikolas Weber
use std::collections::HashMap;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use hnefatafl_prototype::*;
use std::time::Instant;
use hnefatafl_prototype::player::get_move;
use std::collections::VecDeque;
use std::cmp::max;


#[derive(Serialize, Deserialize, Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Serialize, Deserialize, Clone)]
struct ServerMessage {
    message: Option<String>,
    role: Option<String>,
    board_state: Option<BoardState>,
    current_turn: Option<String>,
    winner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BoardState {
    board: HashMap<String, String>,
}

const EVAL: u8 = 99999;
const HISTORY: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
const MOVE_ORDER: u8 = 0;
const A_B_DEPTH: u8 = 3;
const TIME_CAP: u8 = 10;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server");

    let mut buffer = [0; 4096];
    let n = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);

    let mut server_message: ServerMessage = ServerMessage {
        message: None,
        role: None,
        board_state: None,
        current_turn: None,
        winner: None,
    };

    for part in response.split("}{") {
        let json_str = if part.starts_with('{') && part.ends_with('}') {
            part.to_string()
        } else if part.starts_with('{') {
            format!("{}{}", part, "}")
        } else {
            format!("{}{}", "{", part)
        };

        match serde_json::from_str(&json_str) {
            Ok(msg) => server_message = msg,
            Err(e) => {
                eprintln!("Failed to parse server message: {}", e);
                return Ok(());
            }
        }
    }

    let mut player_role = String::new();

    if let Some(message) = server_message.message {
        if message == "Game has started" {
            if let Some(role) = server_message.role {
                player_role = role.clone();
                if role == "Attacker" {
                    if let Some(board_state) = server_message.board_state.clone() {
                        send_move(&mut stream, &board_state.board, &role)?;
                    }
                } else {
                    println!("Waiting for the opponent's move...");
                }
            }
        }
    }

    loop {
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        for part in response.split("}{") {
            let json_str = if part.starts_with('{') && part.ends_with('}') {
                part.to_string()
            } else if part.starts_with('{') {
                format!("{}{}", part, "}")
            } else {
                format!("{}{}", "{", part)
            };

            match serde_json::from_str(&json_str) {
                Ok(msg) => server_message = msg,
                Err(e) => {
                    eprintln!("Failed to parse server message: {}", e);
                    continue;
                }
            }
        }

        if let Some(winner) = server_message.winner.clone() {
            println!("Game over! The winner is: {}", winner);
        }

        if let Some(board_state) = server_message.board_state.clone() {
            println!("Board state: {:?}", board_state.board);
            println!("Board state: {:?}", &player_role);//REMOVE LATER
        }

        if let Some(current_turn) = server_message.current_turn.clone() {
            if current_turn == player_role && server_message.winner.is_none() {
                if let Some(ref board_state) = server_message.board_state {
                    send_move(&mut stream, &board_state.board, &player_role)?;
                } else {
                    println!("No board state available for the current turn.");
                }
            } else {
                println!("Waiting for the opponent's move...");
            }
        }
    }
}

fn send_move(stream: &mut TcpStream, board: &HashMap<String, String>, role: &str) -> io::Result<()> {
    let mut turn : u32 = 1;
    if role == "Defender"{
        turn = 2;
    }
    let mut sizen: u8 = 0;
    let mut board_11: HashMap<u8, Piece> = HashMap::new();
    let mut board_7: HashMap<u8, Piece> = HashMap::new();

    for (pos,piece) in board{
        let coords: Vec<u8> = pos[1..pos.len()-1].split(", ").map(|s| s.parse().unwrap()).collect();
        if piece == "Attacker"{
            board_11.insert(coords[0]+coords[1]*11, Piece::Attacker);
            board_7.insert(coords[0]+coords[1]*7, Piece::Attacker);
        }else if piece == "Defender"{
            board_11.insert(coords[0]+coords[1]*11, Piece::Defender);
            board_7.insert(coords[0]+coords[1]*7, Piece::Defender);
        }else if piece == "King"{
            board_11.insert(coords[0]+coords[1]*11, Piece::King);
            board_7.insert(coords[0]+coords[1]*7, Piece::King);
        }
        sizen = max(sizen, max(coords[0],coords[1]));
    }
    sizen += 1;

    let board_new: HashMap<u8, Piece>;
    if sizen == 7{
        board_new = board_7;
    }else{
        board_new = board_11;
    }

    let corners: Vec<u8> = vec![0, sizen - 1, (sizen * sizen) - 1, (sizen * sizen) - sizen];
    let throne: u8 = ((sizen * sizen) - 1) / 2;
    let victory = None;
    let state = GameState{turn, board: board_new, sizen, corners, throne, victory};

    println!("yes");
    let new_move = get_move(&state, &EVAL, &HISTORY, MOVE_ORDER, A_B_DEPTH, &TIME_CAP, Instant::now()).clone().unwrap();
    println!("yes");
    let curr_pos_x = new_move.position % sizen;
    let curr_pos_y = new_move.position / sizen;
    let new_pos_x;
    let new_pos_y;
    match new_move.direction{
        Direction::U => {
            new_pos_x = curr_pos_x;
            new_pos_y = curr_pos_y - new_move.magnitude;
        }
        Direction::D => {
            new_pos_x = curr_pos_x;
            new_pos_y = curr_pos_y + new_move.magnitude;
        }
        Direction::R => {
            new_pos_x = curr_pos_x + new_move.magnitude;
            new_pos_y = curr_pos_y;
        }
        Direction::L => {
            new_pos_x = curr_pos_x - new_move.magnitude;
            new_pos_y = curr_pos_y;
        }
    }

    // Send the move to the server
    let game_move = Move {
        from: (curr_pos_x as usize,curr_pos_y as usize),
        to: (new_pos_x as usize,new_pos_y as usize),
    };

    let serialized_move = serde_json::to_string(&game_move).unwrap();
    stream.write_all(serialized_move.as_bytes())?;
    println!("Move sent to the server: {:?}", game_move);
    return Ok(());
}
