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

pub fn solve(board: &Board) -> Vec<Board> {
    let mut pq = BinaryHeap::new();
    let mut state = Rc::new(BoardState { prev: None, board: board.clone() });
    while !state.board.is_goal() {
        // println!("Current state is {:?}", state.board);
        for neighbour in state.board.neighbours() {
            if state.prev.is_none() || state.prev.as_ref().unwrap().board != neighbour {
                let nbs = BoardState {
                    prev: Some(state.clone()),
                    board: neighbour,
                };
                pq.push(nbs);
            }
        }

        state = Rc::new(pq.pop().expect("Ran out of moves; looks like the board is unsolveable"));
    }

    let mut solution = Vec::with_capacity(50); // 50 = stab in the dark at average path length
    while state.prev.is_some() {
        solution.push(state.board.clone());
        state = rc::try_unwrap(state).ok().unwrap().prev.unwrap();
    }
    solution.push(state.board.clone());
    solution.reverse();

    return solution;
}


#[cfg(test)]
mod tests {
    use super::solve;
    use board::Board;

    #[test]
    fn solve_already_solved_board() {
        let b = Board::new(vec![1, 2, 3, 0]);
        assert_eq!(solve(&b), vec![b.clone()]);
    }

    #[test]
    fn solve_one_step_away_board() {
        let b = Board::new(vec![1, 2, 0, 3]);
        assert_eq!(solve(&b), vec![b.clone(), Board::new(vec![1, 2, 3, 0])]);
    }

    #[test]
    fn solve_several_steps_on_2_by_2_board() {
        let b = Board::new(vec![2, 0, 1, 3]);
        assert!(solve(&b).len() == 4); // swap 0 with 2, swap 0 with 1, swap 0 with 3
    }
}
