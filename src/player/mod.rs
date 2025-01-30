//Main Organization for alpha-beta algorithmic players
use crate::{GameState, MoveRequest, Direction, Piece};
use std::cmp::{max, min};
use std::collections::{VecDeque, HashMap};

mod game_evaluation;

pub fn get_move(present:&GameState, eval: &u8, history: &VecDeque<HashMap<u8, Piece>>, move_order: u8)-> Option<MoveRequest>{
    //Alpha-Beta algorithmic players recieve a GameState, and return their favorite.

    let a_b_depth: u8 = 3;
    let alpha = i32::MIN;
    let beta = i32::MAX;

    let result = a_b_search(present.clone(), a_b_depth, alpha, beta, None, eval, Some(history), move_order).1;
    Some(result)
}

fn a_b_search(state: GameState, depth: u8, alph: i32, bet: i32, path: Option<MoveRequest>, eval: &u8, history: Option<&VecDeque<HashMap<u8, Piece>>>, move_order:u8) -> (i32, MoveRequest) {
    //Adaptation of Fail-Hard Alpha-Beta search. Return values contain both the obtained value, and the path which leads to it.
    if state.victory.is_some() || depth == 0 {
        return (game_evaluation::game_state_evaluation(&state, eval), path.unwrap())
    }
    
    let maximizing = state.turn % 2 == 0;

    if maximizing {
        //Maximizing player (Defender)
        let mut alpha = alph;
        let mut value = i32::MIN;
        let all_moves = move_list(&state, move_order);
        let mut candidate_move = all_moves[0].clone();
        //We use alpha & beta indirectly here to avoid scopal issues with the loop
        for possible_move in all_moves {
            //Searches through every possible move for a given state
            let backup = possible_move.clone();
            let resultant_state = state.fq_game_update(&possible_move.clone());
            if history.is_some() && history.unwrap().contains(&resultant_state.board) {continue}//Disallow loops (within reason, checking hashmaps is expensive)
            let search_result = a_b_search(resultant_state, depth - 1, alpha, bet, Some(possible_move), eval, None, move_order).0;
            if search_result > value {
                //Update the value and save the relevant move as well
                value = search_result;
                candidate_move = backup;
            }
            if value > bet {break}
            alpha = max(alpha, value);
        }
        (value, candidate_move)//Return Statement
    } else {
        //Minimizing player (Attacker)
        let mut beta = bet;
        let mut value = i32::MAX;
        let all_moves = move_list(&state, move_order);
        let mut candidate_move = all_moves[0].clone();
        for possible_move in move_list(&state, move_order) {
            let backup = possible_move.clone();
            let resultant_state = state.fq_game_update(&possible_move.clone());
            if history.is_some() && history.unwrap().contains(&resultant_state.board) {continue}
            let search_result = a_b_search(resultant_state, depth - 1, alph, beta, Some(possible_move), eval, None, move_order).0;
            if search_result < value {
                value = search_result;
                candidate_move = backup;
            }
            if value < alph {break}
            beta = min(beta, value);
        }
        (value, candidate_move)//Return Statement
    }

}

fn move_list(game: &GameState, move_order: u8) -> Vec<MoveRequest> {
    //Generates all moves which are valid in a given GameState
    let mut moves = Vec::new();
    let search_order = controlled_indices(game, move_order);
    let turn_parity = game.turn % 2 == 1;
    let mut row_moves =Vec::new();
    let mut col_moves = Vec::new();

    for i in search_order.0 {
        let row = game.peek_row(i);
        let new_moves_r = rmg(i, turn_parity, game.sizen, row);
        row_moves.extend(new_moves_r);
    }
    for i in search_order.1 {
        let col = game.peek_col(i);
        let new_moves_c = cmg(i, turn_parity, game.sizen, col);
        col_moves.extend(new_moves_c);
        // moves.extend(new_moves_c);
        // moves.extend(new_moves_r);
    }
    //TODO:Feather the moves from each list together into 
    // let mut intermediate_moves = Vec::new();
    // let rmi = row_moves.into_iter();
    // let cmi = col_moves.into_iter();
    // let mut feather_pair = (rmi.next(), cmi.next());
    // while feather_pair != (None, None) {
    //     if let Some(row_move) = feather_pair.0 {
    //         moves.extend(row_move);
    //     }
    //     if let Some(col_move) = feather_pair.1 {
    //         moves.extend(col_move);
    //     }
    // }

    moves.extend(row_moves);
    moves.extend(col_moves);
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
                        new_moves.push(MoveRequest{magnitude, position: (sizen * (j - none_count_c - 1)) + index, direction: Direction::D});
                    }
                    if j == sizen - 1 && col[(j - none_count_c - 1) as usize].is_none() && (!restricted_positions.contains(&(index + (sizen * j))) || col[(j - none_count_c - 1) as usize] == Some(&Piece::King)) {
                            new_moves.push(MoveRequest{magnitude: none_count_c + 1, position: index + (sizen * (j - none_count_c - 1)), direction: Direction::D});
                    }
                }
            }

            prev_active_c = is_my_piece;
            if is_my_piece {
                //Upward moves to be added
                for magnitude in 1..=none_count_c{
                    let destination = (j * sizen) + index - (sizen * magnitude);
                    if !restricted_positions.contains(&destination) || current_space == Some(&Piece::King) {
                        new_moves.push(MoveRequest{position: index + (j * sizen), direction: Direction::U, magnitude});
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
                        new_moves.push(MoveRequest{direction: Direction::R, magnitude, position: (index * sizen) + j - (none_count_r + 1)});
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
                        new_moves.push(MoveRequest{direction: Direction::L, magnitude, position: (index * sizen) + j});
                    }
                }
            }
            none_count_r = 0;
        }
    }
    new_moves
}

fn controlled_indices(state: &GameState, search_type: u8) -> (Vec<u8>, Vec<u8>) {
    //Calls the indices function dependent upon or search organization argument
    match search_type {
        0 | 1 => {
            //"King First" and "King Last"
            let mut king_pos: u8 = 0;
            for i in 0..(state.sizen * state.sizen) {
                if state.board.get(&i) == Some(&Piece::King) {
                    king_pos = i;
                    break
                }
            }
            let king_row_col: (u8, u8) = (king_pos/state.sizen, king_pos & state.sizen);

            if search_type == 0 {
                //King First
                (indices(state.sizen, king_row_col.0), indices(state.sizen, king_row_col.1))
            } else {
                //King Last
                let row: Vec<u8> = indices(state.sizen, king_row_col.0).iter().cloned().rev().collect();
                let col: Vec<u8> = indices(state.sizen, king_row_col.1).iter().cloned().rev().collect();
                (row, col)
            }
        }
        2 =>{
            //Outwards, from board center
            let outward: Vec<u8> = indices(state.sizen, state.sizen/2);
            (outward.clone(), outward)
        }
        3 =>{
            //Inwards, from board extremes
            let inward: Vec<u8> = indices(state.sizen, state.sizen/2).iter().cloned().rev().collect();
            (inward.clone(), inward)
        }
        _ => {
            println!("Unhandled value in controlled_indices, defaulting to zero");
            let default: Vec<u8> = indices(state.sizen, 0);
            (default.clone(), default)
        }
    }
}

fn indices(sizen:u8, king: u8) -> Vec<u8> {
    //Retrieves "King-centric" row/col ordering for move generation
    //I've elected to hard-code this, because doing this abstractly
    //would be a great way to get untraceable bugs.
    match sizen{
        7 => {
            match king {
                0 => {vec![0,1,2,3,4,5,6]}
                1 => {vec![1,0,2,3,4,5,6]}
                2 => {vec![2,1,3,0,4,5,6]}
                3 => {vec![3,2,4,1,5,0,6]}
                4 => {vec![4,3,5,2,6,1,0]}
                5 => {vec![5,4,6,3,2,1,0]}
                6 => {vec![6,5,4,3,2,1,0]}
                _ => {vec![0,1,2,3,4,5,6]}
            }
        }
        9 => {
            match king {
                0 => {vec![0,1,2,3,4,5,6,7,8]}
                1 => {vec![1,0,2,3,4,5,6,7,8]}
                2 => {vec![2,1,3,0,4,5,6,7,8]}
                3 => {vec![3,2,4,1,5,0,6,7,8]}
                4 => {vec![4,3,5,2,6,1,7,0,8]}
                5 => {vec![5,4,6,3,7,2,8,1,0]}
                6 => {vec![6,5,7,4,8,3,2,1,0]}
                7 => {vec![7,6,8,5,4,3,2,1,0]}
                8 => {vec![8,7,6,5,4,3,2,1,0]}
                _ => {vec![0,1,2,3,4,5,6,7,8]}
            }
        }
        11 => {
            match king {
                0 => {vec![0,1,2,3,4,5,6,7,8,9,10]}
                1 => {vec![1,0,2,3,4,5,6,7,8,9,10]}
                2 => {vec![2,1,3,0,4,5,6,7,8,9,10]}
                3 => {vec![3,2,4,1,5,0,6,7,8,9,10]}
                4 => {vec![4,3,5,2,6,1,7,0,8,9,10]}
                5 => {vec![5,4,6,3,7,2,8,1,9,0,10]}
                6 => {vec![6,5,7,4,8,3,9,2,10,1,0]}
                7 => {vec![7,6,8,5,9,4,10,3,2,1,0]}
                8 => {vec![8,7,9,6,10,5,4,3,2,1,0]}
                9 => {vec![9,8,10,7,6,5,4,3,2,1,0]}
                10 => {vec![10,9,8,7,6,5,4,3,2,1,0]}
                _ => {vec![0,1,2,3,4,5,6,7,8,9,10]}    
            }
        }
        13 => {
            match king {
                0 => {vec![0,1,2,3,4,5,6,7,8,9,10,11,12]}
                1 => {vec![1,0,2,3,4,5,6,7,8,9,10,11,12]}
                2 => {vec![2,1,3,0,4,5,6,7,8,9,10,11,12]}
                3 => {vec![3,2,4,1,5,0,6,7,8,9,10,11,12]}
                4 => {vec![4,3,5,2,6,1,7,0,8,9,10,11,12]}
                5 => {vec![5,4,6,3,7,2,8,1,9,0,10,11,12]}
                6 => {vec![6,5,7,4,8,3,9,2,10,1,0,11,12]}
                7 => {vec![7,6,8,5,9,4,10,3,11,2,12,1,0]}
                8 => {vec![8,7,9,6,10,5,11,4,12,3,2,1,0]}
                9 => {vec![9,8,10,7,11,6,12,5,4,3,2,1,0]}
                10 => {vec![10,9,11,8,12,7,6,5,4,3,2,1,0]}
                11 => {vec![11,10,12,9,8,7,6,5,4,3,2,1,0]}
                12 => {vec![12,11,10,9,8,7,6,5,4,3,2,1,0]}
                _ => {vec![0,1,2,3,4,5,6,7,8,9,10,11,12]}
    
            }
        }
        _ => {panic!("Invalid argument in mod.rs indices function");}
    }
}