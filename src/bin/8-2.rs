use std::{io, collections::HashMap};
use itertools::Itertools;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
  if a == 0 {
      (b, 0, 1)
  } else {
      let (g, x, y) = egcd(b % a, a);
      (g, y - (b / a) * x, x)
  }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
  let (g, x, _) = egcd(x, n);
  if g == 1 {
      Some((x % n + n) % n)
  } else {
      None
  }
}

fn chinese_remainder(residues: &[&i64], modulii: &[i64]) -> Option<i64> {
  let prod = modulii.iter().product::<i64>();

  let mut sum = 0;

  for (&residue, &modulus) in residues.iter().zip(modulii) {
      let p = prod / modulus;
      let inv = mod_inv(p, modulus);
      match inv {
        None => {return None;}
        Some(val) => {sum += residue * val * p}
      }
      
  }

  Some(sum % prod)
}

fn earliest_start(cycle_lengths: &[i64], offsets: &[&i64]) -> Option<i64> {
  chinese_remainder(offsets, cycle_lengths)
}

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  io::stdin().read_line(&mut line).ok();

  let directions = line.trim().as_bytes();

  let mut next_left: HashMap<String, String> = HashMap::new();
  let mut next_right: HashMap<String, String> = HashMap::new();
  let lines = io::stdin().lines();
  let mut labels = Vec::new();
  for line in lines {
    let line = line.unwrap();
    let [current, left_right]: [String; 2] = line.split(" = ").map(|x| x.to_string()).collect::<Vec<String>>().try_into().unwrap();
    let left_right = left_right[1..left_right.len()-1].to_string();
    let [left, right]: [String; 2] = left_right.split(", ").map(|x| x.to_string()).collect::<Vec<String>>().try_into().unwrap();
    let current = current.to_string().clone();
    labels.push(current.clone());
    next_left.insert(current.clone(), left);
    next_right.insert(current, right);
  }
  // Make the labels immutable
  let labels = labels;
  let current = labels.iter().filter(|x|x.ends_with('A')).collect::<Vec<&String>>();
  // We want to collect the following:
  // 1. the period of repetition.
  // 2. the start of the loop
  // 3. within the loop, the position / end leaf combinations that we encounter.
  let dp: Vec<_> = current.iter().map(|starting_label| {
    let mut visited: HashMap<(String, _), _> = HashMap::new();
    visited.insert(((*starting_label).clone(), directions.len()-1), 0);
    let mut label = *starting_label;
    for i in 1.. {
      let dir_index = (i-1) % directions.len();
      let move_index = i % directions.len();
      let direction = directions[dir_index];
      label = match direction {
        b'R' => &next_right[label],
        b'L' => &next_left[label],
        _ => panic!("this is not supposed to happen")
      };
      if visited.contains_key(&(label.clone(), move_index)) {
        // We got to a cycle
        let previous_visit = visited[&(label.clone(), move_index)];
        let cycle_length = (i - previous_visit) / directions.len();
        // Also need a mapping move_index => end labels and their first visit time at that index
        let mut end_label_mapping: HashMap<usize, Vec<_>> = HashMap::new();
        visited.iter().for_each(|((label, index), visit_time)| {
          if label.ends_with('Z') && (i - visit_time) <= cycle_length {
            if !end_label_mapping.contains_key(index) {
              end_label_mapping.insert(*index, Vec::new());
            }
            let v = end_label_mapping.get_mut(index).unwrap();
            v.push((visit_time % cycle_length) as i64);
          }
        });
        return (cycle_length, end_label_mapping);
      }
      visited.insert((label.clone(), move_index), i);
    }
    panic!("not supposed to reach this!");
  }).collect();
  // Then we will go over all positions in the LR sequence and apply
  // Chinese remainder theorem over all possible combinations that might end at that sequence.
  let mut cycle_lengths: Vec<_> = dp.iter().map(|(cycle_length, _)| *cycle_length as i64).collect();
  cycle_lengths.push(directions.len() as i64);
  let mut answer = -1;
  for index in 0..directions.len() {
    let empty = Vec::new();
    let offsets: Vec<_> = dp.iter().map(|(_, mapping)| if mapping.contains_key(&index) {&mapping[&index]} else {&empty}).collect();
    let prod = offsets.iter().map(|x|x.iter()).multi_cartesian_product();
    for offsets in prod {
      let mut offsets = offsets.clone();
      let index = index as i64;
      offsets.push(&index);
      assert_eq!(cycle_lengths.len(), offsets.len());
d      answer = match earliest_start(cycle_lengths.as_slice(), offsets.as_slice()) {
        None => answer,
        Some(val) => if answer == -1 {val} else {std::cmp::min(val, answer)}
      };
    }
  }
  println!("{}", answer);
}