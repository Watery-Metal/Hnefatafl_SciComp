//Matthew Passage
//Early Prototype of Hnefatafl Game logic

use hnefatafl_prototype::*;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let human_players: u8 = if arguments.len() > 1 {
        arguments[1].parse().expect("Could not parse argument as a number.")
        // if human_players > 2 {panic!("Invalid command given. Human player flag can be at most two.");}
        } else {
            2
        };

    println!("What board size would you like to play?");
    let board_size = utility::get_board_size();

    match human_players {
        0 => {
            let a_name = "Computer Attacker".to_string();
            let d_name = "Computer Defender".to_string();
            let eval_1 = utility::get_no("First Evaluation Function:".to_string());
            let eval_2 = utility::get_no("Second Evaluation Function:".to_string());
            game_organization::play(board_size, (a_name, false), (d_name, false), (eval_1, eval_2));
        }
        1 => {
            println!("One human player. Would you like to play Defender? (Y/N)");
            let human_defender = utility::confirm_blank();
            let a_name: String;
            let d_name: String;
            if human_defender {
                d_name = utility::get_name("Defender Name".to_string());
                a_name = "Computer Attacker".to_string();
            } else {
                a_name = utility::get_name("Attacker Name".to_string());
                d_name = "Computer Defender".to_string();
            }
            let evaluator = utility::get_no("Which computer algorithm would you like to play against? (Enter a number 0 or higher)".to_string());
            game_organization::play(board_size, (a_name, !human_defender), (d_name, human_defender), (evaluator, evaluator));
        }
        2 => {
            let a_name = utility::get_name("Attacker Name".to_string());
            let d_name = utility::get_name("Defender Name".to_string());
            game_organization::play(board_size, (a_name, true), (d_name, true), (0,0));
        }
        3 => {
            println!("Trial Match mode");
            let eval_1 = utility::get_no("First Evaluation Function:".to_string());
            let eval_2 = utility::get_no("Second Evaluation Function:".to_string());
            let trial_path = utility::get_name("Directory for test cases:".to_string());
            game_organization::algorithmic_trial_matches(&trial_path, (eval_1, eval_2));
        }
        _ => {
            panic!("This arm is unreachable. More than 2 human players initialized.");
        }
    }
}
