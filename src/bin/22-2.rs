use std::io;

#[derive(Debug, Clone)]
struct Brick {
  x1: i64,
  y1: i64,
  z1: i64,
  x2: i64,
  y2: i64,
  z2: i64
}

impl Brick {
  fn overlaps_with(&self, other: &Brick) -> bool {
    ((self.x1 <= other.x1 && other.x1 <= self.x2) || (other.x1 <= self.x1 && self.x1 <= other.x2))
    && ((self.y1 <= other.y1 && other.y1 <= self.y2) || (other.y1 <= self.y1 && self.y1 <= other.y2))
  }
}

fn main() {
  let mut bricks = io::stdin().lines().map(|line| {
    let line = line.unwrap();
    let [xyz1, xyz2]: [&str; 2] = line.split('~').collect::<Vec<_>>().try_into().unwrap();
    let [x1, y1, z1]: [i64; 3] = xyz1.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let [x2, y2, z2]: [i64; 3] = xyz2.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    assert!(x1 <= x2);
    assert!(y1 <= y2);
    assert!(z1 <= z2);
    Brick {x1, y1, z1, x2, y2, z2}
  }).collect::<Vec<_>>();
  // sort by increasing z1 values
  bricks.sort_by(|a, b| a.z1.cmp(&b.z1));
  for i in 0..bricks.len() {
    let mut min_z1 = 1;
    for j in 0..i {
      if bricks[i].overlaps_with(&bricks[j]) {
        min_z1 = std::cmp::max(min_z1, bricks[j].z2+1);
      }
    }
    let shift_amount = bricks[i].z1 - min_z1;
    bricks[i].z1 -= shift_amount;
    bricks[i].z2 -= shift_amount;
  }
  // dbg!(bricks.clone());
  // for i in 0..bricks.len() {
  //   for j in 0..i {
  //     if bricks[i].overlaps_with(&bricks[j]) {
  //       dbg!("overlap");
  //       dbg!(i, j);
  //     }
  //   }
  // }
  let mut supported_by = vec!(0; bricks.len());
  for i in 0..bricks.len() {
    for j in 0..i {
      if bricks[i].overlaps_with(&bricks[j]) && bricks[j].z2 + 1 == bricks[i].z1 {
        supported_by[i] += 1;
      }
    }
  }
  let mut answer = 0;
  for i in 0..bricks.len() { // try to disintegrate this one
    let mut supported_by = supported_by.clone();
    let mut to_remove = vec!(i);
    while !to_remove.is_empty() {
      let top = to_remove.pop().unwrap();
      for j in i+1..bricks.len() {
        if bricks[top].overlaps_with(&bricks[j]) && bricks[top].z2 + 1 == bricks[j].z1 {
          supported_by[j] -= 1;
          if supported_by[j] == 0 {
            answer += 1;
            to_remove.push(j);
          }
        }
      }
    }
  }
  println!("{answer}");
}