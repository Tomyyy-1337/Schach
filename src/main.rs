fn main() {
    let mut brett = Schach::new();
    brett.init();

    println!("{:?}", brett.show());
}

#[derive(Debug)]
enum Color {
    White,
    Black
}

#[derive(Debug)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Debug)]
struct Schach {
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
}

impl Schach {
    pub fn new() -> Self {
        Schach {
            black_pawns: 0,
            white_pawns: 0,
            black_king: 0,
            white_king: 0,
            black_queen: 0,
            white_queen: 0,
            black_bishops : 0,
            white_bishops: 0,
            black_knights: 0,
            white_knights: 0,
            black_rooks: 0,
            white_rooks: 0,
        }
    }

    pub fn init(&mut self) {
        // Pawns
        self.set_piece(Piece::Pawn, Color::White, 0, 1);
        self.set_piece(Piece::Pawn, Color::White, 1, 1);
        self.set_piece(Piece::Pawn, Color::White, 2, 1);
        self.set_piece(Piece::Pawn, Color::White, 3, 1);
        self.set_piece(Piece::Pawn, Color::White, 4, 1);
        self.set_piece(Piece::Pawn, Color::White, 5, 1);
        self.set_piece(Piece::Pawn, Color::White, 6, 1);
        self.set_piece(Piece::Pawn, Color::White, 7, 1);
        self.set_piece(Piece::Pawn, Color::Black, 0, 6);
        self.set_piece(Piece::Pawn, Color::Black, 1, 6);
        self.set_piece(Piece::Pawn, Color::Black, 2, 6);
        self.set_piece(Piece::Pawn, Color::Black, 3, 6);
        self.set_piece(Piece::Pawn, Color::Black, 4, 6);
        self.set_piece(Piece::Pawn, Color::Black, 5, 6);
        self.set_piece(Piece::Pawn, Color::Black, 6, 6);
        self.set_piece(Piece::Pawn, Color::Black, 7, 6);

        // Rooks
        self.set_piece(Piece::Rook, Color::White, 0, 0);
        self.set_piece(Piece::Rook, Color::White, 7, 0);
        self.set_piece(Piece::Rook, Color::Black, 0, 7);
        self.set_piece(Piece::Rook, Color::Black, 7, 7);

        // Knights
        self.set_piece(Piece::Knight, Color::White, 1, 0);
        self.set_piece(Piece::Knight, Color::White, 6, 0);
        self.set_piece(Piece::Knight, Color::Black, 1, 7);
        self.set_piece(Piece::Knight, Color::Black, 6, 7);


        // Bishop
        self.set_piece(Piece::Bishop, Color::White, 2, 0);
        self.set_piece(Piece::Bishop, Color::White, 5, 0);
        self.set_piece(Piece::Bishop, Color::Black, 2, 7);
        self.set_piece(Piece::Bishop, Color::Black, 5, 7);

        // Queen
        self.set_piece(Piece::Queen, Color::White, 3, 0);
        self.set_piece(Piece::Queen, Color::Black, 3, 7);

        
        // King
        self.set_piece(Piece::King, Color::White, 4, 0);
        self.set_piece(Piece::King, Color::Black, 4, 7);


    }

    pub fn set_piece(&mut self, p: Piece, c: Color, x: u64, y: u64) {        
        let base:u64 = 2;
        let position:u64 = base.pow((x + 8 * y) as u32);
        
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

    pub fn remove_piece(&mut self, p: Piece, c: Color ,x: u64,y: u64) {
        let base:u64 = 2;
        let position:u64 = base.pow((x + 8 * y) as u32);
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

    pub fn get_position(&self, x: u64, y: u64) -> Option<(Piece, Color)> {
        let pos = x + 8 * y;
        if self.black_pawns >> pos & 1 == 1  {
            return Some((Piece::Pawn, Color::Black)); 
        }
        if self.white_pawns >> pos & 1 == 1 {
            return Some((Piece::Pawn, Color::White)); 
        }
        if self.black_bishops >> pos & 1  == 1 {
            return Some((Piece::Bishop, Color::Black)); 
        }
        if self.white_bishops >> pos & 1 == 1 {
            return Some((Piece::Bishop, Color::White)); 
        }
        if self.black_king >> pos & 1 == 1 {
            return Some((Piece::King, Color::Black)); 
        }
        if self.white_king >> pos & 1 == 1 {
            return Some((Piece::King, Color::White)); 
        }
        if self.black_knights >> pos & 1 == 1 {
            return Some((Piece::Knight, Color::Black)); 
        }
        if self.white_knights >> pos & 1 == 1 {
            return Some((Piece::Knight, Color::White)); 
        }
        if self.black_queen >> pos & 1 == 1 {
            return Some((Piece::Queen, Color::Black)); 
        }
        if self.white_queen >> pos & 1 == 1 {
            return Some((Piece::Queen, Color::White)); 
        }
        if self.black_rooks >> pos & 1 == 1 {
            return Some((Piece::Rook, Color::Black)); 
        }
        if self.white_rooks >> pos & 1 == 1 {
            return Some((Piece::Rook, Color::White)); 
        }
        None
    }

    pub fn show(&self) {
        let mut output = String::new();
        for y in (0..8).rev() {
            for x in 0..8 {
                match self.get_position(x, y) {
                    Some((Piece::King, Color::White)) =>   output += " K ",
                    Some((Piece::King, Color::Black)) =>   output += " k ",
                    Some((Piece::Queen, Color::Black)) =>  output += " q ",
                    Some((Piece::Queen, Color::White)) =>  output += " Q ",
                    Some((Piece::Rook, Color::White)) =>   output += " R ",
                    Some((Piece::Rook, Color::Black)) =>   output += " r ",
                    Some((Piece::Bishop, Color::White)) => output += " B ",
                    Some((Piece::Bishop, Color::Black)) => output += " b ",
                    Some((Piece::Knight, Color::White)) => output += " N ",
                    Some((Piece::Knight, Color::Black)) => output += " n ",
                    Some((Piece::Pawn, Color::White)) =>   output += " P ",
                    Some((Piece::Pawn, Color::Black)) =>   output += " p ",
                    None => output += "   ",
                }
            }
            output += "\n";
        }
        println!("{}",output);
    }

}