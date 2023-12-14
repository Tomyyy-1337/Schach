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

#[derive(Clone)]
pub struct Schach {
    active_player: Color,
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
    en_passant: Option<(i32,i32)>,
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
            en_passant    : None,
        }
    }

    pub fn move_piece(&mut self, from_x: u64, from_y: u64, to_x: u64, to_y: u64) {
        self.en_passant = None;
        let from = self.get_piece_at(from_x, from_y);
        let to = self.get_piece_at(to_x, to_y);
        if let Some((p_f,c_f)) = from {
            if p_f == Piece::Pawn && (to_y as i32 - from_y as i32).abs() == 2 {
                self.en_passant = match c_f { 
                    Color::White => Some((to_x as i32, to_y as i32 + 1)),
                    Color::Black => Some((to_x as i32, to_y as i32 - 1)),
                }
            }
            match to {
                Some(_) => {
                    self.remove_piece(to_x, to_y);
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                    self.remove_piece(from_x, from_y);
                },
                None => {
                    if p_f == Piece::Pawn && (to_x as i32 - from_x as i32).abs() == 1 {
                        match c_f {
                            Color::White => self.remove_piece(to_x, to_y + 1),
                            Color::Black => self.remove_piece(to_x, to_y - 1),
                        }
                    }
                    self.set_piece(&p_f, &c_f, to_x, to_y);
                    self.remove_piece(from_x, from_y);
                },
            }
        }
        self.active_player = match self.active_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
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

    fn log2(&self, x: u64) -> u64 {
        63 - x.leading_zeros() as u64
    }

    pub fn get_king_pos(&self, c: &Color) -> (u64,u64) {
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

        for x in 0..8 {
            for y in 0..8 {
                let moves = brett.get_legal_moves(x, y, 0);
                if moves.contains(&(king_x as i32, king_y as i32)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_legal_moves(&self, x: u64, y: u64, tiefe: u8) -> Vec<(i32, i32)> {
        let piece = self.get_piece_at(x, y);
        if let Some((_,c)) = &piece {
            if *c != self.active_player {
                return Vec::new();
            }
        }
        match piece {
            Some((Piece::King  , c)) => self.generate_moves(c, x, y, 1, tiefe, vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]),
            Some((Piece::Queen , c)) => self.generate_moves(c, x, y, 7, tiefe, vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]),
            Some((Piece::Bishop, c)) => self.generate_moves(c, x, y, 7, tiefe, vec![(-1,-1), (-1,1), (1,-1),  (1,1)]),
            Some((Piece::Knight, c)) => self.generate_moves(c, x, y, 1, tiefe, vec![(-1,-2), (-1,2), (1,-2), (1,2), (2,1), (2,-1), (-2,1), (-2,-1)]),
            Some((Piece::Rook  , c)) => self.generate_moves(c, x, y, 7, tiefe, vec![(-1,0), (0,-1), (0,1), (1,0)]),
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

    fn generate_moves(&self,c: Color, x: u64, y:u64 ,range: u8, tiefe: u8, moves: Vec<(i32,i32)>) -> Vec<(i32, i32)> {
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
                    if p_c != c {
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
    
    pub fn set_piece(&mut self, p: &Piece, c: &Color, x: u64, y: u64) -> bool {        
        let position:u64 = 1 << (x + 8 * y);
        
        if let Some((_,c_tmp)) = self.get_piece_at(x, y) {
            if c_tmp == *c {
                return false;
            }
        }

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
        true
    }

    pub fn remove_piece(&mut self ,x: u64,y: u64) {
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

    pub fn get_piece_at(&self, x: u64, y: u64) -> Option<(Piece, Color)> {
        let pos = x + 8 * y;
        let pieces = [ (self.black_pawns, Piece::Pawn, Color::Black), (self.white_pawns, Piece::Pawn, Color::White), (self.black_bishops, Piece::Bishop, Color::Black), (self.white_bishops, Piece::Bishop, Color::White), (self.black_king, Piece::King, Color::Black), (self.white_king, Piece::King, Color::White), (self.black_knights, Piece::Knight, Color::Black), (self.white_knights, Piece::Knight, Color::White), (self.black_queen, Piece::Queen, Color::Black), (self.white_queen, Piece::Queen, Color::White), (self.black_rooks, Piece::Rook, Color::Black), (self.white_rooks, Piece::Rook, Color::White),];

        for (bitboard, piece, color) in pieces {
            if bitboard >> pos & 1 == 1 {
                return Some((piece, color));
            }
        }
        None
    }
}