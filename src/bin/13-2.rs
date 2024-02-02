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

fn get_row_split_index(board: &Vec<Vec<u8>>) -> Vec<u64> {
  let mut answer = Vec::new();
  for i in 0..board.len() - 1 {
    // Try splitting after row i
    let mut rows_above = board[..=i].to_vec();
    let rows_below = board[i + 1..].to_vec();
    rows_above.reverse();
    let num_to_check = std::cmp::min(rows_above.len(), rows_below.len());
    let rows_above = &rows_above[..num_to_check];
    let rows_below = &rows_below[..num_to_check];
    let mut num_diff = 0;
    for i in 0..rows_above.len() {
      for j in 0..rows_above[0].len() {
        if rows_above[i][j] != rows_below[i][j] {
          num_diff += 1;
        }
      }
    }
    if num_diff == 1 {
      answer.push((i + 1) as u64);
    }
  }
  answer
}

fn main() {
  let lines: Vec<Vec<u8>> = io::stdin()
    .lines()
    .map(|line| line.unwrap().as_bytes().to_vec())
    .collect();
  let boards = lines.split(|x| x.is_empty()).map(|x| x.to_vec());
  let scores = boards.map(|board| -> u64 {
    let row_indexes = get_row_split_index(&board);
    let board = rotate(board);
    let col_indexes = get_row_split_index(&board);
    100 * row_indexes.iter().sum::<u64>() + col_indexes.iter().sum::<u64>()
  });
  println!("{}", scores.sum::<u64>());
}
