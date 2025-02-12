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
    defender_mo: u8,
    a_b_depth: u8
}

struct TestConfigurationAdj {
    //A struct for keeping track of all the parameters we need for algorithmic testing
    attacker_eval: u16,
    defender_eval: u16,
    attacker_mo: u8,
    defender_mo: u8,
    a_b_depth: u8
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
    let a_b_depth: u8 = 3;//Algorithmic default depth
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
                new_move = player::get_move(&instance, active_eval, player_history, *move_order, a_b_depth, &120, Instant::now());
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

pub fn algorithmic_trial_matches(trial_directory: &str, evaluations: u8, move_orders: u8, output_name: &str, a_b_depth: u8, time_cap:u8) {
    //Iterates over GameState files to hold matches between Algorithmic Players
    println!("Testing evaluations up to {} over the directory: {}", evaluations, trial_directory);
    let data_name = format!("../{}_test_result.txt", output_name);
    let file_path = PathBuf::from(trial_directory).join(data_name);
    let mut file = fs::File::create(file_path).expect("Error creating file for algorithmic trial match output.");
    let paths = fs::read_dir(trial_directory).unwrap();

    let data_header = "Game File, Search Depth, Victory, Length, Attacker Eval, Attacker Mord, Avg Attack Time, Slowest Attack Time, Defender Eval, Defender Mord, Avg Defense Time, Slowest Defense Time\n".to_string();
        if file.write(data_header.as_bytes()).is_ok() {
            println!("CSV successfully written to file.");
        }

    println!("Setting up test parameters...");
    //Generate Test Configurations:
    let mut eval_pairs: Vec<(u8,u8)> = Vec::new();
    for i in 0..=evaluations{
        for j in i..=evaluations {
            eval_pairs.push((i,j));
            if i != j {eval_pairs.push((j,i));}
        }
    }

    let mut mord_pairs: Vec<(u8,u8)> = Vec::new();
    for i in 0..=move_orders{
        for j in i..=move_orders{
            mord_pairs.push((i,j));
            if i != j {mord_pairs.push((j,i));}
        }
    }

    //Begin iterating game boards with various configurations
    for path in paths {
        if path.is_ok() {
            let trial = path.unwrap();
            println!("Running trial on: {:?}", trial.file_name());
            //Permute Algorithms, and eval heuristics
            for depth in 1..=a_b_depth {
                for m_ord_pair in &mord_pairs {
                    for algorithmic_pair in &eval_pairs{
                        let tc = TestConfiguration{attacker_eval: algorithmic_pair.0, defender_eval:algorithmic_pair.1, attacker_mo: m_ord_pair.0, defender_mo: m_ord_pair.1, a_b_depth:depth};
                        if let Ok(test_results) = trial_play(&trial.path(), &tc, &time_cap) {
                            //Test concluded, write the returned data to our file
                            let new_entry = format!("{:?},{},{},{},{},{},{},{},{},{},{},{}\n", trial.file_name(), tc.a_b_depth, utility::store_vc(&test_results.victory), test_results.length, tc.attacker_eval, tc.attacker_mo, test_results.avg_attack_time, test_results.worst_attack_time, tc.defender_eval, tc.defender_mo, test_results.avg_defend_time, test_results.worst_defend_time);
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
        }
    }
}

fn trial_play(board_path: &PathBuf, tc: &TestConfiguration, time_cap:&u8) -> Result<TestData, ()> {
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
        if instance.turn >= 100 {
            //If a game goes on this long, there's probably a stalemate, exit without victory
            let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
            return Ok(test_results)
        }
        if instance.turn % 10 == 1 {println!("Currently on turn {}", instance.turn);}//Establishes that something is happening in slow tests
        let turn_parity = instance.turn % 2 == 1;
        let player_history = if turn_parity {&attacker_states} else {&defender_states};
        let new_move: Option<MoveRequest>;
        if turn_parity {
            //Attacker Turn
            let start_time = Instant::now();
            new_move = player::get_move(&instance, &tc.attacker_eval, player_history, tc.attacker_mo, tc.a_b_depth, time_cap, Instant::now());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Could be a rare type of Game Victory
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::AttackerExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else { return Err(())}
            }
            let attacker_turn_no = ((instance.turn + 1)/2) as u128;
            avg_attack_time = ((avg_attack_time * (attacker_turn_no - 1)) + total_time) / attacker_turn_no;
            if total_time > worst_attack_time {worst_attack_time = total_time;}
        } else {
            //Defender Turn
            let start_time = Instant::now();
            new_move = player::get_move(&instance, &tc.defender_eval, player_history, tc.defender_mo, tc.a_b_depth, time_cap, Instant::now());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Error handling for rare game conditions
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::DefenderExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else {return Err(())}
            }
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

fn extinction_check(instance: &GameState) -> bool {
    //Check for extinction condition, otherwise, return error
    let mut a_piece_count: u8 = 0;
    let mut d_piece_count: u8 = 0;
    let turn_parity = instance.turn % 2 == 1;
    for i in 0..(instance.sizen * instance.sizen) {
        match instance.board.get(&i) {
            Some(&Piece::Attacker) => {a_piece_count += 1;}
            Some(&Piece::Defender) | Some(&Piece::King) => {d_piece_count += 1;}
            None => {}
        }
    }
    if turn_parity {
        if a_piece_count == 0 {
            println!("Extinction Condition!");
            instance.show_board();
            return true
        }
    } else if d_piece_count < 2 {
            println!("Extinction Condition?");
            instance.show_board();
            return true
    }
    false
}

//Just for sign and Relevancy testing
pub fn algorithmic_trial_eval_for_sign_and_relevancy_testing(trial_directory: &str, evaluations: u16, move_orders: u8, output_name: &str, a_b_depth: u8, time_cap:u8, dir: u8) {
    //Iterates over GameState files to hold matches between Algorithmic Players
    println!("Testing evaluations up to {} over the directory: {}", evaluations, trial_directory);
    let data_name = format!("../{}_test_result.txt", output_name);
    let file_path = PathBuf::from(trial_directory).join(data_name);
    let mut file = fs::File::create(file_path).expect("Error creating file for algorithmic trial match output.");
    let paths = fs::read_dir(trial_directory).unwrap();

    let data_header = "Game File, Search Depth, Victory, Length, Attacker Eval, Attacker Mord, Avg Attack Time, Slowest Attack Time, Defender Eval, Defender Mord, Avg Defense Time, Slowest Defense Time\n".to_string();
        if file.write(data_header.as_bytes()).is_ok() {
            println!("CSV successfully written to file.");
        }

    println!("Setting up test parameters...");
    //Generate Test Configurations:
    let mut eval_pairs: Vec<(u16,u16)> = Vec::new();
    
    
    let binary = format!("{evaluations:014b}");

    for (i,c) in binary.chars().enumerate(){
        if c == '0' && dir==1 {
            let new = evaluations + 2_u16.pow((13-i).try_into().unwrap());
            eval_pairs.push((evaluations,new));
            eval_pairs.push((new, evaluations));
        }
        if c == '1' && dir==2 {
            let new = evaluations - 2_u16.pow((13-i).try_into().unwrap());
            eval_pairs.push((evaluations,new));
            eval_pairs.push((new, evaluations));
        }
    }

    //Begin iterating game boards with various configurations
    for path in paths {
        if path.is_ok() {
            let trial = path.unwrap();
            println!("Running trial on: {:?}", trial.file_name());
            //Permute Algorithms, and eval heuristics
            for algorithmic_pair in &eval_pairs{
                let tc = TestConfigurationAdj{attacker_eval: algorithmic_pair.0, defender_eval: algorithmic_pair.1, attacker_mo: move_orders, defender_mo: move_orders, a_b_depth};
                if let Ok(test_results) = trial_play_for_sign_and_relevancy_testing(&trial.path(), &tc, &time_cap) {
                    //Test concluded, write the returned data to our file
                    let new_entry = format!("{:?},{},{},{},{},{},{},{},{},{},{},{}\n", trial.file_name(), tc.a_b_depth, utility::store_vc(&test_results.victory), test_results.length, tc.attacker_eval, tc.attacker_mo, test_results.avg_attack_time, test_results.worst_attack_time, tc.defender_eval, tc.defender_mo, test_results.avg_defend_time, test_results.worst_defend_time);
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
}

//Just for sign and Relevancy testing
fn trial_play_for_sign_and_relevancy_testing(board_path: &PathBuf, tc: &TestConfigurationAdj, time_cap:&u8) -> Result<TestData, ()> {
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
        if instance.turn % 10 == 1 {println!("Currently on turn {}", instance.turn);}//Establishes that something is happening in slow tests
        let turn_parity = instance.turn % 2 == 1;
        let player_history = if turn_parity {&attacker_states} else {&defender_states};
        let new_move: Option<MoveRequest>;
        if turn_parity {
            //Attacker Turn
            let start_time = Instant::now();
            new_move = player::get_move_for_sign_and_relevancy_testing(&instance, &tc.attacker_eval, player_history, tc.attacker_mo, tc.a_b_depth, time_cap, Instant::now());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Could be a rare type of Game Victory
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::AttackerExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else { return Err(())}
            }
            let attacker_turn_no = ((instance.turn + 1)/2) as u128;
            avg_attack_time = ((avg_attack_time * (attacker_turn_no - 1)) + total_time) / attacker_turn_no;
            if total_time > worst_attack_time {worst_attack_time = total_time;}
        } else {
            //Defender Turn
            let start_time = Instant::now();
            new_move = player::get_move_for_sign_and_relevancy_testing(&instance, &tc.defender_eval, player_history, tc.defender_mo, tc.a_b_depth, time_cap, Instant::now());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Error handling for rare game conditions
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::DefenderExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else {return Err(())}
            }
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

//Just for weight testing
pub fn algorithmic_trial_eval_for_weight_testing(trial_directory: &str, move_orders: u8, a_b_depth: u8, time_cap:u8, mut weights: Vec<f32>) {
    println!("{:?}", weights);
    let mut some = 10;
    while some > 0 {
        let mut i: usize = 559;
        while i >= 280{
            let f = i%14;
            let base = i as u16;
            //println!("{}", base);
            //println!("base: m:{}, f:{}, adj:{}", (base % 280) /14, base % 14, base / 280);
            //println!("base+: m:{}, f:{}, adj:{}", ((base+280) % 280) /14, (base+280) % 14, (base+280) / 280);
            //println!("base-: m:{}, f:{}, adj:{}", ((base-280) % 280) /14, (base-280) % 14, (base-280) / 280);
            if weights[f]!=0.0 && weights[f]!= -0.0{
                let eval_pairs: Vec<(u16,u16)> = vec![(base,base - 280),(base -280 ,base),(base,base + 280),(base +280 ,base)];
                
                let mut res = ((0,0),(0,0));
                //Begin iterating game boards with various configurations
                let paths = fs::read_dir(trial_directory).unwrap();
                for path in paths {
                    if path.is_ok() {
                        let trial = path.unwrap();
                        //Permute Algorithms, and eval heuristics
                        for algorithmic_pair in &eval_pairs{
                            let tc = TestConfigurationAdj{attacker_eval: algorithmic_pair.0, defender_eval: algorithmic_pair.1, attacker_mo: move_orders, defender_mo: move_orders, a_b_depth};
                            if let Ok(test_results) = trial_play_for_weight_testing(&trial.path(), &tc, &time_cap, weights.clone()) {
                                //Test concluded, write the returned data to our file
                                let mut win = 0;
                                let mut loss = 0;
                                let vic = utility::store_vc(&test_results.victory);
                                if tc.attacker_eval == base{
                                    if vic == "K"{//King in corner -> defender won
                                        win = 1;
                                    }
                                    else if vic == "C"{//King captured -> defender lost
                                        loss = 1;
                                    }
                                    if tc.defender_eval == base-280{
                                        res.0.0 += win;
                                        res.0.1 += loss;
                                    }else{
                                        res.1.0 += win;
                                        res.1.1 += loss;
                                    }
                                }else{ //tc.defender_eval == base as u16
                                    if vic == "K"{//King in corner -> attacker lost
                                        loss = 1;
                                    }
                                    else if vic == "C"{//King captures -> attacker won
                                        win = 1;
                                    }
                                    if tc.attacker_eval == base-280{
                                        res.0.0 += win;
                                        res.0.1 += loss;
                                    }else{
                                        res.1.0 += win;
                                        res.1.1 += loss;
                                    }
                                }
                            } else {
                                println!("Test ended in error. Warning written to file.");
                            }
                        }
                    }
                }
                
                let multi = ((i % 280) /14) as f32 ;
        
                //println!("{:?}",res);
                if res.0.0>res.0.1{
                    //println!("Factor {} adjusted from weight {} to {}", f, weights[f], weights[f] * ( 1. - (multi-10.)));
                    weights[f] *= 1. - (multi-10.);
                }else if res.1.0>res.1.1{
                    //println!("Factor {} adjusted from weight {} to {}", f, weights[f], weights[f] * ( 1. + (multi-10.)));
                    weights[f] *= 1. + (multi-10.);
                }else{
                    //println!("Factor {} not adjusted", f);
                }
                println!("{:?}",res);
            }
            println!("{},{:?}", i, weights);
            i -= 1;
        }
    some -= 1;
    }
}

//Just for weight testing
fn trial_play_for_weight_testing(board_path: &PathBuf, tc: &TestConfigurationAdj, time_cap:&u8, weights: Vec<f32>) -> Result<TestData, ()> {
    //Silent gameplay for testing algorithms
    let mut instance: GameState = utility::read_state_from_file(board_path).unwrap();
    let mut defender_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut attacker_states: VecDeque<HashMap<u8, Piece>> = VecDeque::new();
    let mut avg_attack_time: u128 = 0;//All times are being measured in milliseconds
    let mut worst_attack_time: u128 = 0;
    let mut avg_defend_time: u128 = 0;
    let mut worst_defend_time: u128 = 0;
    loop{
        //Main Gameplay loop
        if instance.turn >= 255 {
            //If a game goes on this long, there's probably a stalemate, exit without victory
            let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
            return Ok(test_results)
        }
        let turn_parity = instance.turn % 2 == 1;
        let player_history = if turn_parity {&attacker_states} else {&defender_states};
        let new_move: Option<MoveRequest>;
        if turn_parity {
            //Attacker Turn
            let start_time = Instant::now();
            new_move = player::get_move_for_weight_testing(&instance, &tc.attacker_eval, player_history, tc.attacker_mo, tc.a_b_depth, time_cap, Instant::now(), weights.clone());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Could be a rare type of Game Victory
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::AttackerExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else { return Err(())}
            }
            let attacker_turn_no = ((instance.turn + 1)/2) as u128;
            avg_attack_time = ((avg_attack_time * (attacker_turn_no - 1)) + total_time) / attacker_turn_no;
            if total_time > worst_attack_time {worst_attack_time = total_time;}
        } else {
            //Defender Turn
            let start_time = Instant::now();
            new_move = player::get_move_for_weight_testing(&instance, &tc.defender_eval, player_history, tc.defender_mo, tc.a_b_depth, time_cap, Instant::now(), weights.clone());
            let total_time = start_time.elapsed().as_millis();
            if new_move.is_none() {
                //Error handling for rare game conditions
                if extinction_check(&instance) {
                    instance.victory = Some(VictoryCondition::DefenderExtinction);
                    let test_results = TestData{avg_attack_time,worst_attack_time,avg_defend_time,worst_defend_time, victory: instance.victory, length: instance.turn};
                    return Ok(test_results)
                } else {return Err(())}
            }
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