use crate::*;
use std::collections::{VecDeque, HashMap};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

struct TestConfiguration {
    //A struct for keeping track of all the parameters we need for algorithmic testing
    attacker_eval: u8,
    defender_eval: u8,
    attacker_mo: u8,
    defender_mo: u8
}

struct TestData {
    avg_attack_time: u128,
    worst_attack_time: u128,
    avg_defend_time: u128,
    worst_defend_time: u128,
    victory: Option<VictoryCondition>,
    length: u32
}

pub fn play(board_size: u8, attacker: (String, bool), defender: (String, bool), evaluations: (u8,u8), move_orders: (u8,u8)) {
    //Manages the game logic of hnefatafl
    println!("Let's play Hnefatafl! {} must protect their king from {}.", defender.0, attacker.0);
    //Set board
    let mut instance: GameState = GameState::new(board_size);
    let mut defender_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut attacker_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    
    instance.show_board();
    loop{
        //Main Gameplay loop
        let turn_parity = instance.turn % 2 == 1;
        let active_player = if turn_parity {&attacker} else {&defender};
        let active_eval = if turn_parity {&evaluations.0} else {&evaluations.1};
        let move_order = if turn_parity {&move_orders.0} else {&move_orders.1};
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
                new_move = player::get_move(&instance, active_eval, player_history, *move_order);
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

pub fn algorithmic_trial_matches(trial_directory: &str, evaluations: (u8, u8), move_orders: (u8, u8), output_name: &str) {
    //Iterates over GameState files to hold matches between Algorithmic Players
    println!("Testing evaluations {} and {} over the directory: {}", evaluations.0, evaluations.1, trial_directory);
    let data_name = format!("../{}_test_result.txt", output_name);
    let file_path = PathBuf::from(trial_directory).join(data_name);
    let mut file = fs::File::create(file_path).expect("Error creating file for algorithmic trial match output.");
    let paths = fs::read_dir(trial_directory).unwrap();
    //TO BE CHANGED: in the future, change the TestConfiguration in between tests.
    let tc = TestConfiguration{attacker_eval: evaluations.0, defender_eval: evaluations.1, attacker_mo: move_orders.0, defender_mo: move_orders.1};
    for path in paths {
        if path.is_ok() {
            let trial = path.unwrap();
            println!("Running trial on: {:?}", trial.file_name());
            if let Ok(test_results) = trial_play(&trial.path(), &tc) {
                //Test concluded, write the returned data to our file
                let new_entry = format!("Game File:{:?}, Victory:{}, Length:{}, Avg Attack Time:{}, Slowest Attack Time:{}, Avg Defense Time:{}, Slowest Defense Time:{}\n", trial.file_name(), utility::store_vc(&test_results.victory), test_results.length, test_results.avg_attack_time, test_results.worst_attack_time, test_results.avg_defend_time, test_results.worst_defend_time);
                if file.write(new_entry.as_bytes()).is_ok() {
                    println!("Trial match successfully written to file.");
                }
            } else {
                //Test has returned an error, add some note to the Data File
                let new_entry = format!("%Test on {:?} returned an error.\n", trial.file_name());
                if file.write(new_entry.as_bytes()).is_ok() {
                    println!("Test ended in error. Warning written to file.");
                }
            }
        }
    }
}

fn trial_play(board_path: &PathBuf, tc: &TestConfiguration) -> Result<TestData, ()> {
    //Silent gameplay for testing algorithms
    let mut instance: GameState = utility::read_state_from_file(board_path).unwrap();
    let mut defender_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut attacker_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut avg_attack_time: u128 = 0;//All times are being measured in milliseconds
    let mut worst_attack_time: u128 = 0;
    let mut avg_defend_time: u128 = 0;
    let mut worst_defend_time: u128 = 0;
    println!("Trial on the following GameState:");
    instance.show_board();
    loop{
        //Main Gameplay loop
        if instance.turn >= 255 {
            //If a game goes on this long, there's probably a stalemate, exit without victory
            let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
            return Ok(test_results)
        }
        println!("Currently on turn {}", instance.turn);
        let turn_parity = instance.turn % 2 == 1;
        let player_history = if turn_parity {&attacker_states} else {&defender_states};
        let new_move: Option<MoveRequest>;
        if turn_parity {
            //Attacker Turn
            let start_time = Instant::now();
            new_move = player::get_move(&instance, &tc.attacker_eval, player_history, tc.attacker_mo);
            let total_time = start_time.elapsed().as_millis();
            assert!(new_move.is_some());
            let attacker_turn_no = ((instance.turn + 1)/2) as u128;
            avg_attack_time = ((avg_attack_time * (attacker_turn_no - 1)) + total_time) / attacker_turn_no;
            if total_time > worst_attack_time {worst_attack_time = total_time;}
        } else {
            //Defender Turn
            let start_time = Instant::now();
            new_move = player::get_move(&instance, &tc.defender_eval, player_history, tc.defender_mo);
            let total_time = start_time.elapsed().as_millis();
            assert!(new_move.is_some());
            let defender_turn_no = (instance.turn /2) as u128;
            avg_defend_time = ((avg_defend_time*(defender_turn_no - 1)) + total_time) / (defender_turn_no);
            if total_time > worst_defend_time {worst_defend_time = total_time;}
        }

        //Move received from player, attempting play
        let movement_result = instance.piece_move(new_move.unwrap());
        let movement: u8 = match movement_result.is_ok() {
            true => {
                movement_result.unwrap()
            }
            false => {
                //Algorithmic Player Generated False Move
                return Err(())
            }
        }; 

        //Movement succeeds, Checking for captures
        let captures: Vec<u8> = instance.capture_check(movement);
        //If the king has been captured, the game ends
        if instance.capture_pieces(captures).is_some() {
            instance.victory = Some(VictoryCondition::KingCaptured);
        }

        //If the king is in a corner, the game ends
        if instance.check_corners().is_some() {
            instance.victory = Some(VictoryCondition::KingInCorner);
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
        if instance.victory.is_some() {
            //Once the game is over, we need to send some data back to save
            let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
            return Ok(test_results)
        }
        instance.turn += 1;
    }
}