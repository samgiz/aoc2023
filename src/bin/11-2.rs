use std::{collections::BTreeSet, io};

fn rotate(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
  for i in 0..matrix.len() {
    for (j, &value) in matrix[i].iter().enumerate() {
      new_matrix[j][matrix.len() - 1 - i] = value;
      // 2x2 matrix
      // 0,0 => 0,1
    }
  }
  new_matrix
}

fn get_empty_rows(matrix: &[Vec<u8>]) -> BTreeSet<usize> {
  let mut empty_rows = BTreeSet::new();
  matrix.iter().enumerate().for_each(|(i, x)| {
    if x.iter().all(|&x| x == b'.') {
      empty_rows.insert(i);
    }
  });
  empty_rows
}

fn main() {
  let board: Vec<Vec<_>> = io::stdin()
    .lines()
    .map(|line| line.unwrap().as_bytes().to_vec())
    .collect();
  let empty_rows = get_empty_rows(&board);
  let board = rotate(board);
  let empty_columns = get_empty_rows(&board);
  let board = rotate(rotate(rotate(board)));
  let mut galaxy_locations = Vec::new();
  for i in 0..board.len() {
    for j in 0..board[0].len() {
      if board[i][j] == b'#' {
        galaxy_locations.push((i as i64, j as i64));
      }
    }
  }
  let mut answer = 0;
  for a in galaxy_locations.iter() {
    for b in galaxy_locations.iter() {
      let rows_in_between = empty_rows
        .range((std::cmp::min(a.0, b.0) as usize)..(std::cmp::max(a.0, b.0) as usize))
        .count();
      let columns_in_between = empty_columns
        .range((std::cmp::min(a.1, b.1) as usize)..(std::cmp::max(a.1, b.1) as usize))
        .count();
      answer += (a.0 - b.0).abs()
        + (a.1 - b.1).abs()
        + (rows_in_between as i64 + columns_in_between as i64) * 999_999;
    }
  }
  println!("{}", answer / 2);
}
