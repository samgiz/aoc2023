use std::io;

fn dfs((row, col): (usize, usize), visited: &mut Vec<Vec<bool>>, board: &Vec<Vec<u8>>) -> Option<u64> {
  // dbg!(row, col);
  if row == board.len() - 1 {
    return Some(0);
  }
  let mut answer = None;
  let c = board[row][col];
  for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    let row = row as i64 + drow;
    let col = col as i64 + dcol;
    if row < 0 || col < 0 || row >= visited.len() as i64 || col >= visited[0].len() as i64 {
      continue;
    }
    let row = row as usize;
    let col = col as usize;
    if (c == b'v' && drow != 1) || (c == b'^' && drow != -1) || (c == b'<' && dcol != -1) || (c == b'>' && dcol != 1) {
      continue;
    }
    if visited[row][col] || board[row][col] == b'#' {
      continue;
    }
    visited[row][col] = true;
    let best_length = dfs((row, col), visited, board);
    answer = match best_length {
      Some(best_length) => {
        match answer {
          None => Some(best_length + 1),
          Some(answer) => Some(std::cmp::max(best_length + 1, answer))
        }
      },
      None => answer
    };
    visited[row][col] = false;
  }
  answer
}

fn main() {
  let board = io::stdin().lines().map(|line| line.unwrap().as_bytes().to_vec()).collect::<Vec<_>>();
  // Simple but stupid algorithm: try every path.
  let mut start: i64 = -1;
  for i in 0..board[0].len() {
    if board[0][i] == b'.' {
      start = i as i64;
      break;
    }
  }
  if start == -1 {
    panic!("No start found");
  }
  let start = start as usize;
  let mut visited = vec!(vec!(false; board[0].len()); board.len());
  visited[0][start] = true;
  let answer = dfs((0, start), &mut visited, &board).unwrap();
  println!("{answer}");
}