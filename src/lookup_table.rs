use std::collections::{HashMap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use fastrand;

#[derive(Clone)]
pub struct LookupTable {
    rook_move_mask: HashMap<u64, u64>,
    rook_moves: HashMap<u64, u64>,

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

        let mut rook_move_mask: HashMap<u64, u64> = HashMap::new();
        // iterate through all possible rook positions
        for square in 0..64 {
            let mut moves: u64 = 0;
            for target in 0..64 {
                if (target / 8 == square / 8) || (target % 8 == square % 8) && target != square {
                    moves |= 1 << target;
                }
            }
            rook_move_mask.insert(1 << square, moves);
        }

        let mut indexs: Vec<Vec<u64>> = Vec::new();
        for i in 0..64 {
            indexs.push(Vec::new());
            if rook_move_mask.get(&(1 << i)).unwrap() != &0 {
                indexs[i].push(1 << i);
            }
        }

        let mut rook_moves: HashMap<u64, u64> = HashMap::new();

        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for x in 0..8 {
            for y in 0..8 {
                let iterator = BitboardIterator::new(*rook_move_mask.get(&(1 << (x * 8 + y))).unwrap());
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
                    rook_moves.insert(bitboard, moves);
                }
            }
        }    

        (0..16).into_par_iter().for_each(|a| {
            let mut best_factor;
            let mut trash_bits = 28 + a;
            'outer: loop {
                let factor = fastrand::u64(1 << 63 .. u64::MAX);
                let mut indexset: HashSet<u64> = HashSet::new();
                for (k,_) in rook_moves.iter() {
                    if indexset.contains(&((k * &factor) >> trash_bits as u64)) {
                        continue 'outer;
                    } else {
                        indexset.insert((k * &factor) >> trash_bits as u64);
                    }
                }
                best_factor = factor;
                println!("factor: {}, trash_bits: {}", best_factor, trash_bits);
                trash_bits += 1;
                if trash_bits > 60 {
                    break;
                }
            }
        });
    
        LookupTable {
            rook_move_mask: rook_move_mask,
            rook_moves: rook_moves,
        }
    }

    pub fn get_rook_moves(&self, square: u64) -> u64 {
        *self.rook_moves.get(&square).unwrap()
    }

    pub fn get_rook_move_mask(&self, square: u64) -> u64 {
        *self.rook_move_mask.get(&square).unwrap()
    }
}
