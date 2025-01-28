pub mod utility;
pub mod player;
pub mod game_organization;

use std::collections::HashMap;

const DEFAULT_7_A: [u8; 8] = [3,10,21,22,26,27,38,45];
const DEFAULT_7_D: [u8; 4] = [17,23,25,31];
const DEFAULT_9_A: [u8; 16] = [3,4,5,13,27,36,37,45,35,43,44,53,75,76,77,67];
const DEFAULT_9_D: [u8; 8] = [22,31,38,39,41,42,49,58];
const DEFAULT_11_D: [u8; 12] = [38,48,49,50,58,59,61,62,70,71,72,82];
const DEFAULT_11_A: [u8; 24] = [4,5,6,15,17,27,44,45,53,54,55,57,63,65,66,67,75,76,93,103,105,114,115,116];
const DEFAULT_13_D: [u8; 16] = [45,56,60,70,71,72,81,83,85,87,96,97,98,108,112,123];
const DEFAULT_13_A: [u8; 32] = [4,5,6,7,8,18,20,32,52,64,65,66,76,77,78,80,88,90,91,92,102,103,104,116,136,148,150,160,161,162,163,164];

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Piece {
    Attacker,
    Defender,
    King
}

#[derive(Debug)]
#[derive(Clone)]//Apparently necessary to use VictoryCondition in an option?
pub enum VictoryCondition {
    KingCaptured,
    KingInCorner
}

#[derive(Clone)]
pub enum Direction {
    U,
    D,
    L,
    R
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::U, Direction::D, Direction::L, Direction::R];

#[derive(Clone)]
pub struct GameState {
    sizen: u8,
    turn: u32,
    board: HashMap<u8, Piece>,
    corners: Vec<u8>,
    throne: u8,
    victory: Option<VictoryCondition>
}

#[derive(Clone)]
pub struct MoveRequest {
    position: u8,
    direction: Direction,
    magnitude: u8
}

impl GameState {
    fn new(sizen: u8) -> Self {
        //Return a fresh game, given board size
        let turn: u32 = 1;
        let sizen: u8 = sizen;
        let corners: Vec<u8> = vec![0, sizen - 1, (sizen * sizen) - 1, (sizen * sizen) - sizen];
        let throne: u8 = ((sizen * sizen) - 1) / 2;
        let victory = None;
        let mut board: HashMap<u8, Piece> = HashMap::new();
        match sizen {
            7 => {
                for position in DEFAULT_7_A {
                    board.insert(position, Piece::Attacker);
                }
                for position in DEFAULT_7_D {
                    board.insert(position, Piece::Defender);
                }
                board.insert(throne, Piece::King);
            }
            9 => {
                for position in DEFAULT_9_A {
                    board.insert(position, Piece::Attacker);
                }
                for position in DEFAULT_9_D {
                    board.insert(position, Piece::Defender);
                }
                board.insert(throne, Piece::King);
            }
            11 => {
                for position in DEFAULT_11_A {
                    board.insert(position, Piece::Attacker);
                }
                for position in DEFAULT_11_D {
                    board.insert(position, Piece::Defender);
                }
                board.insert(throne, Piece::King);
            }
            13 => {
                for position in DEFAULT_13_A {
                    board.insert(position, Piece::Attacker);
                }
                for position in DEFAULT_13_D {
                    board.insert(position, Piece::Defender);
                }
                board.insert(throne, Piece::King);
            }
            _ => {println!("Sorry, this board size doesn't have a data table!"); panic!("Board size requsted for which no default game state has been implemented.");}
        }
        GameState{turn, board, sizen, corners, throne, victory}
    }

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

    fn validate_move(&self, request: &MoveRequest) -> bool {
        //For each direction, check an out of bounds condition, then if each tile in path is empty.
        let checked_piece = self.board.get(&request.position);
        if checked_piece.is_none() {println!("There's no piece there."); return false}//No piece at this location

        //Make sure player is moving their own piece.
        let player = self.turn % 2 == 1;
        if (checked_piece == Some(&Piece::Attacker)) != player {println!("That isn't your piece."); return false}

        //Make sure the piece is going to move
        if request.magnitude == 0 {println!("You have to actually MOVE a piece on your turn."); return false}

        let int_pos: u8;
        match request.direction {
            Direction::U => {
                if request.position < (self.sizen * request.magnitude) {println!("That move goes out of bounds."); return false}
                // let oob: i32 = (request.position as i32) - ((self.sizen * request.magnitude)as i32);
                // if oob < 0 {println!("That move goes out of bounds."); return false}//Movement goes out of bounds above board
                for i in 1..=request.magnitude {
                    if !self.board.contains_key(&(request.position - (self.sizen * i))) {continue} else {println!("There's a piece in the way!"); return false}
                }
                int_pos = request.position - (self.sizen * request.magnitude);
            },
            Direction::D => {
                let oob = request.position + (self.sizen * request.magnitude);
                if oob > ((self.sizen * self.sizen) - 1) {println!("That move goes out of bounds!"); return false} //Movement goes out of bounds beneath the board
                for i in 1..=request.magnitude {
                    if !self.board.contains_key(&(request.position + (self.sizen * i))) {continue} else {println!("There's a piece in the way."); return false}
                }
                int_pos = request.position + (self.sizen * request.magnitude);
            },
            Direction::L => {
                if request.position < request.magnitude || (request.position/self.sizen) > ((request.position - request.magnitude)/self.sizen) {println!("That move goes out of bounds."); return false}
                // let oob: i32 = (request.position as i32) - (request.magnitude as i32);
                // if ((request.position as i32)/(self.sizen as i32) > oob/(self.sizen as i32)) || oob < 0 {println!("That move goes out of bounds."); return false} //Movement wraps past left bound
                for i in 1..=request.magnitude {
                    if !self.board.contains_key(&(request.position - i)) {continue} else {println!("There's a piece in the way."); return false}
                }
                int_pos = request.position - request.magnitude;
            },
            Direction::R => {
                let oob = request.position + request.magnitude;
                if oob/self.sizen > request.position/self.sizen {println!("That move goes out of bounds."); return false} //Movement wraps past right bound
                for i in 1..=request.magnitude {
                    if !self.board.contains_key(&(request.position + i)) {continue} else {println!("There's a piece in the way."); return false}
                }
                int_pos = request.position + request.magnitude;
            }
        }
        if *self.board.get(&request.position).unwrap() != Piece::King {
            if int_pos == self.throne || self.corners.contains(&int_pos) {println!("Only the King can occupy the throne and the corners."); return false}
        }
        true
    }

    fn piece_move(&mut self, movement: MoveRequest) -> Result<u8,&str> {
        //Checks if a move is valid, and if so, executes it.
        let new_location: u8;
        if self.validate_move(&movement) {
            //Move is valid, update board
            let token = self.board.remove(&movement.position).unwrap();
            match movement.direction {
                Direction::U => {
                    new_location = movement.position - (movement.magnitude * self.sizen);
                    self.board.insert(new_location, token);
                }
                Direction::D => {
                    new_location = movement.position + (movement.magnitude * self.sizen);
                    self.board.insert( new_location, token);
                }
                Direction::L => {
                    new_location = movement.position - movement.magnitude;
                    self.board.insert(new_location, token);
                }
                Direction::R => {
                    new_location = movement.position + movement.magnitude;
                    self.board.insert(new_location, token);
                }
            }
        }
        else {
            return Err("Provided move was found invalid by validate_move.")
        }
        Ok(new_location)
    }

    fn fq_game_update(&self, movement: &MoveRequest) -> GameState {
        //Updates a gamestate for a move request; ALGORITHMIC PLAYERS AND PRE-VALID MoveRequest ONLY
        //Also Removes captures, and increments turn
        let mut resultant_state = self.clone();
        let new_location: u8;

        let token = resultant_state.board.remove(&movement.position).unwrap();
        match movement.direction {
            Direction::U => {
                new_location = movement.position - (movement.magnitude * resultant_state.sizen);
                resultant_state.board.insert(new_location, token);
            }
            Direction::D => {
                new_location = movement.position + (movement.magnitude * resultant_state.sizen);
                resultant_state.board.insert( new_location, token);
            }
            Direction::L => {
                new_location = movement.position - movement.magnitude;
                resultant_state.board.insert(new_location, token);
            }
            Direction::R => {
                new_location = movement.position + movement.magnitude;
                resultant_state.board.insert(new_location, token);
            }
        }
        //Append things here
        if resultant_state.corners.contains(&new_location) {
            //Only Kings can access the corners
            resultant_state.victory = Some(VictoryCondition::KingInCorner);
        }
        let captures = resultant_state.capture_check(new_location);
        for index in captures {
            let capture = resultant_state.board.remove(&index);
            if capture == Some(Piece::King) {
                resultant_state.victory = Some(VictoryCondition::KingCaptured);
            }
            if capture.is_none() {
                println!("ERROR: Attempted and failed to remove piece at {}", utility::to_coord(&index, &resultant_state.sizen));
            }
        }
        resultant_state.turn += 1;
        resultant_state
    }

    pub fn capture_check(&self, new_location:u8) -> Vec<u8> {
        //Checks if any pieces have been captured, returns number of captured pieces
        let player = self.turn % 2 == 1;
        let mut besiegt: Vec<u8> = Vec::new();
        //Vector of all valid neighbor spaces
        let mut possibilities: Vec<(u8, Direction)> = Vec::new();
        for i in ALL_DIRECTIONS {
            let check: Option<(u8, Direction)> = self.get_neighbor(&new_location, i);
            if check.is_some() {
                possibilities.push(check.unwrap());
            }
        }

        //Keep occupied squares
        possibilities.retain(|x| self.board.contains_key(&x.0));

        // let mut opponents: Vec<(u8, Direction)> = possibilities.iter().map(|&x| (*self.board.get(&x.0).unwrap(),x.1.clone())).collect();
        possibilities.retain(|x| (self.board.get(&x.0) == Some(&Piece::Attacker)) != player);//Only keep opposing pieces
        
        if possibilities.is_empty() {return besiegt};

        for opponent in possibilities {
            let further = self.get_neighbor(&opponent.0, opponent.1);
            if further.is_none() {continue}//There is no further tile
            let space = further.unwrap();
            if !self.board.contains_key(&space.0) {
                //No player here; When it's a hostile space, capture anyway
                if self.corners.contains(&space.0) || space.0 == self.throne {besiegt.push(opponent.0)}
                continue
            }
            if (self.board.get(&space.0) == Some(&Piece::Attacker)) == player {besiegt.push(opponent.0)}//Save location of any captured pieces
        }
        besiegt
    }

    pub fn get_neighbor(&self, spot: &u8, direction: Direction) -> Option<(u8, Direction)> {
        //From a location, look in a direction, and return any "adjacent" tiles within game rules
        match direction {
            Direction::U =>{
                if *spot >= self.sizen {
                    Some(((spot - self.sizen), Direction::U))
                } else {
                    None
                }
            }
            Direction::D =>{
                if (spot + self.sizen) < (self.sizen * self.sizen) {
                    Some(((spot + self.sizen), Direction::D))
                } else {
                    None
                }
            }
            Direction::L =>{
                if *spot == 0 {return None}
                if (spot / self.sizen) == ((spot - 1) / self.sizen) {
                    Some(((spot - 1), Direction::L))
                } else {
                    None
                }
            }
            Direction::R =>{
                if (spot / self.sizen) == ((spot + 1) / self.sizen) {
                    Some(((spot + 1), Direction::R))
                } else {
                    None
                }
            }
        }
    }

    fn capture_pieces(&mut self, captures: Vec<u8>) -> Option<VictoryCondition> {
        //Removes pieces in provided locations.
        let mut king_captured = false;
        for index in captures {
            let capture = self.board.remove(&index);
            if capture.is_some() {
                println!("Great! Piece at {} was captured!", utility::to_coord(&index, &self.sizen));
                if capture == Some(Piece::King) {king_captured = true;}
            } else {
                println!("ERROR: Attempted and failed to remove piece at {}", utility::to_coord(&index, &self.sizen));
            }
        }
        if king_captured {
            self.victory = Some(VictoryCondition::KingCaptured);
            return Some(VictoryCondition::KingCaptured)
        }
        None
    }

    pub fn check_corners(&self) -> Option<VictoryCondition> {
        //Determine if the king has escaped into the corners
        for location in &self.corners {
            if self.board.contains_key(location) {
                assert!(self.board.get(location) == Some(&Piece::King));
                return Some(VictoryCondition::KingInCorner)
            }
        }
        None
    }

    fn end_game(&self) {
        //Print out information (For humans), save playout for algorithms.
        self.show_board();
        println!("Thanks for playing Hnefatafl! This game lasted {} turns.", {self.turn - 1});
    } 

    fn peek_row(&self, row: u8) -> Vec<Option<&Piece>> {
        //Return a Vector containing the contents of a selected row
        let row_offset = self.sizen * row;
        let mut my_row = Vec::new();
        for i in 0..self.sizen{
            my_row.push(self.board.get(&(row_offset + i)));
        }
        my_row
    }

    fn peek_col(&self, col:u8) -> Vec<Option<&Piece>> {
        //Returns a Vector containing the contents of a selected column
        let mut my_col = Vec::new();
        for i in 0..self.sizen {
            my_col.push(self.board.get(&(col + (i * self.sizen))));
        }
        my_col
    }
}