use std::{io, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}
use Direction::*;

type BeamState = (usize, usize, Direction);

fn is_valid_index(i: i64, j: i64, n: usize, m: usize) -> bool {
  i >= 0 && i < n as i64 && j >= 0 && j < m as i64
}

fn add_if_valid((i, j): (i64, i64), dir: Direction, board: &Vec<Vec<u8>>, to_visit: &mut Vec<BeamState>) {
  if is_valid_index(i, j, board.len(), board[0].len()) {
    to_visit.push((i as usize, j as usize, dir));
  }
}
fn move_in(i: usize, j: usize, dir: Direction) -> (i64, i64) {
  match dir {
    Left => (i as i64, j as i64 - 1),
    Right => (i as i64, j as i64 + 1),
    Up => (i as i64 - 1, j as i64),
    Down => (i as i64 + 1, j as i64),
  }
}

fn main() {
  let board: Vec<_> = io::stdin().lines().map(|line|line.unwrap().as_bytes().to_vec()).collect();
  let mut visited: HashSet<BeamState> = HashSet::new();
  let mut to_visit: Vec<BeamState> = Vec::new();
  to_visit.push((0, 0, Right));
  while !to_visit.is_empty() {
    let next = to_visit.pop().unwrap();
    let (i, j, dir) = next;
    if visited.contains(&next) {
      continue;
    }
    visited.insert(next);
    match board[i][j] {
      b'.' => {
        add_if_valid(move_in(i, j, dir), dir, &board, &mut to_visit);
      }
      b'/' => {
        match dir {
          Left => add_if_valid(move_in(i, j, Down), Down, &board, &mut to_visit),
          Down => add_if_valid(move_in(i, j, Left), Left, &board, &mut to_visit),
          Right => add_if_valid(move_in(i, j, Up), Up, &board, &mut to_visit),
          Up => add_if_valid(move_in(i, j, Right), Right, &board, &mut to_visit),
        }
      }
      b'\\' => {
        match dir {
          Right => add_if_valid(move_in(i, j, Down), Down, &board, &mut to_visit),
          Up => add_if_valid(move_in(i, j, Left), Left, &board, &mut to_visit),
          Left => add_if_valid(move_in(i, j, Up), Up, &board, &mut to_visit),
          Down => add_if_valid(move_in(i, j, Right), Right, &board, &mut to_visit),
        }
      }
      b'|' => {
        match dir {
          Up | Down => add_if_valid(move_in(i, j, dir), dir, &board, &mut to_visit),
          Right | Left => {
            add_if_valid(move_in(i, j, Up), Up, &board, &mut to_visit);
            add_if_valid(move_in(i, j, Down), Down, &board, &mut to_visit);
          }
        }
      }
      b'-' => {
        match dir {
          Right | Left => add_if_valid(move_in(i, j, dir), dir, &board, &mut to_visit),
          Up | Down => {
            add_if_valid(move_in(i, j, Left), Left, &board, &mut to_visit);
            add_if_valid(move_in(i, j, Right), Right, &board, &mut to_visit);
          }
        }
      }
      _ => panic!("Encountered invalid character")
    }
    
  }
  let answer = visited.iter().map(|x|(x.0, x.1)).collect::<HashSet<_>>().len();
  println!("{}", answer);
}