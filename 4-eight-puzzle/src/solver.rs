use board::Board;
use std::collections::BinaryHeap;
use std::cmp::{Eq, PartialEq, Ord, Ordering};
use std::rc::{self, Rc};

#[derive(Debug)]
struct BoardState {
    prev: Option<Rc<BoardState>>,
    curr: Board,
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.curr.manhattan().cmp(&self.curr.manhattan()) // compare other to self to reverse the order for the max-heap instead of a min-heap
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
    let mut state = Rc::new(BoardState { prev: None, curr: board.clone() });
    while !state.curr.is_goal() {
        println!("Current state is {:?}", state.curr);
        for neighbour in state.curr.neighbours() {
            let nbs = BoardState {
                prev: Some(state.clone()),
                curr: neighbour,
            };
            pq.push(nbs);
        }

        state = Rc::new(pq.pop().expect("Ran out of moves; looks like the board is unsolveable"));
    }

    let mut solution = Vec::with_capacity(50); // 50 = stab in the dark at average path length
    while state.prev.is_some() {
        solution.push(state.curr.clone());
        state = rc::try_unwrap(state).ok().unwrap().prev.unwrap();
    }
    solution.push(state.curr.clone());
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
}
