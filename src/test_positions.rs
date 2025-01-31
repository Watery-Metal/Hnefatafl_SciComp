use crate::{GameState, Piece};
//STructure: (description, d_list, a_list, king)
const POS_VEC_11: Vec<(&str,Vec<u8>,Vec<u8>,u8)> = vec![
    ("start_position",vec![38, 48, 49, 50, 58, 59, 61, 62, 70, 71, 72, 82],vec![3, 4, 5, 6, 7, 16, 33, 44, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 104, 113, 114, 115, 116, 117],60),
    ("balanced_midgame", vec![38, 48, 59, 62, 72], vec![3, 4, 5, 6, 33, 44, 55, 56, 65, 77, 104, 113], 60),
    ("defender_advantage", vec![38, 48, 50, 58, 59, 61, 72], vec![3, 4, 5, 44, 54, 56, 66, 77, 104, 113, 114], 71),
    ("attacker_advantage", vec![38, 59], vec![3, 4, 5, 33, 44, 54, 55, 65, 66, 76, 87, 104, 113], 60),
    ("king_close_escape", vec![38, 59, 62], vec![3, 4, 5, 33, 44, 54, 56, 65, 76, 104, 113], 105),
    ("defender_near_defeat", vec![59], vec![3, 4, 5, 33, 44, 54, 55, 65, 76, 87, 104, 113], 60),
    ("equal_endgame", vec![38, 59], vec![3, 4, 33, 44, 65,87, 104,117], 60),
    ("king_stalemate", vec![38, 59], vec![3, 7, 11, 44, 32, 76, 104], 38),
    ("trap_configurations", vec![38, 59], vec![49, 44, 71, 55, 53, 54, 76, 104], 60),
    ("escape_pathways", vec![38, 59], vec![3, 33, 44, 54, 65, 104], 49),
    ("deep_position", vec![38, 48, 50, 59, 62, 72], vec![3, 4, 5, 44, 54, 55, 65, 76, 87, 104], 60),
    ("shallow_position", vec![38, 59], vec![3, 33, 44, 54, 65, 104], 72),
    ("high_branching", vec![38, 48, 49, 50, 59, 61, 62, 70, 71, 72], vec![3, 4, 5, 6, 7, 16, 33, 44, 55, 56, 66, 77], 60),
    ("balanced_midgame_2", vec![37, 47, 58, 61, 69], vec![2, 3, 4, 15, 43, 54, 55, 65, 76, 86, 103, 113], 60),
    ("defender_advantage_2", vec![37, 47, 48, 57, 58, 60, 69], vec![2, 3, 4, 43, 53, 55, 64, 75, 103, 112, 113], 70),
    ("attacker_advantage_2", vec![58], vec![2, 3, 4, 32, 43, 53, 54, 64, 75, 86, 103, 112], 60),
    ("king_close_escape_2", vec![37, 60, 61], vec![2, 3, 4, 32, 43, 53, 55, 64, 75, 103, 112], 104),
    ("defender_near_defeat_2", vec![37, 61], vec![2, 3, 4, 32, 43, 53, 54, 64, 65, 75, 86, 103, 112, 22, 77, 27, 82, 56], 60),
    ("trap_configurations_2", vec![38, 59], vec![50, 44, 45,71 ,72, 55, 52, 64, 73, 85, 104], 61),
    ("escape_pathways_2", vec![37, 60], vec![2, 32, 43, 53, 64, 103], 48),
    ("deep_position_2", vec![37, 47, 49, 61, 69], vec![2, 3, 4, 43, 53, 54, 64, 75, 86, 103], 60),
    ("shallow_position_2", vec![37, 60], vec![2, 32, 43, 53, 64, 103], 69),
    ("high_branching_2", vec![37, 47, 48, 49, 59, 58, 61, 69, 70, 71], vec![2, 3, 4, 5, 6, 15, 32, 43, 54, 55, 65, 76], 60),
    ("wc_start_casshern/alex hnefltafl_8", vec![38, 48, 49, 50, 25, 92, 61, 95, 71, 72, 82],vec![3, 4, 5, 6,29, 16, 33, 44, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 104, 102, 112, 115, 116, 117],60),
    ("wc_start_alex hnefltafl/casshern_8", vec![41, 48, 49, 50, 58, 59, 94, 62, 70, 71, 105, 79],vec![25, 4, 5, 6, 7, 16, 33, 46, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 104, 113, 103, 115, 116, 95],60),
    ("wc_start_draganov/garun19_8",vec![35, 48, 49, 20, 58, 59, 61, 62, 70, 71, 72, 85],vec![3, 4, 5, 6, 29, 13, 33, 44, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 104, 113, 114, 115, 94, 95],60),
    ("wc_start_garun19/draganov_8",vec![38, 46, 49, 50, 25, 59, 61, 62, 70, 71, 94],vec![3, 15, 5, 6, 7, 16, 33, 44, 55, 56, 68, 77, 43, 54, 65, 64, 76, 87, 104, 113, 114, 115, 116, 90],60),
    ("start_floki/cacreal_10",vec![16, 49, 17, 91, 15, 61, 62, 70, 71, 72, 82],vec![14, 4, 5, 6, 18, 33, 44, 57, 89, 66, 77, 43, 54, 65, 64, 76, 87, 107, 113, 114, 115, 116, 117],60),
    ("start_cacreal/floki_10",vec![38, 48, 49, 53, 58, 59, 61, 29, 70, 71, 108, 85],vec![3, 4, 5, 6, 7, 19, 33, 44, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 101, 113, 114, 115, 107, 95],60),
    ("wc_start_sqaree/antonius_10",vec![41, 48, 53, 25, 59, 61, 62, 70, 71, 72, 82],vec![3, 4, 5, 6, 7, 16, 33, 44, 55, 56, 66, 79, 96, 54, 65, 64, 74, 87, 104, 113, 114, 115, 116, 117],27),
    ("wc_start_antonius/sqaree_10",vec![38, 48, 49, 50, 58, 59, 61, 62, 68, 108, 75, 85],vec![3, 4, 5, 6, 7, 13, 33, 44, 55, 56, 66, 77, 41, 54, 65, 64, 76, 87, 101, 113, 114, 115, 118, 95],60),
    ("wc_start_hagbard/luizz_16",vec![38, 48, 49, 50, 58, 59, 63, 70, 71, 105, 82],vec![3, 4, 5, 6, 29, 16, 33, 44, 55, 56, 66, 77, 43, 54, 65, 85, 76, 98, 99, 91, 114, 115, 116, 108],92),
    ("wc_start_luizz/hagbard_16",vec![48, 49, 50, 91, 61, 62, 92, 71, 72, 79],vec![25,2, 5, 6, 7, 16, 33, 23, 55, 66, 88, 32, 54, 65, 64, 76, 87, 101, 113, 114, 115, 116, 117],60)
];
const POS_VEC_7: Vec<(&str, Vec<u8>, Vec<u8>, u8)> = vec![
    ("start_position", vec![17, 23, 25, 31], vec![3, 10, 21, 22, 26, 27, 38, 45], 24),
    ("balanced_midgame", vec![17, 23, 25], vec![3, 10, 21, 22, 27, 38], 24),
    ("defender_advantage", vec![17, 23, 25, 31], vec![3, 10, 21, 22, 26, 27], 31),
    ("attacker_advantage", vec![23], vec![3, 10, 21, 22, 26, 27, 38, 45], 24),
    ("king_close_escape", vec![17, 23], vec![3, 10, 21, 26, 27, 38], 6),
    ("defender_near_defeat", vec![25], vec![3, 10, 21, 22, 26, 27, 38, 45], 24),
    ("equal_endgame", vec![17, 23], vec![3, 10, 21, 38, 45], 24),
    ("king_stalemate", vec![17, 23], vec![3, 10, 21, 22, 27, 38], 17),
    ("trap_configurations", vec![23, 31], vec![10, 21, 22, 26, 27, 38], 24),
    ("escape_pathways", vec![23, 25], vec![3, 10, 21, 26, 27, 38], 17),
    ("deep_position", vec![17, 23, 25], vec![3, 10, 21, 22, 26, 27, 38], 24),
    ("shallow_position", vec![17, 23], vec![3, 10, 21, 38], 31),
    ("high_branching", vec![17, 23, 25, 31], vec![3, 10, 21, 22, 26, 27, 38, 45], 24),
    ("balanced_midgame_2", vec![23, 25, 31], vec![3, 10, 21, 26, 27], 24),
    ("defender_advantage_2", vec![17, 23, 31], vec![3, 10, 21, 22, 27], 31),
    ("attacker_advantage_2", vec![23], vec![3, 10, 21, 22, 26, 27, 38], 24),
    ("king_close_escape_2", vec![23, 25], vec![3, 10, 21, 22, 27, 38], 6),
    ("defender_near_defeat_2", vec![25], vec![3, 10, 21, 22, 26, 27, 38], 24),
    ("trap_configurations_2", vec![17, 23], vec![10, 21, 22, 26, 27, 38], 31),
    ("escape_pathways_2", vec![23, 25], vec![3, 10, 21, 22, 27, 38], 17),
    ("deep_position_2", vec![17, 23, 25], vec![3, 10, 21, 22, 27, 38], 24),
    ("shallow_position_2", vec![23], vec![3, 10, 21, 26, 27], 31),
    ("high_branching_2", vec![17, 23, 25, 31], vec![3, 10, 21, 22, 26, 27, 38, 45], 24),
    ("wc_start_draganov/colophonius_4", vec![17, 23, 25, 38], vec![3, 9, 21, 22, 26, 27, 39, 45], 31),
    ("wc_start_draganov/colophonius_8", vec![20, 37, 25, 38], vec![3, 16, 21, 36, 26, 27, 39, 45], 31),
    ("wc_start_draganov/colophonius_12", vec![20, 30, 25, 38], vec![3, 16, 21, 36, 12, 27, 39, 45], 17),
    ("wc_start_draganov/colophonius_16", vec![20, 30, 11, 38], vec![4, 16, 21, 36, 12, 34, 39, 45], 31),
    ("wc_start_draganov/colophonius_20", vec![20, 30, 40], vec![4, 16, 21, 36, 12, 3, 18, 45], 31),
    ("wc_start_draganov/colophonius_24", vec![27, 23, 40], vec![4, 16, 21, 36, 12, 33, 25, 45], 31),
    ("wc_start_draganov/colophonius_28", vec![27, 23, 40], vec![4, 16, 21, 36, 26, 30, 25, 45], 12),
];
impl GameState{
    fn show_board(&self) {
        //Prints the board for human readers
        let side_length : u8 = self.sizen;
        let mut header = String::new();
        header.push_str("__|");
        for i in 0..side_length {
            header.push_str(&format!("_{}_", utility::n2c(i).unwrap()));
        }
        println!("{}", header);
        for row in 0..side_length {
            //Row loop, top to bottom
            let mut rank = String::new();
            if row < 9 {rank.push_str(&format!("{}  |", row+1));} else {rank.push_str(&format!("{} |", row+1));}
            for col in 0..side_length {
                let piece = self.board.get(&((self.sizen * row) + col));
                match piece {
                    Some(Piece::King) => {rank.push_str("K  ");},
                    Some(Piece::Defender) => {rank.push_str("D  ");},
                    Some(Piece::Attacker) => {rank.push_str("A  ")},
                    None => {rank.push_str("*  ")},//Empty Tile
                    // _ => {panic!("Invalid value obtained in board hashmap during show_board function.");},
                }
            }
            println!("{}\n", rank);
        }
    }

    fn create_all_positions(position_list: Vec<>, sizen: u8, length: usize) -> Vec<>{
        let mut list = vec![];
        for element in position_list{
            let state = GameState::create_position_from_list(i.2,i.1,sizen,i.3);
            list.add((i.0,state));
        }
        let mut counter = 1;
        while list.len()<length{
            let state = GameState::rnd(sizen);
            list.add((format!("rnd: {}",counter),state));
            counter+=1;
        }
        list 
    }

    fn create_position_from_list(a_vec: Vec<>; d_vec: Vec<>, sizen: u8, king: u8){
        //Return a fresh game, given positions & board size
        let turn: u32 = 1;
        let sizen: u8 = sizen;
        let corners: Vec<u8> = vec![0, sizen - 1, (sizen * sizen) - 1, (sizen * sizen) - sizen];
        let throne: u8 = ((sizen * sizen) - 1) / 2;
        let victory = None;
        let mut board: HashMap<u8, Piece> = HashMap::new();
        for position in a_vec {
            board.insert(position, Piece::Attacker);
        }
        for position in d_vec {
            board.insert(position, Piece::Defender);
        }
        board.insert(king, Piece::King);
        GameState{turn, board, sizen, corners, throne, victory}
    }

    fn rnd(sizen: u8) -> Self {
        //Return a fresh game, given board size
        let turn: u32 = 1;
        let sizen: u8 = sizen;
        let corners: Vec<u8> = vec![0, sizen - 1, (sizen * sizen) - 1, (sizen * sizen) - sizen];
        let throne: u8 = ((sizen * sizen) - 1) / 2;
        let victory = None;
        let mut board: HashMap<u8, Piece> = HashMap::new();
        let mut taken = Vec::new();
        match sizen {
            7 => {
                let mut rng = rand::thread_rng();
                let a: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                let d: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                let mut la = (DEFAULT_7_A.len() as f64 * a).floor() as u8;
                let mut ld = (DEFAULT_7_D.len() as f64 * d).floor() as u8;
                let mut lk = 1;
                while la>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if pos != throne && !corners.contains(&pos) && !taken.contains(&pos){
                        board.insert(pos, Piece::Attacker);
                        taken.push(pos);
                        la -= 1;
                    }
                }
                while ld>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if pos != throne && !corners.contains(&pos) && !taken.contains(&pos){
                        board.insert(pos, Piece::Defender);
                        taken.push(pos);
                        ld -= 1;
                    }
                }
                while lk>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if !taken.contains(&pos){
                        board.insert(pos, Piece::King);
                        taken.push(pos);
                        lk -= 1;
                    }
                }
            }
            11 => {
                let mut rng = rand::thread_rng();
                let a: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                let d: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                let mut la = (DEFAULT_11_A.len() as f64 * a).floor() as u8;
                let mut ld = (DEFAULT_11_D.len() as f64 * d).floor() as u8;
                let mut lk = 1;
                while la>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if pos != throne && !corners.contains(&pos) && !taken.contains(&pos){
                        board.insert(pos, Piece::Attacker);
                        taken.push(pos);
                        la -= 1;
                    }
                }
                while ld>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if pos != throne && !corners.contains(&pos) && !taken.contains(&pos){
                        board.insert(pos, Piece::Defender);
                        taken.push(pos);
                        ld -= 1;
                    }
                }
                while lk>0{
                    let i: f64 = rng.gen_range(1..=10000) as f64/10000.0;
                    let pos = ((sizen*sizen) as f64 * i).floor() as u8;
                    if !taken.contains(&pos){
                        board.insert(pos, Piece::King);
                        taken.push(pos);
                        lk -= 1;
                    }
                }
            }
            _ => {println!("Sorry, this board size doesn't have a data table!"); panic!("Board size requsted for which no default game state has been implemented.");}
        }
        GameState{turn, board, sizen, corners, throne, victory}
    }
}
