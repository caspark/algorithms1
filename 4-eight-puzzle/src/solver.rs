use board::Board;
use std::collections::BinaryHeap;
use std::cmp::{Eq, PartialEq, Ord, Ordering};
use std::rc::{self, Rc};

#[derive(Debug)]
struct BoardState {
    prev: Option<Rc<BoardState>>,
    board: Board,
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.board.manhattan().cmp(&self.board.manhattan()) // compare other to self to reverse the order for the max-heap instead of a min-heap
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for BoardState {}

pub fn solve(board: &Board) -> Option<Vec<Board>> {
    let mut pq = BinaryHeap::new();
    let mut pq_twin = BinaryHeap::new();
    let mut state = Rc::new(BoardState { prev: None, board: board.clone() });
    let mut state_twin = Rc::new(BoardState { prev: None, board: board.twin() });
    while !state.board.is_goal() && !state_twin.board.is_goal() {
        // println!("Current state is {:?}", state.board);
        for neighbour in state.board.neighbours() {
            if state.prev.is_none() || state.prev.as_ref().unwrap().board != neighbour {
                pq.push(BoardState {
                    prev: Some(state.clone()),
                    board: neighbour,
                });
            }
        }
        for neighbour in state_twin.board.neighbours() {
            if state_twin.prev.is_none() || state_twin.prev.as_ref().unwrap().board != neighbour {
                pq_twin.push(BoardState {
                    prev: Some(state_twin.clone()),
                    board: neighbour,
                });
            }
        }

        //FIXME distance measures sometimes increase/decrease
        println!("Board: {:?} Twin: {:?}", state.board.manhattan(), state_twin.board.manhattan());

        state = Rc::new(pq.pop().expect("Ran out of moves; looks like the board is unsolveable"));
        state_twin = Rc::new(pq_twin.pop().expect("Ran out of moves; looks like the board is unsolveable"));
    }

    if state_twin.board.is_goal() {
        None
    } else {
        let mut solution = Vec::with_capacity(50); // 50 = stab in the dark at average path length
        while state.prev.is_some() {
            solution.push(state.board.clone());
            state = rc::try_unwrap(state).ok().unwrap().prev.unwrap();
        }
        solution.push(state.board.clone());
        solution.reverse();
        Some(solution)
    }
}


#[cfg(test)]
mod tests {
    use super::solve;
    use board::Board;

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
    fn solve_finishes_for_random_board_of_size_2() { //FIXME this test sometimes never terminates
        solve(&Board::random(2));
    }
}
