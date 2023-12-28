use std::{io, collections::{VecDeque, HashSet}};

// --F7..
// --|L-7
// S-J.FJ
// L--7|.
// ...LJ.

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
    return self == &UpLeft || self == &DownLeft || self == &LeftRight;
  }
  fn connects_right(&self) -> bool {
    return self == &UpRight || self == &DownRight || self == &LeftRight;
  }
  fn connects_up(&self) -> bool {
    return self == &UpRight || self == &UpLeft || self == &UpDown;
  }
  fn connects_down(&self) -> bool {
    return self == &DownRight || self == &DownLeft || self == &UpDown;
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

fn rotate_left(coords: (i64, i64)) -> (i64, i64) {
  match coords {
    (1, 0) => (0, 1),
    (-1, 0) => (0, -1),
    (0, 1) => (-1, 0),
    (0, -1) => (1, 0),
    _ => panic!("passed wrong coords to rotation funtion: {coords:?}")
  }
}

fn rotate_right(coords: (i64, i64)) -> (i64, i64) {
  match coords {
    (1, 0) => (0, -1),
    (-1, 0) => (0, 1),
    (0, 1) => (1, 0),
    (0, -1) => (-1, 0),
    _ => panic!("passed wrong coords to rotation funtion: {coords:?}")
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
    return false;
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

  let starting_tile = (*board.iter().flat_map(|x|x).find(|t|t.pipe == Animal).unwrap()).clone();
  
  for pipe in [UpRight, UpDown, UpLeft, DownLeft, DownRight, LeftRight] {
    board[starting_tile.row][starting_tile.col].pipe = pipe.clone();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((&board[starting_tile.row][starting_tile.col], 0));
    visited.insert(&board[starting_tile.row][starting_tile.col]);
    let mut wrong_pipe = false;
    // dbg!("!!!!!!!!!!", pipe.clone());
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
            }
            visited.insert(tile_to_try);
          }
        }
      }
      if num_connected != 2 {
        wrong_pipe = true;
        break;
      }
    }
    if !wrong_pipe {
      let top_left_tile = *visited.iter().min_by_key(|x| (x.row, x.col)).unwrap();
      let mut move_direction = (0, 1);
      let mut search_direction = (1, 0);
      let mut current = &board[top_left_tile.row][top_left_tile.col+1];
      let mut nest = HashSet::new();
      while current != top_left_tile {
        // Search in direction search_direction
        let row = (current.row as i64) + search_direction.0;
        let col = (current.col as i64) + search_direction.1;
        if valid_indices(row, col, board.len(), board[0].len()) {
          let start_tile = &board[row as usize][col as usize];
          let mut q = Vec::new();
          if !nest.contains(start_tile) && !visited.contains(start_tile) {
            nest.insert(start_tile);
            q.push(start_tile);
          }
          while !q.is_empty() {
            let top = q.pop().unwrap();
            for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
              let row = drow + (top.row as i64);
              let col = dcol + (top.col as i64);
              if valid_indices(row, col, board.len(), board[0].len()) {
                let tile_to_try = &board[row as usize][col as usize];
                if !nest.contains(tile_to_try) && !visited.contains(tile_to_try) {
                  nest.insert(tile_to_try);
                  q.push(tile_to_try);
                }
              }
            }
          }
        }
        // Try straight
        let row = move_direction.0 + (current.row as i64);
        let col = move_direction.1 + (current.col as i64);
        if valid_indices(row, col, board.len(), board[0].len()) {
          let tile_to_try = &board[row as usize][col as usize];
          if current.connects_to(tile_to_try) {
            current = tile_to_try;
            continue;
          }
        }
        // Try right
        let rotated_move_direction = rotate_right(move_direction);
        let row = rotated_move_direction.0 + (current.row as i64);
        let col = rotated_move_direction.1 + (current.col as i64);
        if valid_indices(row, col, board.len(), board[0].len()) {
          let tile_to_try = &board[row as usize][col as usize];
          if current.connects_to(tile_to_try) {
            current = tile_to_try;
            move_direction = rotated_move_direction;
            search_direction = rotate_right(search_direction);
            continue;
          }
        }
        // Try left
        let rotated_move_direction = rotate_left(move_direction);
        let row = rotated_move_direction.0 + (current.row as i64);
        let col = rotated_move_direction.1 + (current.col as i64);
        if valid_indices(row, col, board.len(), board[0].len()) {
          let tile_to_try = &board[row as usize][col as usize];
          if current.connects_to(tile_to_try) {
            // Need to check in the new move direction as we might otherwise miss a tile.
            search_direction = rotate_left(search_direction);
            move_direction = rotated_move_direction;
            // Search in direction search_direction
            let row = (current.row as i64) + search_direction.0;
            let col = (current.col as i64) + search_direction.1;
            if valid_indices(row, col, board.len(), board[0].len()) {
              let start_tile = &board[row as usize][col as usize];
              let mut q = Vec::new();
              if !nest.contains(start_tile) && !visited.contains(start_tile) {
                nest.insert(start_tile);
                q.push(start_tile);
              }
              while !q.is_empty() {
                let top = q.pop().unwrap();
                for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                  let row = drow + (top.row as i64);
                  let col = dcol + (top.col as i64);
                  if valid_indices(row, col, board.len(), board[0].len()) {
                    let tile_to_try = &board[row as usize][col as usize];
                    if !nest.contains(tile_to_try) && !visited.contains(tile_to_try) {
                      nest.insert(tile_to_try);
                      q.push(tile_to_try);
                    }
                  }
                }
              }
            }
            current = tile_to_try;
            continue;
          }
        }
      }
      let usable_nest_amount = nest.len();
      println!("{}", usable_nest_amount);
      break;
    } 
  }
}