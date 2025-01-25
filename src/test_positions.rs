use crate::{GameState, Piece};
//STructure: (description, d_list, a_list, king)
const POSITION_VEC_11 = vec![
    ("start_position",[38, 48, 49, 50, 58, 59, 61, 62, 70, 71, 72, 82],[3, 4, 5, 6, 7, 16, 33, 44, 55, 56, 66, 77, 43, 54, 65, 64, 76, 87, 104, 113, 114, 115, 116, 117],60),
    ("balanced_midgame", [38, 48, 59, 62, 72], [3, 4, 5, 6, 33, 44, 55, 56, 65, 77, 104, 113], 60),
    ("defender_advantage", [38, 48, 50, 58, 59, 61, 72], [3, 4, 5, 44, 54, 56, 66, 77, 104, 113, 114], 71),
    ("attacker_advantage", [38, 59], [3, 4, 5, 33, 44, 54, 55, 65, 66, 76, 87, 104, 113], 60),
    ("king_close_escape", [38, 59, 62], [3, 4, 5, 33, 44, 54, 56, 65, 76, 104, 113], 105),
    ("defender_near_defeat", [59], [3, 4, 5, 33, 44, 54, 55, 65, 76, 87, 104, 113], 60),
    ("equal_endgame", [38, 59], [3, 4, 33, 44, 65, 104], 60),
    ("king_stalemate", [38, 59], [3, 33, 44, 54, 65, 76, 104], 38),
    ("trap_configurations", [38, 59], [3, 44, 54, 55, 65, 76, 87, 104], 60),
    ("escape_pathways", [38, 59], [3, 33, 44, 54, 65, 104], 49),
    ("deep_position", [38, 48, 50, 59, 62, 72], [3, 4, 5, 44, 54, 55, 65, 76, 87, 104], 60),
    ("shallow_position", [38, 59], [3, 33, 44, 54, 65, 104], 72),
    ("high_branching", [38, 48, 49, 50, 59, 61, 62, 70, 71, 72], [3, 4, 5, 6, 7, 16, 33, 44, 55, 56, 66, 77], 60),
    ("balanced_midgame_2", [37, 47, 58, 61, 69], [2, 3, 4, 15, 43, 54, 55, 65, 76, 86, 103, 113], 60),
    ("defender_advantage_2", [37, 47, 48, 57, 58, 60, 69], [2, 3, 4, 43, 53, 55, 64, 75, 103, 112, 113], 70),
    ("attacker_advantage_2", [37, 60], [2, 3, 4, 32, 43, 53, 54, 64, 65, 75, 86, 103, 112], 60),
    ("king_close_escape_2", [37, 60, 61], [2, 3, 4, 32, 43, 53, 55, 64, 75, 103, 112], 104),
    ("defender_near_defeat_2", [60], [2, 3, 4, 32, 43, 53, 54, 64, 75, 86, 103, 112], 60),
    ("equal_endgame_2", [37, 60], [2, 3, 32, 43, 64, 103], 60),
    ("king_stalemate_2", [37, 60], [2, 32, 43, 53, 64, 75, 103], 37),
    ("trap_configurations_2", [37, 60], [2, 43, 53, 54, 64, 75, 86, 103], 60),
    ("escape_pathways_2", [37, 60], [2, 32, 43, 53, 64, 103], 48),
    ("deep_position_2", [37, 47, 49, 60, 61, 69], [2, 3, 4, 43, 53, 54, 64, 75, 86, 103], 60),
    ("shallow_position_2", [37, 60], [2, 32, 43, 53, 64, 103], 69),
    ("high_branching_2", [37, 47, 48, 49, 60, 58, 61, 69, 70, 71], [2, 3, 4, 5, 6, 15, 32, 43, 54, 55, 65, 76], 60)
]
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