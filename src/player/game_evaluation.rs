//GameState Evaluation functions
use crate::{GameState, Direction, Piece, VictoryCondition, DEFAULT_7_D, DEFAULT_7_A, DEFAULT_11_A, DEFAULT_11_D};
use std::cmp;
use std::collections::{VecDeque, HashMap};

pub fn game_state_evaluation(state: &GameState, eval_no: &u8) -> i32 {
    match eval_no {
        0 => {default_evaluation(state)}
        1 => {
            let signs = vec![-1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0]; //Defender wants max
            //              ["MB", "N","FRC","FC", "FE", "MC", "ME", "MD", "MA","CD", "CA","GD","GA", "CR"];
            let weights = calc_weights(state, &signs);
            eval(state, weights) as i32}
        _=> {panic!("No evaluation function with index {} is present.", eval_no);}
    }
}

fn default_evaluation (state: &GameState) -> i32 {
    //Evaluation of how favorable a GameState is for the Defending player (the maximizer)
    if state.victory.is_some() {
        //Winning move should always take precedence
        match state.victory {
            Some(VictoryCondition::KingCaptured) => {return i32::MIN}
            Some(VictoryCondition::KingInCorner) => {return i32::MAX}
            _=> {panic!("This pattern in game_state_evaluation should be unreachable. GameState victory field was not None, but didn't match to any existing VictoryCondition.");}
        }
    }
    let board_size = state.sizen as i32;
    let mut defender_count: i32 = 0;
    let mut attacker_count: i32 = 0;
    let mut king_pos: i32 = state.throne as i32;
    let mut king_loc: u8 = state.throne;
    for i in 0..(state.sizen * state.sizen) {
        match state.board.get(&i) {
            Some(&Piece::Attacker) => {attacker_count += 1;}
            Some(&Piece::Defender) => {defender_count += 1;}
            Some(&Piece::King) => {king_pos = i as i32; king_loc = i;}
            _ => {}
        }
    }
    let piece_balence: i32 = 20 * ((2 * defender_count) - attacker_count);//Material Balence temp implementation

    let midpoint: i32 = ((state.sizen as i32) + 1) / 2;

    //Fields to Corner temp implementation
    let king_distance = ((state.turn as i32) / 3) * ((king_pos % (state.sizen as i32)) - midpoint).abs() + ((king_pos / (state.sizen as i32)) - midpoint).abs();
    //Lateral distance from an mid is |(king_pos % sizen)-midpoint|
    //Vertical distance from an mid is |(king_pos / sizen)-midpoint|

    //Defense of King temp implementation// Can be more robust with an advanced "peek" function
    let mut king_caravan: i32 = 0;
    let king_surroundings = [state.get_neighbor(&king_loc, Direction::U), state.get_neighbor(&king_loc, Direction::D), state.get_neighbor(&king_loc, Direction::L), state.get_neighbor(&king_loc, Direction::R)];
    //Checking Vertical situation
    if king_surroundings[0].is_some() && king_surroundings[1].is_some() {
        //King has spaces above and below
        let occupancy = (state.board.get(&king_surroundings[0].clone().unwrap().0), state.board.get(&king_surroundings[1].clone().unwrap().0));
        let occ_mat = (occupancy.0.is_some(), occupancy.1.is_some());
        match occ_mat {
            (true, true) => {king_caravan += defender_count;} //King not vulnerable
            (false, false) => {king_caravan += defender_count/2;} //King neither protected nor defended
            (true, false) => {
                if occupancy.0 == Some(&Piece::Defender) {
                    king_caravan += defender_count + board_size; //King Protected and vulnerable
                } else {
                    king_caravan -= defender_count + board_size; //King possibly immediately vulberable
                }
            }
            (false, true) => {
                if occupancy.1 == Some(&Piece::Defender) {
                    king_caravan += defender_count + board_size; //King Protected and vulnerable
                } else {
                    king_caravan -= defender_count + board_size; //King possibly immediately vulberable
                }
            }
        }
    } else {
        //King is on vertical edge
        king_caravan += 2000;//Effectively victory
    }
    //Checking Horizontal situation
    if king_surroundings[2].is_some() && king_surroundings[3].is_some() {
        //King has horizontal spaces
        let occupancy = (state.board.get(&king_surroundings[2].clone().unwrap().0), state.board.get(&king_surroundings[3].clone().unwrap().0));
        let occ_mat = (occupancy.0.is_some(), occupancy.1.is_some());
        match occ_mat {
            (true, true) => {king_caravan += defender_count;} //King not vulnerable
            (false, false) => {king_caravan += defender_count/2;} //King neither protected nor defended
            (true, false) => {
                if occupancy.0 == Some(&Piece::Defender) {
                    king_caravan += defender_count + board_size; //King Protected and vulnerable
                } else {
                    king_caravan -= defender_count + board_size; //King possibly immediately vulberable
                }
            }
            (false, true) => {
                if occupancy.1 == Some(&Piece::Defender) {
                    king_caravan += defender_count + board_size; //King Protected and vulnerable
                } else {
                    king_caravan -= defender_count + board_size; //King possibly immediately vulberable
                }
            }
        }
    } else {
        //King on a horizontal edge
        king_caravan += 2000;
    }

    piece_balence + king_distance + king_caravan
}

fn eval(state: &GameState, weights: Vec<f32>) -> f32 {
    //weight function is assumed to be constant for now
    if state.victory.is_some() {
        //Winning move should always take precedence
        match state.victory {
            Some(VictoryCondition::KingCaptured) => {return f32::MIN}
            Some(VictoryCondition::KingInCorner) => {return f32::MAX}
            _=> {panic!("This pattern in game_state_evaluation should be unreachable. GameState victory field was not None, but didn't match to any existing VictoryCondition.");}
        }
    }
    let _legend = ["MB", "N", "FRC", "FC", "FE", "MC", "ME", "MD", "MA", "CD", "CA", "GD", "GA", "CR"];
    let mut attackers = vec![];
    let mut defenders = vec![];
    let mut king = (state.sizen * state.sizen)+1;
    for (pos, piece) in &state.board{
        match piece{
            Piece::Attacker => {
                attackers.push(*pos);
            }
            Piece::Defender => {
                defenders.push(*pos);
            }
            Piece::King => {
                king = *pos;
            }
            // _ =>{println!("Sorry, this board size doesn't have a data table!"); panic!("Board size requsted for which no default game state has been implemented.");}
        }
    }
    assert_ne!(king, (state.sizen * state.sizen)+1);

    let king_vec = vec![king];
    let mut edgelist = vec![];
    for i in 1..(state.sizen * state.sizen){
        if i%state.sizen== 0 || i%state.sizen== state.sizen -1 || i<state.sizen || i>state.sizen*(state.sizen -1){
            edgelist.push(i);
        }
    }

    let mb = attackers.len() as f32 / defenders.len() as f32;
    let n = attackers.len() as f32 + defenders.len() as f32;
    let fc = coordination(&king_vec, 0, state.sizen);
    let fe = cmp::min(cmp::min(king % state.sizen, state.sizen -1 - king % state.sizen),cmp::min(king/state.sizen, state.sizen -1 - king/state.sizen)) as f32;
    let mc = moves_to_goal(state, &state.corners);
    let me = moves_to_goal(state, &edgelist);
    let md = mobility(state, &defenders);
    let ma = mobility(state, &attackers);
    let cd = coordination(&defenders, king, state.sizen);
    let ca = coordination(&attackers, king, state.sizen);
    let gd = grouping(&defenders, state.sizen);
    let ga = grouping(&attackers, state.sizen);
    let frc = f_r_control(&attackers, &defenders, state.sizen);
    let cr = corners_reachable(state, &attackers);
    let evals = [mb,n,frc,fc,fe,mc,me,md,ma,cd,ca,gd,ga, cr];
    let mut eval: f32 = 0.0;
    //println!("factor: eval * weight = result");
    for i in 0..evals.len(){
        //println!("{}: {}*{} = {}",legend[i], evals[i],weights[i],evals[i]* weights[i]);
        eval += evals[i]* weights[i];
    }
    eval
}

fn corners_reachable(state: &GameState, a_list: &Vec<u8>) -> f32{
    let mut new_board : HashMap<u8, Piece> = HashMap::new();
    for i in a_list{
        new_board.insert(*i, Piece::Attacker);
    }
    let goallist = &state.corners;
    let mut king = state.sizen * state.sizen;
    let mut corners_reachable = vec![];
    for (pos, piece) in &state.board{
        if *piece == Piece::King{
            king = *pos;
            break;
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back((king, 'c')); //'c' is just some cahrachte != vh
    let mut bin = vec![];
    while !queue.is_empty(){
        let curr = queue.pop_front().expect("This shouldn't be possible!");
        if goallist.contains(&curr.0){
            if !corners_reachable.contains(&curr.0){
                corners_reachable.push(curr.0);
            }
        }
        else{
            if curr.1 != 'h'{
                let list = get_moves_with_corners(&new_board, state.throne,state.sizen, curr.0, 'h');
                for i in list{
                    let element = (i, 'h');
                    if !queue.contains(&element) && !bin.contains(&element){
                        queue.push_back(element);
                    }
                }
            }
            if curr.1 != 'v'{
                let list = get_moves_with_corners(&new_board, state.throne,state.sizen, curr.0, 'v');
                for i in list{
                    let element = (i, 'v');
                    if !queue.contains(&element) && !bin.contains(&element){
                        queue.push_back(element);
                    }
                }
            }
            bin.push(curr);
        }
    }
    corners_reachable.len() as f32
}

fn mobility(state: &GameState, piece_list:  &Vec<u8>) -> f32{
    let mut counter = 0;
    for i in piece_list{
        let h = get_moves(&state.board, &state.corners, state.throne, state.sizen, *i, 'h');
        let v = get_moves(&state.board, &state.corners, state.throne, state.sizen, *i, 'v');
        counter += h.len()+ v.len();
    }
    counter as f32
}

fn moves_to_goal(state: &GameState, goallist: &[u8]) -> f32{
    let mut king = state.sizen * state.sizen;
    for (pos, piece) in &state.board{
        if *piece == Piece::King{
            king = *pos;
            break;
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back((king, 0, 'c')); //'c' is just some cahrachte != vh
    let mut queue_check = VecDeque::new();
    queue_check.push_back((king, 'c'));
    let mut bin = vec![];
    while !queue.is_empty(){
        let curr = queue.pop_front().expect("This shouldn't be possible!");
        let curr_c = queue_check.pop_front().expect("This shouldn't be possible!");
        if goallist.contains(&curr.0){
            return curr.1 as f32
        }
        else{
            if curr.2 != 'h'{
                let list = get_moves_with_corners(&state.board, state.throne,state.sizen, curr.0, 'h');
                for i in list{
                    let element = (i, curr.1 + 1, 'h');
                    let element_c = (i, 'h');
                    if !queue_check.contains(&element_c) && !bin.contains(&element_c){
                        queue.push_back(element);
                        queue_check.push_back(element_c);
                    }
                }
            }
            if curr.2 != 'v'{
                let list = get_moves_with_corners(&state.board, state.throne,state.sizen, curr.0, 'v');
                for i in list{
                    let element = (i, curr.1 + 1, 'v');
                    let element_c = (i, 'v');
                    if !queue_check.contains(&element_c) && !bin.contains(&element_c){
                        queue.push_back(element);
                        queue_check.push_back(element_c);
                    }
                }
            }
            bin.push(curr_c);
        }
    }
    f32::INFINITY
}

fn coordination(vec: &Vec<u8>, king: u8, size: u8) -> f32{
    let king_x = king % size;
    let king_y = (king - king_x)/size;
    let mut sum: u8 = 0;
    for i in vec{
        let i_x = i % size;
        let i_y = (i -i_x)/size;
        let dist_corner = cmp::min(i_x, size -1 - i_x) + cmp::min(i_y, size -1 - i_y);
        let dist_king = ((i_x as i16 -king_x as i16).abs() + (i_y as i16 -king_y as i16).abs()) as u8;
        sum += cmp::min(dist_corner, dist_king);
    }
    sum as f32
}

fn grouping(vec: &Vec<u8>, size: u8) -> f32{
    let mut sum: u8 = 0;
    for i in vec{
        let i_x = i % size;
        let i_y = (i -i_x)/size;
        let mut min = size * size;
        for j in vec{
            if i != j{
                let j_x = j % size;
                let j_y = (j-j_x)/size;
                min = cmp::min(min, ((i_x as i16-j_x as i16).abs() + (i_y as i16-j_y as i16).abs())as u8);
            }
        }
        sum += min;
    }
    sum as f32
}

fn f_r_control(a_list:  &Vec<u8>, d_list:  &Vec<u8>, size: u8) -> f32{ //def max
    let mut board = vec![vec![0; size as usize];size as usize];
    let mut sum = 0;
    for i in a_list{
        let i_x = i % size;
        let i_y = (i -i_x)/size;
        board[i_y as usize][i_x as usize] = -1;
    }
    for i in d_list{
        let i_x = i % size;
        let i_y = (i -i_x)/size;
        board[i_y as usize][i_x as usize] = 1;
    }
    for j in 0..size{ //files
        let mut first = 0;
        let mut last = 0;
        for k in 0..size{
            let val = board[j as usize][k as usize];
            if  val != 0{
                last = val;
                if first == 0{
                    first = val;
                }
            }
        }
        sum += first + last;
    }
    for j in 0..size{ //files
        let mut first = 0;
        let mut last = 0;
        for k in 0..size{
            let val = board[k as usize][j as usize];
            if  val != 0{
                last = val;
                if first == 0{
                    first = val;
                }
            }
        }
        sum += first + last;
    }
    (sum as f32)/2.0
}

fn calc_weights(state: &GameState, signs: &[f32]) -> Vec<f32>{
    let mut mb: f32 = 0.0;
    let mut n: f32 = 0.0;
    let mut frc: f32 = 0.0;
    let fc: f32 = (state.sizen - 1) as f32;
    let fe: f32 = (state.sizen - 1) as f32 / 2.0;
    let mc: f32 = (state.sizen - 1) as f32 / 2.0;
    let me: f32 = (state.sizen - 1) as f32 / 4.0;
    let mut md: f32 = 0.0;
    let mut ma: f32 = 0.0;
    let mut cd: f32 = 0.0;
    let mut ca: f32 = 0.0;
    let mut gd: f32 = 0.0;
    let mut ga: f32 = 0.0;
    let cco: f32 = 4.0;
    let _jsduhf= mb *n*frc*md*ma*cd*ca*gd*ga; //just to get rid of stupid warnings ;)
    match state.sizen {
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

fn get_moves(board: &HashMap<u8, Piece>, corners: &[u8], throne: u8, size: u8, piece: u8, c: char) -> Vec<u8>{
    let mut moves = vec![];
    match c {
        'h' => {
            let constant = piece / size;
            let mut curr = piece +1;
            while curr / size == constant && board.get(&curr).is_none() && !corners.contains(&curr){
                if curr != throne{
                    moves.push(curr);
                }
                curr += 1;
            }
            curr = piece -1;
            while curr / size == constant && board.get(&curr).is_none() && !corners.contains(&curr){
                if curr != throne{
                    moves.push(curr);
                }
                if curr == 0{
                    break;
                }
                curr -= 1;
            }
        }
        'v' =>{
            let constant = size * size;
            let mut curr = piece + size;
            while curr < constant && board.get(&curr).is_none() && !corners.contains(&curr){
                if curr != throne{
                    moves.push(curr);
                }
                curr += size;
            }
            if piece > size{
                curr = piece - size;
                while board.get(&curr).is_none() && !corners.contains(&curr){
                    if curr != throne{
                        moves.push(curr);
                    }
                    if curr < size{
                        break;
                    }
                    curr -= size;
                }
            }
        }
        _ =>{println!("Problem in get_moves"); panic!("Board size requsted for which no default game state has been implemented.");}
    }
    moves
}

fn get_moves_with_corners(board: &HashMap<u8, Piece>,  throne: u8, size: u8, piece: u8, c: char) -> Vec<u8>{
    let mut moves = vec![];
    match c {
        'h' => {
            let constant = piece / size;
            let mut curr = piece +1;
            while curr / size == constant && board.get(&curr).is_none() {
                if curr != throne{
                    moves.push(curr);
                }
                curr += 1;
            }
            curr = piece -1;
            while curr / size == constant && board.get(&curr).is_none() {
                if curr != throne{
                    moves.push(curr);
                }
                if curr == 0{
                    break;
                }
                curr -= 1;
            }
        }
        'v' =>{
            let constant = size * size;
            let mut curr = piece + size;
            while curr < constant && board.get(&curr).is_none() {
                if curr != throne{
                    moves.push(curr);
                }
                curr += size;
            }
            if piece > size{
                curr = piece - size;
                while board.get(&curr).is_none() {
                    if curr != throne{
                        moves.push(curr);
                    }
                    if curr < size{
                        break;
                    }
                    curr -= size;
                }
            }
        }
        _ =>{println!("Problem in get_moves"); panic!("Board size requsted for which no default game state has been implemented.");}
    }
    moves
}