use std::num::{Int, SignedInt};
use std::iter::IteratorExt;

/// An 8 Puzzle board (or N puzzle board).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    board: Vec<i64>
}

fn usize_square_root(i: usize) -> usize {
    use std::num::Float;
    (i as f64).sqrt() as usize
}

impl Board {
    pub fn new(board: Vec<i64>) -> Board {
        if usize_square_root(board.len()).pow(2) != board.len() {
            panic!(format!("Provided board is not square! Size = {}", board.len()))
        }

        let mut copy_to_sort = board.clone();
        copy_to_sort[..].sort();
        for (i, b) in copy_to_sort.iter().enumerate() {
            if i as i64 != *b {
                panic!("Missing expected number {} in board!", i);
            }
        }

        Board { board: board }
    }

    pub fn random(n: usize) -> Board {
        Board {
            board: {
                use rand::{thread_rng, Rng};
                let mut v = (0 .. (n as i64).pow(2)).collect::<Vec<i64>>();
                thread_rng().shuffle(&mut v[..]);
                v
            }
        }
    }

    fn dimension(&self) -> usize {
        usize_square_root(self.board.len())
    }

    pub fn is_goal(&self) -> bool {
        for (i, b) in self.board.iter().enumerate() {
            if i < self.board.len() - 1 && *b != (i as i64) + 1 // all except end of board
                    || i == self.board.len() - 1 && *b != 0 { // end of board
                return false;
            }
        }
        true
    }

    /// Returns the hamming distance to the goal board
    pub fn hamming(&self) -> i64 {
        let mut dist = 0;
        for (i, b) in self.board.iter().enumerate() {
            if *b != (i as i64) + 1 && *b != 0 {
                dist += 1;
            }
        }
        dist
    }

    /// Returns the manhattan distance to the goal board
    pub fn manhattan(&self) -> i64 {
        let dim = self.dimension() as i64; // unsafe but we assume our boards aren't going to get too big
        let mut dist = 0;
        for (i, b) in self.board.iter().enumerate() {
            let iu64 = i as i64;
            if *b != iu64 + 1 && *b != 0 {
                // block is in wrong spot; find out where it's meant to be
                let curr_b_x = iu64 % dim;
                let curr_b_y= iu64 / dim;
                let goal_b_x = (*b - 1) % dim;
                let goal_b_y= (*b - 1) / dim;

                let i_dist = (curr_b_x - goal_b_x).abs() + (curr_b_y - goal_b_y).abs();
                // println!("In wrong spot: {} at {:?}, should be at {:?}, i_dist = {}",
                //         *b, (curr_b_x, curr_b_y), (goal_b_x, goal_b_y), i_dist);
                dist += i_dist;
            }
        }
        dist
    }

    /// Return a new board with the blocks at the given coordinates swapped
    fn create_swapped(&self, x1: i64, y1: i64, x2: i64, y2: i64) -> Board {
        let dim = self.dimension() as i64;
        let pos1 = y1 * dim + x1;
        let pos2 = y2 * dim + x2;
        let mut swapped_board = self.board.clone();
        swapped_board.swap(pos1 as usize, pos2 as usize);
        Board { board: swapped_board }
    }

    /// Return each board that can be made by swapping the zero with an adjacent block
    pub fn neighbours(&self) -> Vec<Board> {
        let dim = self.dimension() as i64;
        let zero_pos = self.board.iter().position(|b| *b == 0).unwrap() as i64;
        let x = zero_pos % dim;
        let y = zero_pos / dim;

        let mut vec = Vec::with_capacity(4);
        if x > 0 { // zero can move left
            vec.push(self.create_swapped(x, y, x - 1, y));
        }
        if x < dim - 1 { // zero can move right
            vec.push(self.create_swapped(x, y, x + 1, y));
        }
        if y > 0 { // zero can move up
            vec.push(self.create_swapped(x, y, x, y - 1));
        }
        if y < dim - 1 { // zero can move down
            vec.push(self.create_swapped(x, y, x, y + 1));
        }
        vec
    }

    pub fn twin(&self) -> Board {
        let dim = self.dimension() as i64;
        if dim < 2 {
            panic!("Boards of less than dimension 2 have no twin");
        }
        let (first_x, first_y) = if self.board[0] != 0 { (0, 0) } else { (1, 1) };
        let (second_x, second_y) = if self.board[1] != 0 { (1, 0) } else { (0, 1) };
        self.create_swapped(first_x, first_y, second_x, second_y)
    }
}

pub fn generate_all_boards_of_size(dim: i64) -> Vec<Board> {
    let base_board = (0 .. dim.pow(2)).collect::<Vec<i64>>();
    base_board[..].permutations().map(|perm_vec| Board::new(perm_vec)).collect()
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn random_board_has_correct_dimensions() {
        let n = 3; // standard 8-puzzle
        let b = Board::random(n);
        assert_eq!(b.dimension(), n)
    }

    #[test]
    fn goal_testing_works_correctly() {
        assert!(Board::new(vec![1,2,3,0]).is_goal());
        assert!(Board::new(vec![0]).is_goal());

        assert!(!Board::new(vec![1, 3, 2, 0]).is_goal());
        assert!(!Board::new(vec![0, 1, 2, 3]).is_goal());
    }

    #[test]
    fn hamming_distance() {
        assert_eq!(Board::new(
            vec![0, 1,
                 2, 3]).hamming(), 3); // all numbers in wrong position

        assert_eq!(Board::new(
            vec![1, 2,
                 0, 3]).hamming(), 1); // 3 is in the wrong position

        assert_eq!(Board::new(
            vec![1, 2,
                 3, 0]).hamming(), 0); // all numbers in correct position

        assert_eq!(Board::new(
            vec![8, 1, 3,
                 4, 0, 2,
                 7, 6, 5]).hamming(), 5); // provided example
    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(Board::new(
            vec![0, 1,
                 2, 3]).manhattan(), 4); // all numbers in wrong position (note that 2 needs to move twice)

        assert_eq!(Board::new(
            vec![1, 2,
                 0, 3]).manhattan(), 1); // 3 is in the wrong position

        assert_eq!(Board::new(
            vec![1, 2,
                 3, 0]).manhattan(), 0); // all numbers in correct position

        assert_eq!(Board::new(
            vec![8, 1, 3,
                 4, 0, 2,
                 7, 6, 5]).manhattan(), 10);
    }

    #[test]
    fn neighbours() {
        assert_eq!(Board::new(vec![
                0, 1,
                2, 3
            ]).neighbours(),
            vec![ // zero should move right and down
                Board::new(vec![
                    1, 0,
                    2, 3
                ]),
                Board::new(vec![
                    2, 1,
                    0, 3
                ])
            ]
        );

        assert_eq!(Board::new(vec![
                1, 2,
                3, 0
            ]).neighbours(),
            vec![ // zero should move left and up
                Board::new(vec![
                    1, 2,
                    0, 3
                ]),
                Board::new(vec![
                    1, 0,
                    3, 2
                ])
            ]
        ); // all numbers in wrong position (note that 2 needs to move twice)

        assert_eq!(Board::new(vec![
                1, 2, 3,
                4, 0, 5,
                6, 7, 8
            ]).neighbours(),
            vec![ // zero should move in all four directions
                Board::new(vec![
                    1, 2, 3,
                    0, 4, 5,
                    6, 7, 8
                ]),
                Board::new(vec![
                    1, 2, 3,
                    4, 5, 0,
                    6, 7, 8
                ]),
                Board::new(vec![
                    1, 0, 3,
                    4, 2, 5,
                    6, 7, 8
                ]),
                Board::new(vec![
                    1, 2, 3,
                    4, 7, 5,
                    6, 0, 8
                ])
            ]
        ); // all numbers in wrong position (note that 2 needs to move twice)
    }
}
