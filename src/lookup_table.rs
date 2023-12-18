use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone)]
pub struct LookupTable {
    rook_move_mask: Vec<u64>,
    rook_moves1: Vec<u64>,
    rook_moves2: Vec<u64>,
    rook_moves3: Vec<u64>,
    rook_moves4: Vec<u64>,
    rook_moves5: Vec<u64>,
    rook_moves6: Vec<u64>,
    rook_moves7: Vec<u64>,
    rook_moves8: Vec<u64>,
    
}


impl LookupTable {
    pub fn new() -> Self {
        // contains all possible moves for the rook. The key is the bitboard with the rook's position, the value is the bitboard of all possible moves
        struct BitboardIterator {
            stack: Vec<(u64, usize)>,
        }
        
        impl BitboardIterator {
            fn new(bitboard: u64) -> Self {
                BitboardIterator {
                    stack: vec![(bitboard, 0)],
                }
            }
        }
        
        impl Iterator for BitboardIterator {
            type Item = u64;
            fn next(&mut self) -> Option<Self::Item> {
                while let Some((bitboard, start)) = self.stack.pop() {
                    for i in start..64 {
                        if (bitboard & (1 << i)) != 0 {
                            let new_bitboard = bitboard & !(1 << i);
                            self.stack.push((bitboard, i + 1));
                            self.stack.push((new_bitboard, i + 1));
                            return Some(bitboard);
                        }
                    }
                }
                None
            }
        }

        let mut rook_move_mask: Vec<u64> = Vec::new();
        // iterate through all possible rook positions
        for square in 0..64 {
            let mut moves: u64 = 0;
            for target in 0..64 {
                if (target / 8 == square / 8) || (target % 8 == square % 8) && target != square {
                    moves |= 1 << target;
                }
            }
            rook_move_mask.push(moves);
        }
        
        let mut rook_moves: HashMap<u64, u64> = HashMap::new();
        
        let filter:u64 = 0b00000000_01111110_01111110_01111110_01111110_01111110_01111110_00000000;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for x in 0..8 {
            for y in 0..8 {
                let iterator = BitboardIterator::new(rook_move_mask[x * 8 + y]);
                for bitboard in  iterator {   
                    let mut moves: u64 = 0;
                    for dir in dirs.iter() {
                        let mut i = x as i32 + dir.0;
                        let mut j = y as i32 + dir.1;
                        while i >= 0 && i < 8 && j >= 0 && j < 8 {
                            moves |= 1 << (i * 8 + j);
                            if bitboard & (1 << (i * 8 + j)) != 0 {
                                break;
                            }
                            i += dir.0;
                            j += dir.1;
                        }
                    }
                    rook_moves.insert(bitboard & filter, moves);
                }
            }
        }    

        // Magic number generation
        // (0..16).into_par_iter().for_each(|a| {
        //     let mut best_factor;
        //     let mut trash_bits = 41;
        //     'outer: loop {
        //         let factor = fastrand::u64(..);
        //         let mut indexset: HashSet<u64> = HashSet::new();
        //         for (k,_) in rook_moves.iter() {
        //             if k.count_zeros() % 8 != a % 8 {
        //                 continue;
        //             }
        //             if indexset.contains(&((k * &factor) >> trash_bits as u64)) {
        //                 continue 'outer;
        //             } else {
        //                 indexset.insert((k * &factor) >> trash_bits as u64);
        //             }
        //         }
        //         best_factor = factor;
        //         println!("a mod 8: {}, factor: {}, trash_bits: {}", a % 8, best_factor, trash_bits);
        //         trash_bits += 1;
        //         if trash_bits > 60 {
        //             break;
        //         }
        //     }
        // });


        let mut rook_moves1 = vec![0; 2usize.pow(20)];
        let mut rook_moves2 = vec![0; 2usize.pow(22)];
        let mut rook_moves3 = vec![0; 2usize.pow(23)];
        let mut rook_moves4 = vec![0; 2usize.pow(23)];
        let mut rook_moves5 = vec![0; 2usize.pow(22)];
        let mut rook_moves6 = vec![0; 2usize.pow(19)];
        let mut rook_moves7 = vec![0; 2usize.pow(15)];
        let mut rook_moves8 = vec![0; 2usize.pow(17)];

        let magic_numbers = [(8458573397933691365u64, 44u64), (8757177435623954480, 42), (9916889599927644802, 41), (15590192793224235728, 41), (5672935802224073713, 42), (7039174516308744092, 45), (71762459037376191, 49), (15625145245058179273, 47)];
        for (k,v) in rook_moves.iter() {
            let (factor, trash_bits) = magic_numbers[(k.count_zeros() % 8) as usize];
            let index = ((k * factor) >> trash_bits) as usize;
            match k.count_zeros() % 8 {
                0 => rook_moves1[index] = *v,
                1 => rook_moves2[index] = *v,
                2 => rook_moves3[index] = *v,
                3 => rook_moves4[index] = *v,
                4 => rook_moves5[index] = *v,
                5 => rook_moves6[index] = *v,
                6 => rook_moves7[index] = *v,
                7 => rook_moves8[index] = *v,
                _ => panic!("Index out of bounds"),
            }
        }    
        print!("Hai");
        Self {
            rook_move_mask,
            rook_moves1,
            rook_moves2,
            rook_moves3,
            rook_moves4,
            rook_moves5,
            rook_moves6,
            rook_moves7,
            rook_moves8,
        }
    }

    pub fn get_rook_moves(&self, position_map:u64, all_pieces: u64) -> u64 {
        let filter:u64 = 0b00000000_01111110_01111110_01111110_01111110_01111110_01111110_00000000;
        let relevant_pieces = all_pieces & self.get_rook_move_mask(63 - position_map.leading_zeros() as u64) & filter;
        let magic_index = relevant_pieces.count_zeros() % 8;
        let (magic_number, magic_shift) = match magic_index {
            0 => (8458573397933691365u64, 44u64),
            1 => (8757177435623954480, 42),
            2 => (9916889599927644802, 41),
            3 => (15590192793224235728, 41),
            4 => (5672935802224073713, 42),
            5 => (7039174516308744092, 45),
            6 => (71762459037376191, 49),
            _ => (15625145245058179273, 47),
        };
        let index = ((relevant_pieces * magic_number) >> magic_shift) as usize;
        match magic_index {
            0 => self.rook_moves1[index],
            1 => self.rook_moves2[index],
            2 => self.rook_moves3[index],
            3 => self.rook_moves4[index],
            4 => self.rook_moves5[index],
            5 => self.rook_moves6[index],
            6 => self.rook_moves7[index],
            _ => self.rook_moves8[index],
        }
    }

    pub fn get_rook_move_mask(&self, square: u64) -> u64 {
        self.rook_move_mask[63 - square.leading_zeros() as usize]
    }
}
