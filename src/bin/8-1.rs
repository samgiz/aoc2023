use std::{collections::HashMap, io};

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  io::stdin().read_line(&mut line).ok();

  let directions = line.trim().as_bytes();

  let mut next_left: HashMap<String, String> = HashMap::new();
  let mut next_right: HashMap<String, String> = HashMap::new();
  let lines = io::stdin().lines();
  for line in lines {
    let line = line.unwrap();
    let [current, left_right]: [String; 2] = line
      .split(" = ")
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .try_into()
      .unwrap();
    let left_right = left_right[1..left_right.len() - 1].to_string();
    let [left, right]: [String; 2] = left_right
      .split(", ")
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .try_into()
      .unwrap();
    let current = current.to_string().clone();
    next_left.insert(current.clone(), left);
    next_right.insert(current, right);
  }
  let mut current = "AAA";
  for i in 1.. {
    let direction = directions[(i - 1) % directions.len()];
    current = match direction {
      b'R' => &next_right[current],
      b'L' => &next_left[current],
      _ => panic!("this is not supposed to happen"),
    };
    if current == "ZZZ" {
      println!("{i}");
      break;
    }
  }
}
