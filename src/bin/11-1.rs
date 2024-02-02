use std::io;

fn rotate(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
  for i in 0..matrix.len() {
    for (j, &value) in matrix[i].iter().enumerate() {
      new_matrix[j][matrix.len()-1-i] = value;
    }
  }
  new_matrix
}

fn expand(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_matrix = Vec::new();
  matrix.iter().for_each(|x| {
    new_matrix.push(x.clone());
    if x.iter().all(|&x| x == b'.') {
      new_matrix.push(x.clone());
    }
  });
  new_matrix
}

fn main() {
  let initial_board: Vec<Vec<_>> = io::stdin().lines().map(|line|line.unwrap().as_bytes().to_vec()).collect();
  let board = expand(rotate(expand(initial_board)));
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
      answer += (a.0 - b.0).abs() + (a.1 - b.1).abs();
    }
  }
  println!("{}", answer / 2);
}