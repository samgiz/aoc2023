use std::{io, collections::HashSet};

fn find_edges(i: usize, intersections: &Vec<(usize, usize)>, board: &Vec<Vec<u8>>) -> Vec<(usize, u64)> {
  let mut answer = Vec::new();
  let (row, col) = intersections[i];
  let mut visited = HashSet::new();
  visited.insert((row, col));
  let mut to_visit = vec!(((row, col), 0));
  while !to_visit.is_empty() {
    let ((row, col), length) = to_visit.pop().unwrap();
    if is_intersection(board, row, col) && (row, col) != intersections[i] {
      let neighbour = intersections.iter().position(|&(r, c)| r == row && c == col).unwrap();
      answer.push((neighbour, length));
      continue;
    }
    for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let row = row as i64 + drow;
      let col = col as i64 + dcol;
      if row < 0 || col < 0 || row >= board.len() as i64 || col >= board[0].len() as i64 {
        continue;
      }
      let row = row as usize;
      let col = col as usize;
      if visited.contains(&(row, col)) || board[row][col] == b'#' {
        continue;
      }
      visited.insert((row, col));
      to_visit.push(((row, col), length + 1));
    }
  }
  answer
}

fn is_intersection(board: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
  if row == 0 || row == board.len() - 1 {
    return true;
  }
  let mut num_paths = 0;
  for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    let row = row as i64 + drow;
    let col = col as i64 + dcol;
    if row < 0 || col < 0 || row >= board.len() as i64 || col >= board[0].len() as i64 {
      continue;
    }
    let row = row as usize;
    let col = col as usize;
    if board[row][col] == b'.' {
      num_paths += 1;
    }
  }
  return num_paths > 2;
}

fn find_longest_path(cur: usize, edges: &Vec<Vec<(usize, u64)>>, visited: &mut Vec<bool>) -> Option<u64> {
  if cur == visited.len() - 1 {
    return Some(0);
  }
  let mut answer = None;
  for &(next, cost) in edges[cur].iter() {
    if visited[next] {
      continue;
    }
    visited[next] = true;
    let best_length = find_longest_path(next, edges, visited);
    answer = match best_length {
      Some(best_length) => {
        match answer {
          None => Some(best_length + cost),
          Some(answer) => Some(std::cmp::max(best_length + cost, answer))
        }
      },
      None => answer
    };
    visited[next] = false;
  }
  answer
}

fn main() {
  let mut board = io::stdin().lines().map(|line| line.unwrap().as_bytes().to_vec()).collect::<Vec<_>>();
  for i in 0..board.len() {
    for j in 0..board[0].len() {
      if board[i][j] != b'.' && board[i][j] != b'#' {
        board[i][j] = b'.';
      }
    }
  }

  let mut intersections = Vec::new();
  for i in 0..board.len() {
    for j in 0..board[0].len() {
      if board[i][j] == b'.' && is_intersection(&board, i, j) {
        intersections.push((i, j));
      }
    }
  }
  let edges = (0..intersections.len()).map(|i| find_edges(i, &intersections, &board)).collect::<Vec<_>>();
  let start = 0;
  let mut visited = vec!(false; intersections.len());
  visited[start] = true;
  let longest_path = find_longest_path(start, &edges, &mut visited).unwrap();
  println!("{longest_path}");
}