use crate::*;
use std::collections::{VecDeque, HashMap};

pub fn play(board_size: u8, attacker: (String, bool), defender: (String, bool), evaluations: (u8,u8)) {
    //Manages the game logic of hnefatafl
    println!("Let's play Hnefatafl! {} must protect their king from {}.", defender.0, attacker.0);
    //Set board
    let mut instance: GameState = GameState::new(board_size);
    let mut defender_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut attacker_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    
    instance.show_board();
    if utility::save_state_to_file(&instance, "test_file.txt".to_string()).is_ok() {println!("Successfully saved file");}
    instance = utility::read_state_from_file(&"./test_file.txt".to_string()).unwrap();
    loop{
        //Main Gameplay loop
        let turn_parity = instance.turn % 2 == 1;
        let active_player = if turn_parity {&attacker} else {&defender};
        let active_eval = if turn_parity {&evaluations.0} else {&evaluations.1};
        let player_history = if turn_parity {&attacker_states} else {&defender_states};
        let mut new_move: Option<MoveRequest>;
        let movement: u8;
        loop{
            if active_player.1 {
                //Player is a human!
                println!("It's {}'s turn! Please enter your move.", active_player.0);
                let player_input = utility::get_player_move(instance.sizen);
                if player_input.is_none() {
                    println!("Movement request not received. Please re-enter:");
                    continue
                }
                new_move = player_input;
            } else {
                //Player is an algorithm!
                println!("{} is playing! Searching for move...", active_player.0);
                new_move = player::get_move(&instance, active_eval, player_history);
                assert!(new_move.is_some());
                let output_info = new_move.clone().unwrap();
                println!("{} is going to move the piece at {} {} by {}.", active_player.0, utility::to_coord(&output_info.position, &instance.sizen), utility::say_direction(&output_info.direction), output_info.magnitude);
            }

            //Move received from player, attempting play
            let movement_result = instance.piece_move(new_move.unwrap());
            match movement_result.is_ok() {
                true => {
                    movement = movement_result.unwrap();
                    break
                }
                false => {
                    if active_player.1 {//Human Player handling for invalid moves
                        println!("That move isn't valid! Enter another move:");
                        continue
                    } else {
                        panic!("Algorithm has submitted an invalid move. Fatal Error.");
                        }
                }
            } 
        }

        //Movement succeeds, Checking for captures
        let captures: Vec<u8> = instance.capture_check(movement);
        //If the king has been captured, the game ends
        if instance.capture_pieces(captures).is_some() {
            instance.end_game();
            break
        }

        //If the king is in a corner, the game ends
        if instance.check_corners().is_some() {
            instance.victory = Some(VictoryCondition::KingInCorner);
            instance.end_game();
            break
        }
        if instance.turn < 21 {
            //Append board to history
            if instance.turn % 2 == 1 {
                attacker_states.push_back(instance.board.clone());
            } else {
                defender_states.push_back(instance.board.clone());
            }
        } else {
            //Remove oldest board, and update history
            if instance.turn % 2 == 1{
                attacker_states.push_back(instance.board.clone());
                assert!(attacker_states.pop_front().is_some());
                assert!(attacker_states.len() < 11);
            } else {
                defender_states.push_back(instance.board.clone());
                assert!(defender_states.pop_front().is_some());
                assert!(defender_states.len() < 11);
            }
        }
        instance.show_board();
        instance.turn += 1;
    }
}