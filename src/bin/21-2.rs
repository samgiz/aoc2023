use std::{
  collections::{HashMap, VecDeque},
  io,
};

// returned_vec[i] denotes how many positions are reachable in i steps if we start at starting_position
// returned_vec[i] = returned_vec[returned_vec.len() - 1] for i >= returned_vec.len()
fn bfs_from(starting_position: (usize, usize), map: &Vec<Vec<u8>>) -> Vec<u64> {
  let mut visited = HashMap::new();
  let mut to_visit = VecDeque::new();
  to_visit.push_back(starting_position);
  visited.insert(starting_position, 0_u64);
  while !to_visit.is_empty() {
    let (row, col) = to_visit.pop_front().unwrap();
    let amount = visited[&(row, col)];
    for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let row = row as i64 + drow as i64;
      let col = col as i64 + dcol as i64;
      if row < 0 || col < 0 || row >= map.len() as i64 || col >= map[0].len() as i64 {
        continue;
      }
      let row = row as usize;
      let col = col as usize;
      if (map[row][col] == b'.' || map[row][col] == b'S') && !visited.contains_key(&(row, col)) {
        visited.insert((row, col), amount + 1);
        to_visit.push_back((row, col));
      }
    }
  }
  // find max number of steps
  let max_num_steps = *visited.values().max().unwrap();
  let mut answers = vec![0_u64; (max_num_steps + 1) as usize];
  visited
    .iter()
    .for_each(|(_, &steps)| answers[steps as usize] += 1);
  for i in 2..answers.len() {
    answers[i] += answers[i - 2];
  }
  answers
}

fn answer_from_corner(
  mut sideways_amount: i64,
  num_reachable: &Vec<u64>,
  map: &Vec<Vec<u8>>,
  total_reachable_in_block: u64,
) -> u64 {
  let mut answer = 0;
  while sideways_amount >= 0 {
    let mut amount = sideways_amount as u64 % map.len() as u64;
    let mut block = 1 + sideways_amount as u64 / map.len() as u64;
    while block >= 1 && amount < num_reachable.len() as u64 - 1 {
      answer += num_reachable[amount as usize];
      amount += map.len() as u64;
      block -= 1;
    }
    answer += block * total_reachable_in_block;
    sideways_amount -= map[0].len() as i64;
  }
  answer
}

fn answer_vertically(
  amount: u64,
  num_reachable: &Vec<u64>,
  map: &Vec<Vec<u8>>,
  total_reachable_in_block: u64,
) -> u64 {
  let mut answer = 0;
  // move as far as we can down
  let mut block = 1 + amount / map.len() as u64;
  let mut amount = amount % map.len() as u64;
  while amount < num_reachable.len() as u64 - 1 {
    answer += num_reachable[amount as usize];
    amount += map.len() as u64;
    block -= 1;
  }
  answer += block * total_reachable_in_block;
  answer
}

fn answer_horizontally(
  amount: u64,
  num_reachable: &[u64],
  map: &[Vec<u8>],
  total_reachable_in_block: u64,
) -> u64 {
  let mut answer = 0;
  // move as far as we can down
  let mut block = 1 + amount / map[0].len() as u64;
  let mut amount = amount % map[0].len() as u64;
  while amount < num_reachable.len() as u64 - 1 {
    answer += num_reachable[amount as usize];
    amount += map[0].len() as u64;
    block -= 1;
  }
  answer += block * total_reachable_in_block;
  answer
}

fn expand(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_map = vec![vec!(b'.'; map[0].len() * 2); map.len() * 2];
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      new_map[i][j] = map[i][j];
      if map[i][j] == b'S' {
        new_map[i][j + map[0].len()] = b'.';
        new_map[i + map.len()][j + map[0].len()] = b'.';
        new_map[i + map.len()][j] = b'.';
      } else {
        new_map[i][j + map[0].len()] = map[i][j];
        new_map[i + map.len()][j + map[0].len()] = map[i][j];
        new_map[i + map.len()][j] = map[i][j];
      }
    }
  }
  new_map
}

fn total_reachable(even: u64, odd: u64, i: usize, j: usize) -> u64 {
  if i + j % 2 == 0 {
    even
  } else {
    odd
  }
}

fn main() {
  let map = io::stdin()
    .lines()
    .map(|line| line.unwrap().as_bytes().to_vec())
    .collect::<Vec<_>>();
  let map = expand(map);
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

  let from_top_center = bfs_from((0, starting_position.1), &map);
  let from_bottom_center = bfs_from((map.len() - 1, starting_position.1), &map);
  let from_center_left = bfs_from((starting_position.0, 0), &map);
  let from_center_right = bfs_from((starting_position.0, map[0].len() - 1), &map);

  let from_top_left = bfs_from((0, 0), &map);
  let from_top_right = bfs_from((0, map[0].len() - 1), &map);
  let from_bottom_left = bfs_from((map.len() - 1, 0), &map);
  let from_bottom_right = bfs_from((map.len() - 1, map[0].len() - 1), &map);

  let total_steps = 26501365;
  // All cells in the start block are reachable
  let odd_reachable = if from_top_left.len() % 2 == 0 {
    from_top_left[from_top_left.len() - 1]
  } else {
    from_top_left[from_top_left.len() - 2]
  };

  let even_reachable = if from_top_left.len() % 2 == 1 {
    from_top_left[from_top_left.len() - 1]
  } else {
    from_top_left[from_top_left.len() - 2]
  };
  let mut answer: u64 = total_reachable(
    even_reachable,
    odd_reachable,
    starting_position.0,
    starting_position.1,
  );
  // down
  // S to top middle of the next block
  let amount = total_steps - (map.len() - starting_position.0) as u64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    map.len() - starting_position.0,
    starting_position.1,
  );
  answer += answer_vertically(amount, &from_top_center, &map, num_reachable);
  // up
  // S to bottom middle of the next block
  let amount = total_steps - (starting_position.0 + 1) as u64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    map.len() - 1 + starting_position.0 + 1,
    starting_position.1,
  );
  answer += answer_vertically(amount, &from_bottom_center, &map, num_reachable);
  // left
  // S to middle right of the next block
  let amount = total_steps - (starting_position.1 + 1) as u64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    starting_position.0 + (starting_position.1 + 1),
    map[0].len() - 1,
  );
  answer += answer_horizontally(amount, &from_center_right, &map, num_reachable);
  // right
  // S to middle left of the next block
  let amount = total_steps - (map[0].len() - starting_position.1) as u64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    starting_position.0 + (map[0].len() - starting_position.1),
    0,
  );
  answer += answer_horizontally(amount, &from_center_left, &map, num_reachable);

  // down left
  let left_amount = total_steps as i64
    - (map.len() - starting_position.0) as i64
    - (starting_position.1 + 1) as i64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    (map.len() - starting_position.0) + (starting_position.1 + 1),
    map[0].len() - 1,
  );
  answer += answer_from_corner(left_amount, &from_top_right, &map, num_reachable);
  // down right
  let right_amount = total_steps as i64
    - (map.len() - starting_position.0) as i64
    - (map[0].len() - starting_position.1) as i64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    (map.len() - starting_position.0) + (map[0].len() - starting_position.1),
    0,
  );
  answer += answer_from_corner(right_amount, &from_top_left, &map, num_reachable);
  // up left
  let left_amount =
    total_steps as i64 - (starting_position.0 + 1) as i64 - (starting_position.1 + 1) as i64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    map.len() - 1 + (starting_position.0 + 1) + (starting_position.1 + 1),
    map[0].len() - 1,
  );
  answer += answer_from_corner(left_amount, &from_bottom_right, &map, num_reachable);
  // up right
  let right_amount = total_steps as i64
    - (starting_position.0 + 1) as i64
    - (map[0].len() - starting_position.1) as i64;
  let num_reachable = total_reachable(
    even_reachable,
    odd_reachable,
    map.len() - 1 + (starting_position.0 + 1) + (map[0].len() - starting_position.1),
    0,
  );
  answer += answer_from_corner(right_amount, &from_bottom_left, &map, num_reachable);
  println!("{answer}");
}
