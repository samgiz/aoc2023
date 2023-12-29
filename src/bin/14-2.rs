use std::io;

fn rotate(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
  for i in 0..matrix.len() {
    for j in 0..matrix[0].len() {
      new_matrix[j][matrix.len()-1-i] = matrix[i][j]
      // 2x2 matrix
      // 0,0 => 0,1
    }
  }
  new_matrix
}
fn tumble(board: &mut Vec<Vec<u8>>) {
  board.iter_mut().for_each(|row| {
    let mut empty_index = 0;
    for index in 0..row.len() {
      assert!(empty_index <= index);
      if row[index] == b'O' {
        row[index] = b'.';
        while row[empty_index] == b'#' {
          empty_index += 1;
        }
        row[empty_index] = b'O';
        empty_index += 1;
      } else if row[index] == b'#' {
        empty_index = index + 1;
      }
    }
  });
}

fn cycle(board: &mut Vec<Vec<u8>>) {
  for _ in 0..4 {
    tumble(board);
    *board = rotate(board);
  }
}

fn main() {
  let mut board: Vec<Vec<u8>> = io::stdin().lines().map(|line| line.unwrap().as_bytes().to_vec()).collect();
  for _ in 0..3 {
    board = rotate(&board);
  }
  // Do cycles until we (hopefully) get to a cycle
  // Increase this value if it's not enough.
  let num_initial_cycles = 200;
  for _ in 0..num_initial_cycles {
    cycle(&mut board);
  }
  let board_to_reach = board.clone();
  cycle(&mut board);
  let mut cycle_length = 1;
  while board != board_to_reach {
    cycle_length += 1;
    cycle(&mut board);
  }
  // At this point we've cycled num_initial_cycles + cycle_length times
  // Cycle the remaining amount % cycle_length
  for _ in 0..((1_000_000_000-num_initial_cycles) % cycle_length) {
    cycle(&mut board);
  }

  // Compute the answer
  // North is facing left currently
  let mut answer = 0;
  for i in 0..board.len() {
    for j in 0..board[0].len() {
      if board[i][j] == b'O' {
        answer += board[0].len() - j;
      }
    }
  }
  println!("{}", answer);
}