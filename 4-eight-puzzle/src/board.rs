use std::num::Int;

/// An 8 Puzzle board (or N puzzle board).
#[derive(Debug, Eq, PartialEq)]
struct Board {
    board: Vec<u64>
}

fn usize_square_root(i: usize) -> usize {
    use std::num::Float;
    (i as f64).sqrt() as usize
}

impl Board {
    pub fn new(board: Vec<u64>) -> Board {
        if usize_square_root(board.len()).pow(2) != board.len() {
            panic!(format!("Provided board is not square! Size = {}", board.len()))
        }

        let mut copy_to_sort = board.clone();
        &mut copy_to_sort[..].sort();
        for (i, p) in copy_to_sort.iter().enumerate() {
            if i as u64 != *p {
                panic!("Missing expected number {} in board!", i);
            }
        }

        Board { board: board }
    }

    pub fn random(n: usize) -> Board {
        Board {
            board: {
                use rand::{thread_rng, Rng};
                let mut v = (0 .. (n as u64).pow(2)).collect::<Vec<u64>>();
                thread_rng().shuffle(&mut v[..]);
                v
            }
        }
    }

    fn dimension(&self) -> usize {
        usize_square_root(self.board.len())
    }

    fn is_goal(&self) -> bool {
        for (i, p) in self.board.iter().enumerate() {
            if i < self.board.len() - 1 && *p != (i as u64) + 1 // all except end of board
                    || i == self.board.len() - 1 && *p != 0 { // end of board
                return false;
            }
        }
        true
    }
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
}
