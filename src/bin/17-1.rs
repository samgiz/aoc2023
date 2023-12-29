use std::cmp::Ordering;
use std::io;
use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}
use Direction::*;

impl Direction {
  fn left(&self) -> Direction {
    match self {
      Up => Left,
      Left => Down,
      Down => Right,
      Right => Up
    }
  }
  fn right(&self) -> Direction {
    match self {
      Up => Right,
      Left => Up,
      Down => Left,
      Right => Down
    }
  }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct QueueState {
  cost: u64,
  row: usize,
  col: usize,
  num_forward: u64,
  dir: Direction
}

impl QueueState {
  fn move_in(&self, dir: Direction, board: &Vec<Vec<u64>>) -> Option<QueueState> {
    let (new_row, new_col) = match dir {
      Up => (self.row as i64 - 1, self.col as i64),
      Down => (self.row as i64 + 1, self.col as i64),
      Right => (self.row as i64, self.col as i64 + 1),
      Left => (self.row as i64, self.col as i64 - 1)
    };
    if new_row < 0 || new_col < 0 || new_row >= board.len() as i64 || new_col >= board[0].len() as i64 {
      None
    } else if dir == self.dir && self.num_forward == 3 {
      None
    } else {
      Some(QueueState {
        row: new_row as usize,
        col: new_col as usize,
        cost: self.cost + board[new_row as usize][new_col as usize],
        num_forward: if dir == self.dir {self.num_forward + 1} else {1},
        dir
      })
    }
  }
}

impl Ord for QueueState {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
      .then_with(|| self.row.cmp(&other.row))
      .then_with(|| self.col.cmp(&other.col))
      .then_with(|| self.num_forward.cmp(&other.num_forward))
      .then_with(|| self.dir.cmp(&other.dir))
  }
}

impl PartialOrd for QueueState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct VisitedState {
  row: usize,
  col: usize,
  num_forward: u64,
  dir: Direction
}

impl From<&QueueState> for VisitedState {
    fn from(state: &QueueState) -> Self {
      return VisitedState {
        row: state.row, 
        col: state.col,
        num_forward: state.num_forward,
        dir: state.dir
      }
    }
}

fn main() {
  let board: Vec<_> = io::stdin().lines().map(|line|line.unwrap().as_bytes().iter().map(|&x|(x-b'0') as u64).collect::<Vec<_>>()).collect();
  let mut q = BinaryHeap::<QueueState>::new();
  q.push(QueueState{
    cost: 0,
    row: 0,
    col: 0,
    num_forward: 0,
    dir: Down
  });
  let mut visited = HashSet::<VisitedState>::new();
  let mut answer: Option<u64> = None;
  while !q.is_empty() {
    let top = q.pop().unwrap();
    let visited_state = VisitedState::from(&top);
    if visited.contains(&visited_state) {
      continue;
    }
    visited.insert(visited_state);
    if top.row == board.len() - 1 && top.col == board[0].len()-1 {
      answer = match answer {
        None => Some(top.cost),
        Some(answer) => Some(std::cmp::min(answer, top.cost))
      };
    }
    // Try forward
    let next = top.move_in(top.dir, &board);
    match next {
      None => (),
      Some(next) => {
        q.push(next);
      }
    }
    // Try left
    let next = top.move_in(top.dir.left(), &board);
    match next {
      None => (),
      Some(next) => {
        q.push(next);
      }
    }
    // Try right
    let next = top.move_in(top.dir.right(), &board);
    match next {
      None => (),
      Some(next) => {
        q.push(next);
      }
    }
  }
  println!("{}", answer.unwrap());
}