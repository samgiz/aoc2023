use std::io;

fn rotate(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
  for i in 0..matrix.len() {
    for (j, &value) in matrix[i].iter().enumerate() {
      new_matrix[j][matrix.len() - 1 - i] = value
      // 2x2 matrix
      // 0,0 => 0,1
    }
  }
  new_matrix
}

fn main() {
  let board: Vec<Vec<u8>> = io::stdin()
    .lines()
    .map(|line| line.unwrap().as_bytes().to_vec())
    .collect();
  let board = rotate(rotate(rotate(board)));
  let mut answer: u64 = 0;
  board.iter().for_each(|row| {
    let mut empty_index = 0;
    for index in 0..row.len() {
      if row[index] == b'O' {
        while row[empty_index] == b'#' {
          empty_index += 1;
        }
        answer += (row.len() - empty_index) as u64;
        empty_index += 1;
      } else if row[index] == b'#' {
        empty_index = index + 1;
      }
    }
  });
  println!("{}", answer);
}
