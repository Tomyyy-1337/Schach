use std::time::{Duration, SystemTime};
use rayon::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Clone, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(PartialEq)]
pub enum Outcome {
    Checkmate(Color),
    Stalemate,
    None,
}

#[derive(Clone)]
pub struct Schach {
    pub active_player: Color,
    black_pawns: u64,
    white_pawns: u64,
    black_king:  u64,
    white_king:  u64,
    black_queen: u64,
    white_queen: u64,
    black_bishops : u64,
    white_bishops: u64,
    black_knights: u64,
    white_knights: u64,
    black_rooks: u64,
    white_rooks: u64,
    castle: u64,
    en_passant: Option<(i32,i32)>,
    fifty_move: u32,
}

impl Schach {
    pub fn new() -> Self {
        Schach {
            active_player: Color::White,
            black_pawns   : 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            white_pawns   : 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            black_king    : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            white_king    : 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_queen   : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            white_queen   : 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_bishops : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            white_bishops : 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_knights : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            white_knights : 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_rooks   : 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            white_rooks   : 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            castle        : 0b10010001_00000000_00000000_00000000_00000000_00000000_00000000_10010001,
            en_passant    : None,
            fifty_move    : 0, 
        }
    }

    fn get_all_legal_moves(&self) -> Vec<(u64, u64, u64, u64)> {
        let mut result: Vec<(u64, u64, u64, u64)> = Vec::new();
        
        for x in 0..8 {
            for y in 0..8 {
                let mut m: Vec<(u64, u64, u64, u64)> = self.get_legal_moves(x, y, 1).iter().map(| (a,b) | (x,y,*a as u64, *b as u64)).collect();
                result.append(&mut m);
            }
        }
        result.shuffle(&mut thread_rng());
        result
    }


    pub fn minmax(&mut self, depth:u64, mut alpha: f32, mut beta: f32, maximizing_player: bool ) -> f32 {
        let mut brett = self.clone();
        brett.active_player = match brett.active_player {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
        if depth == 0 {
            return self.eval_position();
        }

        if maximizing_player {
            let mut max_eval = f32::NEG_INFINITY;
            for (a,b,c,d) in self.get_all_legal_moves() {
                let mut brett = self.clone();
                brett.move_piece(a, b, c, d);
                match brett.get_outcome() { Outcome::None => () , _ => return brett.eval_position(),}
                let eval = brett.minmax(depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            } 
            return max_eval;
        } else {
            let mut min_eval = f32::INFINITY;
            for (a,b,c,d) in self.get_all_legal_moves() {
                let mut brett = self.clone();
                brett.move_piece(a, b, c, d);
                let eval = brett.minmax(depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            return min_eval;
        } 
    }   


    pub fn best_move(&self, depth: u64, start: SystemTime) -> (u64,u64,u64,u64) {
        // println!("Suchtiefe: {}",&depth);
        let mut best = f32::MIN;
        
        let maximizing_player = match self.active_player {
            Color::Black => true,
            Color::White => false,
        };
        let factor = match self.active_player {
            Color::Black => -1.0,
            Color::White =>  1.0,
        };
        let all_moves = self.get_all_legal_moves();
        let mut best_move = all_moves[0];
        let mut moves: Vec<(f32,u64,u64,u64,u64)> = Vec::new();
        all_moves.par_iter()
            .map(|(a,b,c,d)| {
                let mut brett = self.clone();
                brett.move_piece(*a, *b, *c, *d);
                let eval = factor *  brett.minmax(depth, f32::NEG_INFINITY, f32::INFINITY, maximizing_player);
                (eval ,*a,*b,*c,*d)
            }).collect_into_vec(&mut moves);

        if SystemTime::now() < start + Duration::new(0,1_000_000_000/5) {
            return self.best_move(depth+1, start);
        }
            
        for (f,a,b,c,d) in moves {
            if f > best {
                best = f;
                best_move = (a,b,c,d);
            }
        }
        
        best_move
    }

    pub fn eval_position(&self) -> f32 {
        match self.get_outcome() {
            Outcome::Checkmate(Color::White) => return 1000.0,
            Outcome::Checkmate(Color::Black) => return -1000.0,
            Outcome::Stalemate => return 0.0,
            Outcome::None => (),
        }
        let mut eval:f32 = 0.0;
        for (c,p,_, _) in self.get_positions() {
            let value = match (&c,&p) {
                (Color::White, Piece::King) => 0.0,
                (Color::White, Piece::Queen) => 9.5,
                (Color::White, Piece::Rook) => 5.63,
                (Color::White, Piece::Bishop) => 3.33,
                (Color::White, Piece::Knight) => 3.05,
                (Color::White, Piece::Pawn) => 1.0,
                (Color::Black, Piece::King) => 0.0,
                (Color::Black, Piece::Queen) => -9.5,
                (Color::Black, Piece::Rook) => -5.63,
                (Color::Black, Piece::Bishop) => -3.33,
                (Color::Black, Piece::Knight) => -3.05,
                (Color::Black, Piece::Pawn) => -1.0,
            };
            eval += value;
        }
        eval 
    }

    pub fn move_piece(&mut self, from_x: u64, from_y: u64, to_x: u64, to_y: u64) {
        self.en_passant = None;
        self.fifty_move += 1;

        if let Some((p_f,c_f)) = self.get_piece_at(from_x, from_y) {
            // Update En-Passant und 50 move
            if p_f == Piece::Pawn {
                self.fifty_move = 0;
                if (to_y as i32 - from_y as i32).abs() == 2 {
                    self.en_passant = match c_f { 
                        Color::White => Some((to_x as i32, to_y as i32 + 1)),
                        Color::Black => Some((to_x as i32, to_y as i32 - 1)),
                    };
                }
            }

            // Update castle
            if (p_f == Piece::Rook || p_f == Piece::King) && self.castle >> (from_x + 8 * from_y) & 1 == 1  {
                self.castle -= 1 << (from_x + 8 * from_y);
            }

            if let Some((p_t,c_t)) = self.get_piece_at(to_x, to_y) {
                self.fifty_move = 0;
                if p_f == Piece::Pawn && (to_y == 7 || to_y == 0) { 
                    self.remove_piece_at(&p_t, &c_t, to_x, to_x);
                    self.set_piece(&Piece::Queen, &c_f, to_x, to_y);
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                } else {
                    self.remove_piece_at(&p_t, &c_t, to_x, to_y);
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                }
            } else {
                if p_f == Piece::Pawn && (to_y == 7 || to_y == 0) { 
                    self.set_piece(&Piece::Queen, &c_f, to_x, to_y);
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                }
                // Castle Ausführen 
                 else if p_f == Piece::King && (to_x as i32 - from_x as i32).abs() == 2 {
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                    if to_x as i32 - from_x as i32 == 2 {
                        self.set_piece(&Piece::Rook, &c_f, to_x - 1, to_y);
                        self.remove_piece_at(&Piece::Rook, &c_f, from_x + 3, from_y);
                    } else {
                        self.set_piece(&Piece::Rook, &c_f, to_x + 1, to_y);
                        self.remove_piece_at(&Piece::Rook, &c_f, from_x - 4, from_y);
                    }
                }
                else if p_f == Piece::Pawn && (to_x as i32 - from_x as i32).abs() == 1 {
                    // En-passant zug ausführen
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                    match c_f {
                        Color::White => self.remove_at(to_x, to_y + 1),
                        Color::Black => self.remove_at(to_x, to_y - 1),
                    }
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                } else {
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                    self.remove_piece_at(&p_f, &c_f, from_x, from_y);
                }
            }

            // Update active player
            self.active_player = match self.active_player {
                Color::White => Color::Black,
                Color::Black => Color::White,
            } 
        }
    }

    pub fn get_positions(&self) -> Vec<(Color, Piece, u64, u64)> {
        let mut result = Vec::new();
        let pieces = [(Color::White, Piece::King), (Color::White, Piece::Queen), (Color::White, Piece::Rook), (Color::White, Piece::Bishop), (Color::White, Piece::Knight), (Color::White, Piece::Pawn), (Color::Black, Piece::King), (Color::Black, Piece::Queen), (Color::Black, Piece::Rook), (Color::Black, Piece::Bishop), (Color::Black, Piece::Knight), (Color::Black, Piece::Pawn),];
        let bitboards = [self.white_king, self. white_queen, self.white_rooks, self.white_bishops, self.white_knights, self.white_pawns, self.black_king, self.black_queen, self.black_rooks, self.black_bishops, self.black_knights, self.black_pawns];

        for x in 0..8 {
            for y in 0..8 {
                for i in 0..12 {
                    let pos = x + 8 * y;
                    if bitboards[i] >> pos & 1 == 1  {
                        result.push((pieces[i].0.clone(), pieces[i].1.clone(), x, y));
                        break;
                    }
                }
            } 
        }
        result
    }

    pub fn get_outcome(&self) -> Outcome {
        if self.fifty_move >= 50 ||  0 == self.white_bishops + self.white_queen + self.white_rooks + self.white_pawns + self.black_bishops + self.black_queen + self.black_rooks + self.black_pawns {
            return Outcome::Stalemate;
        }

        let attacked_squares = self.atacked_squares_bitmap();
        let aktive_king = match self.active_player { Color::White => self.white_king, Color::Black => self.black_king};
        let opponent = match self.active_player { Color::White => Color::Black, Color::Black => Color::White};
        let is_check = attacked_squares & aktive_king != 0;
        let mut legal_moves = 0;
        for x in 0..8 {
            for y in 0..8 {
                legal_moves += self.get_legal_moves(x, y, 1).len();
            }
        }

        if legal_moves == 0 {
            if is_check {
                Outcome::Checkmate(opponent)
            } else {
                Outcome::Stalemate
            }
        } else {
            Outcome::None
        }
    }

    fn log2(&self, x: u64) -> i32 {
        63 - x.leading_zeros() as i32
    }

    pub fn get_king_pos(&self, c: &Color) -> (i32,i32) {
        let bin_log = match c {
            Color::White => self.log2(self.white_king),
            Color::Black => self.log2(self.black_king),
        };
        (bin_log % 8, bin_log / 8)
    }

    fn is_valid_move(&self, c: &Color, from_x: u64, from_y: u64, to_x: u64, to_y:u64) -> bool {
        let mut brett = self.clone();
        brett.move_piece(from_x, from_y, to_x, to_y);
        let (king_x, king_y) = brett.get_king_pos(c);

        let dirs = [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,-1),(-1,1)];
        for (d_x,d_y) in dirs {
            let mut x = king_x + d_x;
            let mut y = king_y + d_y;
            while x >= 0 && y >= 0 && x < 8 && y < 8 {
                let moves = brett.get_legal_moves(x as u64, y as u64, 0);
                if moves.contains(&(king_x, king_y)) {
                    return false;
                }
                x += d_x;
                y += d_y; 
            }           
        }
        let dirs = [(2,1),(2,-1),(1,2),(-2,1),(-2,-1),(-1,2),(1,-2),(-1,-2)];
        for (d_x,d_y) in dirs {
            let x = king_x + d_x;
            let y = king_y + d_y;
            if x >= 0 && y >= 0 && x < 8 && y < 8 {
                let moves = brett.get_legal_moves(x as u64, y as u64, 0);
                if moves.contains(&(king_x, king_y)) {
                    return false;
                }
            }           
        }
        true
    }

    // Gibt eine bitmap der vom gegner attakierten Felder zurück
    fn atacked_squares_bitmap(&self) -> u64 {
        let mut brett = self.clone();
        let mut bitmap = 0;

        match self.active_player {
            Color::White => brett.active_player = Color::Black,
            Color::Black => brett.active_player = Color::White,
        }

        for x in 0..8 {
            for y in 0..8 {
                let moves = brett.get_legal_moves(x, y, 0);
                for (x_m,y_m) in moves {
                    bitmap |= 1 << (x_m + 8 * y_m) as u64;
                }
            }
        }
        bitmap
    }

    pub fn get_legal_moves(&self, x: u64, y: u64, tiefe: u8) -> Vec<(i32, i32)> {
        let piece = self.get_piece_at(x, y);
        if let Some((_,c)) = &piece {
            if *c != self.active_player {
                return Vec::new();
            }
        }
        match piece {
            Some((Piece::Queen , c)) => self.generate_moves(&c, x, y, 7, tiefe, vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]),
            Some((Piece::Bishop, c)) => self.generate_moves(&c, x, y, 7, tiefe, vec![(-1,-1), (-1,1), (1,-1),  (1,1)]),
            Some((Piece::Knight, c)) => self.generate_moves(&c, x, y, 1, tiefe, vec![(-1,-2), (-1,2), (1,-2), (1,2), (2,1), (2,-1), (-2,1), (-2,-1)]),
            Some((Piece::Rook  , c)) => self.generate_moves(&c, x, y, 7, tiefe, vec![(-1,0), (0,-1), (0,1), (1,0)]),
            Some((Piece::King  , c)) => {
                let mut result = self.generate_moves(&c, x, y, 1, tiefe, vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]);
                if tiefe > 0 && self.castle >> (x + 8 * y) & 1 == 1 {
                    let collision = self.white_bishops + self.white_knights + self.white_queen + self.white_rooks + self.white_pawns + self.black_bishops + self.black_knights + self.black_queen + self.black_rooks + self.black_pawns;
                    match c {
                        Color::White => {
                            let path     = 0b00001110_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
                            let king_path:u64 = 0b00011100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
                            let attacked = self.atacked_squares_bitmap();
                            if  collision & path == 0 && king_path & attacked == 0 && self.castle >> (x - 4 + 8 * y) & 1 == 1 {
                                if tiefe == 0 || self.is_valid_move(&c, x, y, x-2, y) {
                                    result.push((x as i32 - 2, y as i32));
                                }
                            }
                            let path     = 0b01110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
                            if collision & path == 0 && path & attacked == 0 && self.castle >> (x + 3 + 8 * y) & 1 == 1 {
                                if tiefe == 0 || self.is_valid_move(&c, x, y, x + 2, y) {
                                    result.push((x as i32 + 2, y as i32));
                                }
                            }
                        },
                        Color::Black => {
                            let path     = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001110;
                            let king_path:u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00011100;
                            let attacked = self.atacked_squares_bitmap();
                            if  collision & path == 0 && king_path & attacked == 0 && self.castle >> (x - 4 + 8 * y) & 1 == 1 {
                                if tiefe == 0 || self.is_valid_move(&c, x, y, x - 2 , y) {
                                    result.push((x as i32 - 2, y as i32));
                                }
                            }
                            let path     = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01110000;
                            if collision & path == 0 && path & attacked == 0 && self.castle >> (x + 3 + 8 * y) & 1 == 1 {
                                if tiefe == 0 || self.is_valid_move(&c, x, y, x, y) {
                                    result.push((x as i32 + 2, y as i32));
                                }
                            }
                        },
                    }
                }
                
                result
            },
            Some((Piece::Pawn  , c)) => {
                let mut result = Vec::new();
                let direction = match c { Color::White => -1, Color::Black => 1 };
                let a = x as i32;
                let mut b = y as i32 + direction;
                if b < 8 && b >= 0 && self.get_piece_at(a as u64, b as u64).is_none() {
                    if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                        result.push((a, b));
                    }
                    if (c == Color::White && y == 6) || (c == Color::Black && y == 1) {
                        b += direction;
                        if b < 8 && b >= 0 && self.get_piece_at(a as u64, b as u64).is_none() {
                            if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                                result.push((a, b));
                            }
                        }
                    }
                }
                for (a,b) in [(x as i32 + direction, y as i32 + direction), (x as i32 - direction, y as i32 + direction)] {
                    if a >= 0 && a < 8 && b >= 0 && b < 8 {
                        if let Some((_,c_d)) = self.get_piece_at(a as u64, b as u64) {
                            if c != c_d {
                                if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                                    result.push((a, b));
                                }
                            }
                        }
                    }
                }
                if let Some(pos) = self.en_passant {
                    let b = match c { Color::White => y as i32 - 1, Color::Black => y as i32 + 1 };
                    for a in [x as i32 + 1, x as i32 - 1] {
                        if (a,b) == pos {
                            if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                                result.push((a,b));
                            }
                        } 
                    }
                }
                result
            },
            None => Vec::new(),
        }
    }

    fn generate_moves(&self,c: &Color, x: u64, y:u64 ,range: u8, tiefe: u8, moves: Vec<(i32,i32)>) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        for m in moves {
            let mut a = x as i32;
            let mut b = y as i32;
            for _ in 0..range {
                a += m.0;
                b += m.1;
                if a >= 8 || b >= 8 || a < 0 || b < 0 {
                    break;
                }
                if let Some((_, p_c)) = self.get_piece_at(a as u64, b as u64) {
                    if p_c != *c {
                        if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                            result.push((a, b));
                        }
                    } 
                    break;
                } 
                if tiefe == 0 || self.is_valid_move(&c, x, y, a as u64, b as u64) {
                    result.push((a, b));
                }
            }  
        }
        result
    }
    
    fn set_piece(&mut self, p: &Piece, c: &Color, x: u64, y: u64) {        
        let position:u64 = 1 << (x + 8 * y);
    
        match (p, c) {
            (Piece::King, Color::White) =>   self.white_king |= position,
            (Piece::King, Color::Black) =>   self.black_king |= position,
            (Piece::Queen, Color::White) =>  self.white_queen |= position,
            (Piece::Queen, Color::Black) =>  self.black_queen |= position,
            (Piece::Rook, Color::White) =>   self.white_rooks |= position,
            (Piece::Rook, Color::Black) =>   self.black_rooks |= position,
            (Piece::Bishop, Color::White) => self.white_bishops |= position,
            (Piece::Bishop, Color::Black) => self.black_bishops |= position,
            (Piece::Knight, Color::White) => self.white_knights |= position,
            (Piece::Knight, Color::Black) => self.black_knights |= position,
            (Piece::Pawn, Color::White) =>   self.white_pawns |= position,
            (Piece::Pawn, Color::Black) =>   self.black_pawns |= position,
        }
    }

    fn remove_at(&mut self ,x: u64,y: u64) {
        let position:u64 = 1 << (x + 8 * y);
        if let Some((p,c)) = self.get_piece_at(x, y) {
            match (p, c) {
                (Piece::King, Color::White) =>   self.white_king -= position,
                (Piece::King, Color::Black) =>   self.black_king -= position,
                (Piece::Queen, Color::White) =>  self.white_queen -= position,
                (Piece::Queen, Color::Black) =>  self.black_queen -= position,
                (Piece::Rook, Color::White) =>   self.white_rooks -= position,
                (Piece::Rook, Color::Black) =>   self.black_rooks -= position,
                (Piece::Bishop, Color::White) => self.white_bishops -= position,
                (Piece::Bishop, Color::Black) => self.black_bishops -= position,
                (Piece::Knight, Color::White) => self.white_knights -= position,
                (Piece::Knight, Color::Black) => self.black_knights -= position,
                (Piece::Pawn, Color::White) =>   self.white_pawns -= position,
                (Piece::Pawn, Color::Black) =>   self.black_pawns -= position,
            }
        }
    }

    fn remove_piece_at(&mut self , piece: &Piece, color: &Color,x: u64,y: u64) {
        let position:u64 = 1 << (x + 8 * y);
        match (piece, color) {
            (Piece::King, Color::White)   => self.white_king    &= !position,
            (Piece::King, Color::Black)   => self.black_king    &= !position,
            (Piece::Queen, Color::White)  => self.white_queen   &= !position,
            (Piece::Queen, Color::Black)  => self.black_queen   &= !position,
            (Piece::Rook, Color::White)   => self.white_rooks   &= !position,
            (Piece::Rook, Color::Black)   => self.black_rooks   &= !position,
            (Piece::Bishop, Color::White) => self.white_bishops &= !position,
            (Piece::Bishop, Color::Black) => self.black_bishops &= !position,
            (Piece::Knight, Color::White) => self.white_knights &= !position,
            (Piece::Knight, Color::Black) => self.black_knights &= !position,
            (Piece::Pawn, Color::White)   => self.white_pawns   &= !position,
            (Piece::Pawn, Color::Black)   => self.black_pawns   &= !position,
        }   
    }


    fn get_piece_at(&self, x: u64, y: u64) -> Option<(Piece, Color)> {
        let pos = x + 8 * y;
        let pieces = [ (self.black_pawns, Piece::Pawn, Color::Black), (self.white_pawns, Piece::Pawn, Color::White), (self.black_bishops, Piece::Bishop, Color::Black), (self.white_bishops, Piece::Bishop, Color::White), (self.black_knights, Piece::Knight, Color::Black), (self.white_knights, Piece::Knight, Color::White), (self.black_rooks, Piece::Rook, Color::Black), (self.white_rooks, Piece::Rook, Color::White), (self.black_king, Piece::King, Color::Black), (self.white_king, Piece::King, Color::White), (self.black_queen, Piece::Queen, Color::Black), (self.white_queen, Piece::Queen, Color::White),];

        for (bitboard, piece, color) in pieces {
            if bitboard >> pos & 1 == 1 {
                return Some((piece, color));
            }
        }
        None
    }
}