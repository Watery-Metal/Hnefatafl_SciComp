//Main Organization for alpha-beta algorithmic players
use crate::{GameState, MoveRequest, Direction, Piece};
use std::cmp::{max, min};
use std::collections::{VecDeque, HashMap};

mod game_evaluation;

pub fn get_move(present:&GameState, eval: &u8, history: &VecDeque<HashMap<u8, Piece>>)-> Option<MoveRequest>{
    //Alpha-Beta algorithmic players recieve a GameState, and return their favorite.

    let a_b_depth: u8 = 3;
    let alpha = i32::MIN;
    let beta = i32::MAX;

    let result = a_b_search(present.clone(), a_b_depth, alpha, beta, None, eval, Some(history)).1;
    Some(result)
}

fn a_b_search(state: GameState, depth: u8, alph: i32, bet: i32, path: Option<MoveRequest>, eval: &u8, history: Option<&VecDeque<HashMap<u8, Piece>>>) -> (i32, MoveRequest) {
    //Adaptation of Fail-Hard Alpha-Beta search. Return values contain both the obtained value, and the path which leads to it.
    if state.victory.is_some() || depth == 0 {
        return (game_evaluation::game_state_evaluation(&state, eval), path.unwrap())
    }
    
    let maximizing = state.turn % 2 == 0;

    if maximizing {
        //Maximizing player (Defender)
        let mut alpha = alph;
        let mut value = i32::MIN;
        let all_moves = move_list(&state);
        if all_moves.is_empty() {
            println!("No moves found for this state:");
            state.show_board();
            println!("More info: Turn no: {} Victory: {:?}", state.turn, state.victory);
            panic!("Intentionally killing process for bug fixing.");
        }
        let mut candidate_move = all_moves[0].clone();
        //We use alpha & beta indirectly here to avoid scopal issues with the loop
        for possible_move in all_moves { //originally : "while a_move.is_some()"
            //Searches through every possible move for a given state
            let backup = possible_move.clone();
            let resultant_state = state.fq_game_update(&possible_move.clone());
            if history.is_some() && history.unwrap().contains(&resultant_state.board) {continue}//Disallow loops 
            let search_result = a_b_search(resultant_state, depth - 1, alpha, bet, Some(possible_move), eval, None).0;
            if search_result > value {
                //Instead of the max statement, update the value and save the relevant move as well
                value = search_result;
                candidate_move = backup;
            }
            // value = max(value, a_b_search(resultant_state, depth - 1, alpha, bet, &a_move).0);
            if value > bet {break}
            alpha = max(alpha, value);

            // a_move = next_move(&a_move.unwrap(), &state);
        }
        (value, candidate_move)//Return Statement
    } else {
        //Minimizing player (Attacker)
        let mut beta = bet;
        let mut value = i32::MAX;
        let all_moves = move_list(&state);
        let mut candidate_move = all_moves[0].clone();
        for possible_move in move_list(&state) {
            let backup = possible_move.clone();
            let resultant_state = state.fq_game_update(&possible_move.clone());
            if history.is_some() && history.unwrap().contains(&resultant_state.board) {continue}
            let search_result = a_b_search(resultant_state, depth - 1, alph, beta, Some(possible_move), eval, None).0;
            if search_result < value {
                value = search_result;
                candidate_move = backup;
            }
            // value = min(value, a_b_search(resultant_state, depth - 1, alph, beta, &a_move).0);
            if value < alph {break}
            beta = min(beta, value);

            // a_move = next_move(&a_move.unwrap(), &state);
        }
        (value, candidate_move)//Return Statement
    }

}

fn move_list(game: &GameState) -> Vec<MoveRequest> {
    //Generates all moves which are valid in a given GameState
    let mut moves = Vec::new();
    // let throne_space = game.throne % game.sizen;
    let turn_parity = game.turn % 2 == 1;
    for i in 0..game.sizen {
        let row = game.peek_row(i);
        let col = game.peek_col(i);
        let new_moves_r = rmg(i, turn_parity, game.sizen, row);
        let new_moves_c = cmg(i, turn_parity, game.sizen, col);
        //TODO FEATHER MOVES TOGETHER
        moves.extend(new_moves_c);
        moves.extend(new_moves_r);
        }
    moves
}

fn cmg(index: u8, turn_parity: bool,  sizen: u8, col: Vec<Option<&Piece>>) -> Vec<MoveRequest> {
    //"Variable-bound row and column move generation"
    let mut new_moves = Vec::new();
    let restricted_positions = [0, sizen-1, sizen * sizen - 1, (sizen * sizen) - sizen, ((sizen * sizen) - 1)/2];
    // let mut none_count_r: u8 = 0;
    // let mut prev_active_r: bool = false;
    let mut none_count_c: u8 = 0;
    let mut prev_active_c: bool = false;
    for j in 0..sizen {
        let current_space = col[j as usize];
        if current_space.is_none() && j != sizen - 1{
            none_count_c += 1;
        } else {
            let is_my_piece = turn_parity == (current_space == Some(&Piece::Attacker)) && current_space.is_some();
            if none_count_c == 0 {prev_active_c = is_my_piece; continue}

            if prev_active_c {
                //Downward moves to be added
                for magnitude in 1..=none_count_c{
                    let destination = (sizen * (j- none_count_c - 1 + magnitude)) + index;
                    if !restricted_positions.contains(&destination) || col[(j - none_count_c - 1) as usize] == Some(&Piece::King) {
                        new_moves.push(MoveRequest{magnitude: magnitude, position: (sizen * (j - none_count_c - 1)) + index, direction: Direction::D});
                    }
                    if j == sizen - 1 && col[(j - none_count_c - 1) as usize].is_none() {
                        if !restricted_positions.contains(&(index + (sizen * j))) || col[(j - none_count_c - 1) as usize] == Some(&Piece::King) {
                            new_moves.push(MoveRequest{magnitude: none_count_c + 1, position: index + (sizen * (j - none_count_c - 1)), direction: Direction::D});
                        }
                    }
                }
            }

            prev_active_c = is_my_piece;
            if is_my_piece {
                //Upward moves to be added
                for magnitude in 1..=none_count_c{
                    let destination = (j * sizen) + index - (sizen * magnitude);
                    if !restricted_positions.contains(&destination) || current_space == Some(&Piece::King) {
                        new_moves.push(MoveRequest{position: index + (j * sizen), direction: Direction::U, magnitude: magnitude});
                    }
                }
            }
            none_count_c = 0;
        }
    }
    new_moves
}

fn rmg(index: u8, turn_parity: bool,  sizen: u8, row: Vec<Option<&Piece>>) -> Vec<MoveRequest> {
    //"Variable-bound row and column move generation"
    let mut new_moves = Vec::new();
    let restricted_positions = [0, sizen-1, sizen * sizen - 1, (sizen * sizen) - sizen, ((sizen * sizen) - 1)/2];
    let mut none_count_r: u8 = 0;
    let mut prev_active_r: bool = false;
    // let mut none_count_c: u8 = 0;
    // let mut prev_active_c: bool = false;

    //GENERATION OF MOVES (MIXED)
    for j in 0..sizen{
        let current_space = row[j as usize];
        if current_space.is_none() && j != sizen - 1 {
            none_count_r += 1;
        } else {
            // if current_space.is_none() {none_count_r += 1;}
            let is_my_piece = turn_parity == (current_space == Some(&Piece::Attacker)) && current_space.is_some();

            if none_count_r == 0 {prev_active_r = is_my_piece; continue}

            if prev_active_r{
                //Rightward moves to be added
            
                for magnitude in 1..=none_count_r{
                    let destination = (index * sizen) + j - (none_count_r + 1) + magnitude;
                    if !restricted_positions.contains(&destination) || row[(j - none_count_r - 1) as usize] == Some(&Piece::King) {
                        new_moves.push(MoveRequest{direction: Direction::R, magnitude: magnitude, position: (index * sizen) + j - (none_count_r + 1)});
                    }
                }
                if j == (sizen - 1) && current_space.is_none() {
                    //Catching when the end of the board is empty, but a piece can move there still
                    if !restricted_positions.contains(&((index * sizen) + j)) || row[(j - none_count_r - 1) as usize] == Some(&Piece::King) {
                        new_moves.push(MoveRequest{direction: Direction::R, magnitude: none_count_r + 1, position: (index * sizen) + j - (none_count_r + 1)})
                    }
                }
            }
            prev_active_r = is_my_piece;

            if is_my_piece {
                //Leftward moves to be added
                for magnitude in 1..=none_count_r{
                    let destination = (index * sizen) + j - magnitude;
                    if !restricted_positions.contains(&destination) || row[j as usize] == Some(&Piece::King) {
                        new_moves.push(MoveRequest{direction: Direction::L, magnitude: magnitude, position: (index * sizen) + j});
                    }
                }
            }
            none_count_r = 0;
        }
    }
    new_moves
}

