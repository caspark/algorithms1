use board::Board;
use std::collections::BinaryHeap;
use std::cmp::{Eq, PartialEq, Ord, Ordering};
use std::rc::Rc;

#[derive(Debug)]
struct BoardState {
    parent: Option<Rc<BoardState>>,
    depth: u32, // number of parent nodes
    priority: i64, // cached priority to determine ordering: depth + score of board
    board: Board,
}

impl BoardState {
    fn new(parent: Option<Rc<BoardState>>, current_depth: u32, board: Board) -> BoardState {
        BoardState {
            parent: parent,
            depth: current_depth,
            priority: current_depth as i64 + board.manhattan(),
            board: board,
        }
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare other to self to reverse the order to use BinaryHeap as a min-heap instead of a max-heap
        other.priority.cmp(&self.priority)
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for BoardState {}

pub fn solve(board: &Board) -> Option<Vec<Board>> {
    let mut a_pq = BinaryHeap::new();
    let mut b_pq = BinaryHeap::new();
    let mut a_state_rc = Rc::new(BoardState::new(None, 0, board.clone()));
    let mut b_state_rc = Rc::new(BoardState::new(None, 0, board.twin()));

    while !a_state_rc.board.is_goal() && !b_state_rc.board.is_goal() {
        // println!("Current a_state_rc is {:?}", a_state_rc.board);
        for neighbour in a_state_rc.board.neighbours() {
            if a_state_rc.parent.is_none() || a_state_rc.parent.as_ref().unwrap().board != neighbour {
                a_pq.push(BoardState::new(Some(a_state_rc.clone()), a_state_rc.depth + 1, neighbour));
            }
        }
        for neighbour in b_state_rc.board.neighbours() {
            if b_state_rc.parent.is_none() || b_state_rc.parent.as_ref().unwrap().board != neighbour {
                b_pq.push(BoardState::new(Some(b_state_rc.clone()), b_state_rc.depth + 1, neighbour));
            }
        }

        a_state_rc = Rc::new(a_pq.pop().expect("From any search node there is at least 1 legal move"));
        b_state_rc = Rc::new(b_pq.pop().expect("From any search node there is at least 1 legal move"));
    }

    if b_state_rc.board.is_goal() {
        None
    } else {
        let mut current = &a_state_rc;
        let mut solution = Vec::with_capacity(current.depth as usize + 1);
        while current.parent.is_some() {
            solution.push(current.board.clone());
            current = current.parent.as_ref().unwrap();
        }
        solution.push(current.board.clone());
        solution.reverse();
        Some(solution)
    }
}


#[cfg(test)]
mod tests {
    use super::solve;
    use board::{self, Board};

    #[test]
    fn solve_already_solved_board() {
        let b = Board::new(vec![1, 2, 3, 0]);
        assert_eq!(solve(&b), Some(vec![b.clone()]));
    }

    #[test]
    fn solve_one_step_away_board() {
        let b = Board::new(vec![1, 2, 0, 3]);
        assert_eq!(solve(&b), Some(vec![b.clone(), Board::new(vec![1, 2, 3, 0])]));
    }

    #[test]
    fn solve_several_steps_on_2_by_2_board() {
        let b = Board::new(vec![2, 0, 1, 3]);
        assert!(solve(&b).unwrap().len() == 4); // swap 0 with 2, swap 0 with 1, swap 0 with 3
    }

    #[test]
    fn solve_unsolvable() {
        let b = Board::new(vec![2, 1, 3, 0]); // 1 and 2 are swapped
        assert_eq!(solve(&b), None);
    }

    #[test]
    fn solve_finishes_for_all_boards_of_size_2() {
        for board in board::generate_all_boards_of_size(2) {
            // println!("Trying to solve: {:?}", board);
            solve(&board);
        }
    }

    #[test]
    fn solve_finishes_for_a_lot_of_boards_of_size_3() {
        let arbitrary_big_prime = 3299; // because there are a lot of permutations of a 9 element array, only solve some
        let all_size_3_boards = board::generate_all_boards_of_size(3);
        let mut count = 0;
        let mut avg_steps = 0.0;
        let mut failures = 0;
        for board in all_size_3_boards.iter()
                                      .enumerate()
                                      .filter(|&(i, _)| i % arbitrary_big_prime == 0)
                                      .map(|(_, b)| b) {
            // println!("Solving {:?}...", board);
            match solve(&board) {
                Some(solution) => avg_steps = (avg_steps * count as f64 + solution.len() as f64) / (count as f64 + 1.0),
                None => failures += 1,
            }
            count += 1;
        }
        println!("Solved {}/{} 3x3 boards; average solution length = {}", count - failures, count, avg_steps);
    }
}
