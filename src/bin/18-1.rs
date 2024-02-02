use std::io;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
use Direction::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Location {
  col: usize,
  row: usize,
}

impl Location {
  fn move_in(&self, dir: Direction, board: &Vec<Vec<u8>>) -> Option<Location> {
    let (new_row, new_col) = match dir {
      Up => (self.row as i64 - 1, self.col as i64),
      Down => (self.row as i64 + 1, self.col as i64),
      Right => (self.row as i64, self.col as i64 + 1),
      Left => (self.row as i64, self.col as i64 - 1),
    };
    if new_row < 0
      || new_col < 0
      || new_row >= board.len() as i64
      || new_col >= board[0].len() as i64
    {
      None
    } else {
      Some(Location {
        row: new_row as usize,
        col: new_col as usize,
      })
    }
  }
}

struct Plan {
  dir: Direction,
  amount: i64,
  // color:
}

impl From<u8> for Direction {
  fn from(value: u8) -> Self {
    match value {
      b'D' => Down,
      b'R' => Right,
      b'L' => Left,
      b'U' => Up,
      _ => panic!("Invalid character passed to direction"),
    }
  }
}

fn dfs(board: &mut Vec<Vec<u8>>, row: usize, col: usize) -> Option<u64> {
  let mut to_visit = Vec::new();
  to_visit.push(Location { row, col });
  board[row][col] = b'v';
  let mut amount = 0;
  let mut return_none = false;
  while let Some(cur) = to_visit.pop() {
    amount += 1;
    for dir in [Up, Down, Left, Right] {
      let next = cur.move_in(dir, board);
      match next {
        None => {
          return_none = true;
        }
        Some(next) => {
          if board[next.row][next.col] != b'#' && board[next.row][next.col] != b'v' {
            board[next.row][next.col] = b'v';
            to_visit.push(next);
          }
        }
      }
    }
  }
  if return_none {
    None
  } else {
    Some(amount)
  }
}

fn main() {
  let lines = io::stdin().lines();
  let plans: Vec<_> = lines
    .map(|line| {
      let line = line.unwrap();
      let [dir, amount, _color]: [&str; 3] =
        line.split(' ').collect::<Vec<&str>>().try_into().unwrap();
      let dir = Direction::from(*dir.as_bytes().first().unwrap());
      let amount = amount.parse::<i64>().unwrap();
      Plan { dir, amount }
    })
    .collect();
  let mut row = 0;
  let mut col = 0;
  let mut row_max = 0;
  let mut row_min = 0;
  let mut col_max = 0;
  let mut col_min = 0;
  for plan in plans.iter() {
    match plan.dir {
      Down => row += plan.amount,
      Up => row -= plan.amount,
      Right => col += plan.amount,
      Left => col -= plan.amount,
    }
    use std::cmp::{max, min};
    row_max = max(row, row_max);
    row_min = min(row, row_min);
    col_max = max(col, col_max);
    col_min = min(col, col_min);
  }
  let row_size = (row_max - row_min + 1) as usize;
  let col_size = (col_max - col_min + 1) as usize;
  let mut current_location = Location {
    row: -row_min as usize,
    col: -col_min as usize,
  };
  let mut board = vec![vec!(b'.'; col_size); row_size];
  board[current_location.row][current_location.col] = b'#';
  let mut already_dug_out = 1;
  for plan in plans {
    for _ in 0..plan.amount {
      current_location = current_location.move_in(plan.dir, &board).unwrap();
      if board[current_location.row][current_location.col] != b'#' {
        already_dug_out += 1;
      }
      board[current_location.row][current_location.col] = b'#';
    }
  }
  for i in 0..board.len() {
    for j in 0..board[0].len() {
      if board[i][j] == b'.' {
        let answer = dfs(&mut board, i, j);
        if let Some(answer) = answer {
          println!("{}", answer + already_dug_out)
        }
      }
    }
  }
}
