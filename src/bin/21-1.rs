use std::{
  collections::{HashMap, VecDeque},
  io,
};

fn main() {
  let map = io::stdin()
    .lines()
    .map(|line| line.unwrap().as_bytes().to_vec())
    .collect::<Vec<_>>();
  let starting_position = (|| {
    for i in 0..map.len() {
      for j in 0..map[0].len() {
        if map[i][j] == b'S' {
          return (i, j);
        }
      }
    }
    panic!("Couldn't find S in map");
  })();
  let mut visited = HashMap::new();
  let mut to_visit = VecDeque::new();
  to_visit.push_back(starting_position);
  visited.insert(starting_position, 0_u64);

  while !to_visit.is_empty() {
    let (row, col) = to_visit.pop_front().unwrap();
    let amount = visited[&(row, col)];
    if amount == 64 {
      continue;
    }
    for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let row = row as i64 + drow;
      let col = col as i64 + dcol;
      if row >= 0 && row < map.len() as i64 && col >= 0 && col < map[0].len() as i64 {
        let row = row as usize;
        let col = col as usize;
        if map[row][col] == b'.' && !visited.contains_key(&(row, col)) {
          visited.insert((row, col), amount + 1);
          to_visit.push_back((row, col));
        }
      }
    }
  }

  let answer = visited
    .iter()
    .filter(|&(_, &num_steps)| num_steps % 2 == 0)
    .count();
  println!("{answer}");
}
