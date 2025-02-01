//Matthew Passage
use hnefatafl_prototype::*;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let human_players: u8 = if arguments.len() > 1 {
        arguments[1].parse().expect("Could not parse argument as a number.")
        } else {
            2
        };

    println!("What board size would you like to play?");
    let board_size = utility::get_board_size();

    match human_players {
        0 => {
            let a_name = "Computer Attacker".to_string();
            let d_name = "Computer Defender".to_string();
            let eval_1 = utility::get_no("Attacker Evaluation Function:".to_string());
            let attacker_move_order = utility::get_no("Atacker Move Ordering:".to_string());
            let eval_2 = utility::get_no("Defender Evaluation Function:".to_string());
            let defender_move_order = utility::get_no("Defender Move Ordering:".to_string());
            game_organization::play(board_size, (a_name, false), (d_name, false), (eval_1, eval_2), (attacker_move_order, defender_move_order));
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
            let move_ordering = utility::get_no("Which move ordering should the algorithm use? (Enter a number 0 or higher)".to_string());
            game_organization::play(board_size, (a_name, !human_defender), (d_name, human_defender), (evaluator, evaluator), (move_ordering, move_ordering));
        }
        2 => {
            let a_name = utility::get_name("Attacker Name".to_string());
            let d_name = utility::get_name("Defender Name".to_string());
            game_organization::play(board_size, (a_name, true), (d_name, true), (0,0), (0,0));
        }
        3 => {
            println!("Trial Match mode");
            let a_b_depth: u8 = utility::get_no("Maximal Search depth:".to_string());
            let eval = utility::get_no("Maximal Evaluation Function:".to_string());
            let order = utility::get_no("Maximal Move Ordering:".to_string());
            let time_cap: u8 = utility::get_no("Time Limit for algorithm (in seconds):".to_string());
            // let eval_2 = utility::get_no("Second Evaluation Function:".to_string());
            // let order_two = utility::get_no("Second Move Ordering:".to_string());
            let trial_path = utility::get_name("Directory for test cases:".to_string());
            let output_name = utility::get_name("Output file name:".to_string());
            game_organization::algorithmic_trial_matches(&trial_path, eval, order, &output_name, a_b_depth, time_cap);
        }
        _ => {
            panic!("Command line argrument was undefined.");
        }
    }
}
