use std::cmp::Ordering;
use std::io;
use std::collections::{BinaryHeap, HashSet, BTreeSet};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right
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
      Left => (self.row as i64, self.col as i64 - 1)
    };
    if new_row < 0 || new_col < 0 || new_row >= board.len() as i64 || new_col >= board[0].len() as i64 {
      None
    } else {
      Some(Location {
        row: new_row as usize,
        col: new_col as usize
      })
    }
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Plan {
  dir: Direction,
  amount: i64,
}

impl From<u8> for Direction {
  fn from(value: u8) -> Self {
    match value {
      b'0' => Right,
      b'1' => Down,
      b'2' => Left,
      b'3' => Up,
      _ => panic!("Invalid character passed to direction")
    }
  }
}

struct Vertical {
  col: i64,
  top: i64,
  bottom: i64
}

struct Horizontal {
  row: i64,
  left: i64,
  right: i64
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum IntervalState {
  Inside,
  Outside
}
use IntervalState::*;

impl IntervalState {
  fn flip(self) -> IntervalState {
    match self {
      Inside => Outside,
      Outside => Inside
    }
  }
}

struct Intervals {
  // Sorted list of disjoint intervals
  v: Vec<Interval>
}

impl Intervals {
  
  fn init(&mut self, init: Interval) {
    self.v = vec!(init);
  }
  // The intervals that get passed here will never intersect 2 intervals already in v
  fn flip(&mut self, mut left: i64, mut right: i64) {
    // We assume that the interval is contained within one of the existing intervals, find it.
    let index = (|| {
      for (i, interval) in self.v.iter().enumerate() {
        match interval.state {
          Inside => {
            if interval.left <= left && right < interval.right {
              return i;
            }
          }
          Outside => {
            if interval.left <= left+1 && right <= interval.right {
              return i;
            }
          }
        }
      }
      panic!("Could not find an index");
    })();
    let candidate = self.v[index];
    let mut replace_with = Vec::new();
    match candidate.state {
      Outside => {
        right += 1;
        if candidate.left != left {
          // This is outside
          replace_with.push(Interval {left: candidate.left, right: left, state: candidate.state});
        }
        // This is inside
        replace_with.push(Interval {left, right, state: candidate.state.flip()});
        if right < candidate.right {
          // This is outside
          replace_with.push(Interval {left: right, right: candidate.right, state: candidate.state});
        }
      }
      Inside => {
        if candidate.left != left {
          left += 1;
        }
        if candidate.right == right+1 {
          right += 1;
        }
        if candidate.left != left {
          // this is inside
          replace_with.push(Interval {left: candidate.left, right: left, state: candidate.state});
        }
        // this is outside
        replace_with.push(Interval {left, right, state: candidate.state.flip()});
        if candidate.right > right {
          // this is inside
          replace_with.push(Interval {left: right, right: candidate.right, state: candidate.state});
        }
      }
    };

    let mut new_v = self.v[..index].to_vec();
    for i in replace_with {
      if i.right - i.left < 1 {
        continue;
      }
      if new_v.len() > 0 && i.state == new_v.last().unwrap().state {
        // Combine the two intervals
        new_v.last_mut().unwrap().right = i.right;
      } else {
        new_v.push(i);
      }
    }
    for i in self.v[index+1..].iter() {
      if new_v.len() > 0 && i.state == new_v.last().unwrap().state {
        // Combine the two intervals
        new_v.last_mut().unwrap().right = i.right;
      } else {
        new_v.push(*i);
      }
    }
    self.v = new_v;
  }
  fn get_state(&self, left: i64, right: i64) -> IntervalState {
    for interval in self.v.iter() {
      match interval.state {
        Inside => {
          if interval.left <= left && right < interval.right {
            return Inside;
          }
        }
        Outside => {
          if interval.left <= left+1 && right <= interval.right {
            return Outside;
          }
        }
      }
    }
    panic!("Did not find the correct interval state");
  }
  fn get_num_inside(&self) -> u64 {
    self.v.iter().map(|x| if x.state == Inside {(x.right - x.left) as u64} else {0}).sum()
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Interval {
  left: i64,
  right: i64,
  state: IntervalState
}

fn main() {
  let lines = io::stdin().lines();
  let plans: Vec<_> = lines.map(|line| {
    let line = line.unwrap();
    let [_dir, _amount, color]: [&str; 3] = line.split(' ').collect::<Vec<&str>>().try_into().unwrap();
    let color = color.as_bytes().to_vec();
    let dir = Direction::from(color[color.len()-2]);
    let amount = i64::from_str_radix(std::str::from_utf8(&color[2..7]).unwrap(), 16).unwrap();
    Plan {dir, amount}
  }).collect();
  let mut row = 0;
  let mut col = 0;
  let mut row_max = 0;
  let mut row_min = 0;
  let mut col_max = 0;
  let mut col_min = 0;
  let mut horizontals = Vec::new();
  let mut rows_of_interest = BTreeSet::new();
  for plan in plans.iter() {
    match plan.dir {
      Down => {
        rows_of_interest.insert(row);
        rows_of_interest.insert(row+plan.amount);
        row += plan.amount
      },
      Up => {
        rows_of_interest.insert(row-plan.amount);
        rows_of_interest.insert(row);
        row -= plan.amount
      },
      Right => {
        horizontals.push(Horizontal {
          row,
          left: col,
          right: col + plan.amount
        });
        col += plan.amount
      },
      Left => {
        horizontals.push(Horizontal {
          row,
          left: col - plan.amount,
          right: col
        });
        col -= plan.amount
      }
    }
    use std::cmp::{max, min};
    row_max = max(row, row_max);
    row_min = min(row, row_min);
    col_max = max(col, col_max);
    col_min = min(col, col_min);
  };
  let mut current_intervals = Intervals {v: Vec::new()};
  current_intervals.init(Interval {
    left: col_min - 1,
    right: col_max + 2,
    state: Outside
  });
  let mut answer = 0;
  let rows_of_interest: Vec<i64> = rows_of_interest.iter().map(|x| *x).collect();
  for (i, &row) in rows_of_interest.iter().enumerate() {
    let to_process = horizontals.iter().filter(|x| x.row == row);
    let to_process_first: Vec<_> = to_process.clone().filter(|x| current_intervals.get_state(x.left, x.right) == Outside).collect();
    let to_process_second: Vec<_> = to_process.filter(|x| current_intervals.get_state(x.left, x.right) == Inside).collect();
    to_process_first.iter().for_each(|&Horizontal {row, left, right}| current_intervals.flip(*left, *right));
    answer += current_intervals.get_num_inside();
    if i != rows_of_interest.len()-1 {
      to_process_second.iter().for_each(|&Horizontal {row, left, right}| current_intervals.flip(*left, *right));
      let next_row = rows_of_interest[i+1];
      let amount = (next_row - row - 1) as u64;
      answer += amount * current_intervals.get_num_inside();
    }
  }
  println!("{}", answer);
}