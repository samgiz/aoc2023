use std::{io, collections::{VecDeque, HashSet}};
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Pipe {
  Animal,
  Empty,
  UpDown,
  LeftRight,
  UpRight,
  UpLeft,
  DownLeft,
  DownRight
}
use Pipe::*;
impl Pipe {
  fn connects_left(&self) -> bool {
    self == &UpLeft || self == &DownLeft || self == &LeftRight
  }
  fn connects_right(&self) -> bool {
    self == &UpRight || self == &DownRight || self == &LeftRight
  }
  fn connects_up(&self) -> bool {
    self == &UpRight || self == &UpLeft || self == &UpDown
  }
  fn connects_down(&self) -> bool {
    self == &DownRight || self == &DownLeft || self == &UpDown
  }
}

impl From<&u8> for Pipe {
  fn from(c: &u8) -> Pipe {
    match c {
      b'|' => UpDown,
      b'-' => LeftRight,
      b'L' => UpRight,
      b'J' => UpLeft,
      b'7' => DownLeft,
      b'F' => DownRight,
      b'.' => Empty,
      b'S' => Animal,
      _ => panic!("Pipe::from received an invalid byte {c}")
    }
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Tile {
  row: usize,
  col: usize,
  pipe: Pipe
}

fn valid_indices(i: i64, j: i64, n: usize, m: usize)  -> bool {
  i >= 0 && j >= 0 && i < n as i64 && j < m as i64
}

impl Tile {
  fn connects_to(&self, other: &Tile) -> bool {
    if self.row == other.row && self.col + 1 == other.col
      && self.pipe.connects_right()
      && other.pipe.connects_left() {
        return true;
    }
    if self.row == other.row && self.col == other.col + 1
      && self.pipe.connects_left()
      && other.pipe.connects_right() {
        return true;
    }
    if self.col == other.col && self.row + 1 == other.row
      && self.pipe.connects_down()
      && other.pipe.connects_up() {
        return true;
    }
    if self.col == other.col && self.row == other.row + 1
      && self.pipe.connects_up()
      && other.pipe.connects_down() {
        return true;
    }
    false
  }
} 

fn main() {
  let mut board = io::stdin().lines().enumerate().map(|(line_number, line)| {
    let line = line.unwrap();
    let pipes = line
      .as_bytes()
      .iter()
      .map(Pipe::from)
      .enumerate()
      .map(|(i, pipe)| Tile {
        row: line_number,
        col: i,
        pipe,
      })
      .collect::<Vec<_>>();
    pipes
  }).collect::<Vec<_>>();

  let starting_tile = (*board.iter().flatten().find(|t|t.pipe == Animal).unwrap()).clone();
  
  for pipe in [UpRight, UpDown, UpLeft, DownLeft, DownRight, LeftRight] {
    board[starting_tile.row][starting_tile.col].pipe = pipe;
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((&board[starting_tile.row][starting_tile.col], 0));
    visited.insert(&board[starting_tile.row][starting_tile.col]);
    let mut wrong_pipe = false;
    let mut answer = 0;
    while !q.is_empty() {
      let (tile, distance) = q.pop_front().unwrap();
      let mut num_connected = 0;
        for (drow, dcol) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let row = drow + (tile.row as i64);
        let col = dcol + (tile.col as i64);
        if valid_indices(row, col, board.len(), board[0].len()) {
          let tile_to_try = &board[row as usize][col as usize];
          if tile.connects_to(tile_to_try) {
            num_connected += 1;
            if !visited.contains(tile_to_try) {
              q.push_back((tile_to_try, distance + 1));
              answer = std::cmp::max(answer, distance + 1);
            }
            visited.insert(tile_to_try);
          }
        }
      }
      if num_connected != 2 {
        wrong_pipe = true;
      }
    }
    if !wrong_pipe {
      println!("{}", answer);
    } 
  }
}