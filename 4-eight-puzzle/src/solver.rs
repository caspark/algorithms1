use board::Board;
use std::collections::BinaryHeap;
use std::cmp::{Eq, PartialEq, Ord, Ordering};
use std::rc::{self, Rc};

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
    let mut a_state = Rc::new(BoardState::new(None, 0, board.clone()));
    let mut b_state = Rc::new(BoardState::new(None, 0, board.twin()));

    while !a_state.board.is_goal() && !b_state.board.is_goal() {
        // println!("Current a_state is {:?}", a_state.board);
        for neighbour in a_state.board.neighbours() {
            if a_state.parent.is_none() || a_state.parent.as_ref().unwrap().board != neighbour {
                a_pq.push(BoardState::new(Some(a_state.clone()), a_state.depth + 1, neighbour));
            }
        }
        for neighbour in b_state.board.neighbours() {
            if b_state.parent.is_none() || b_state.parent.as_ref().unwrap().board != neighbour {
                b_pq.push(BoardState::new(Some(b_state.clone()), b_state.depth + 1, neighbour));
            }
        }

        a_state = Rc::new(a_pq.pop().expect("Ran out of moves; looks like the board is unsolveable"));
        b_state = Rc::new(b_pq.pop().expect("Ran out of moves; looks like the board is unsolveable"));
    }

    if b_state.board.is_goal() {
        None
    } else {
        let mut solution = Vec::with_capacity(a_state.depth as usize + 1 as usize);
        while a_state.parent.is_some() {
            solution.push(a_state.board.clone());
            a_state = rc::try_unwrap(a_state).ok().unwrap().parent.unwrap();
        }
        solution.push(a_state.board.clone());
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
}
