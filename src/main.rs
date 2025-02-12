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
        4 => {
            println!("Trial eval mode for signs/relevancy");
            let a_b_depth: u8 = utility::get_no("Search depth:".to_string());
            let eval = utility::get_no_16("Evaluation Function:".to_string());
            let order = utility::get_no("Move Ordering:".to_string());
            let time_cap: u8 = utility::get_no("Time Limit for algorithm (in seconds):".to_string());
            let dir: u8 = utility::get_no("Direction(1 Up, 2 Down):".to_string());
            // let eval_2 = utility::get_no("Second Evaluation Function:".to_string());
            // let order_two = utility::get_no("Second Move Ordering:".to_string());
            let trial_path = utility::get_name("Directory for test cases:".to_string());
            let output_name = utility::get_name("Output file name:".to_string());
            game_organization::algorithmic_trial_eval_for_sign_and_relevancy_testing(&trial_path, eval, order, &output_name, a_b_depth, time_cap, dir);
        }
        5 => {
            println!("Trial eval mode for weights");
            let a_b_depth: u8 = utility::get_no("Search depth:".to_string());
            let order = utility::get_no("Move Ordering:".to_string());
            let time_cap: u8 = utility::get_no("Time Limit for algorithm (in seconds):".to_string());
            // let eval_2 = utility::get_no("Second Evaluation Function:".to_string());
            // let order_two = utility::get_no("Second Move Ordering:".to_string());
            let trial_path = utility::get_name("Directory for test cases:".to_string());
            
            let signs = vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0]; //Defender wants max
            //              ["MB", "N","FRC","FC", "FE", "MC", "ME", "MD", "MA","CD", "CA","GD","GA", "CR"];
            let choice = [0.,0.,0.,1.,0.,0.,1.,1.,1.,0.,0.,0.,1.,0.];
            //let choice = vec![0.,0.,0.,1.,1.,0.,1.,1.,1.,0.,0.,1.,0.,1.];
            let mut weights = calc_weights(board_size, &signs);
            for i in 0..weights.len(){
                weights[i] *= choice[i];
            }
            //let weights = vec![-0.0, 0.0, 0.0, 2800.0, 0.0, -0.0, 308.00003, 56.0, 3240.0, 0.0, 0.0, -103.125, -0.0, 524880.0];
            //let weights = vec![-0.0, 0.0, 0.0, 470400.0, 0.0, -0.0, -1422.9602, 752.64, 4199040.0, 0.0, 0.0, 850.78125, -0.0, -11019961000.0];
            //let weights = vec![-0.0, 0.0, 0.0, 7902720000.0, 0.0, -0.0, 657407.6, 1011548.1, 544195580000.0, 0.0, 0.0, -701894.5, -0.0, 2.3136628e16];
            game_organization::algorithmic_trial_eval_for_weight_testing(&trial_path, order, a_b_depth, time_cap, weights);
        }
        _ => {
            panic!("Command line argrument was undefined.");
        }
    }
}

const DEFAULT_7_A: [u8; 8] = [3,10,21,22,26,27,38,45];
const DEFAULT_7_D: [u8; 4] = [17,23,25,31];
const DEFAULT_11_D: [u8; 12] = [38,48,49,50,58,59,61,62,70,71,72,82];
const DEFAULT_11_A: [u8; 24] = [4,5,6,15,17,27,44,45,53,54,55,57,63,65,66,67,75,76,93,103,105,114,115,116];

fn calc_weights(size: u8, signs: &[f32]) -> Vec<f32>{
    let mut mb: f32 = 0.0;
    let mut n: f32 = 0.0;
    let mut frc: f32 = 0.0;
    let fc: f32 = (size - 1) as f32;
    let fe: f32 = (size - 1) as f32 / 2.0;
    let mc: f32 = (size - 1) as f32 / 2.0;
    let me: f32 = (size - 1) as f32 / 4.0;
    let mut md: f32 = 0.0;
    let mut ma: f32 = 0.0;
    let mut cd: f32 = 0.0;
    let mut ca: f32 = 0.0;
    let mut gd: f32 = 0.0;
    let mut ga: f32 = 0.0;
    let cco: f32 = 4.0;
    let _jsduhf= mb *n*frc*md*ma*cd*ca*gd*ga; //just to get rid of stupid warnings ;)
    match size {
        7 => {
            mb = DEFAULT_7_A.len() as f32/DEFAULT_7_D.len() as f32;
            n = DEFAULT_7_A.len() as f32+DEFAULT_7_D.len() as f32;
            frc = 6.0;
            md = 24.0;
            ma = 40.0;
            cd = 4.0;
            ca = 20.0;
            gd = 8.0;
            ga = DEFAULT_7_A.len() as f32;
        }
        _ => {
            mb = DEFAULT_11_A.len() as f32/DEFAULT_11_D.len() as f32;
            n = DEFAULT_11_A.len() as f32+DEFAULT_11_D.len() as f32;
            frc = 18.0;
            md = 60.0;
            ma = 116.0;
            cd = 20.0;
            ca = 92.0;
            gd = DEFAULT_11_D.len() as f32;
            ga = DEFAULT_11_A.len() as f32;
        }
    }
    let mut weights = vec![mb, n, frc, fc, fe, mc, me, md, ma, cd, ca, gd, ga, cco];
    for i in 0..weights.len(){
        weights[i] = signs[i]/ weights[i];
    }
    weights
}