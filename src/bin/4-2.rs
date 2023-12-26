use std::io;
use std::collections::HashSet;

fn main() {
  let lines = io::stdin().lines();
  let mut points: Vec<u64> = Vec::new();
  for line in lines {
    let line = line.unwrap();
    let [_, cards]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
    let [winning, ours]: [&str; 2] = cards.split(" | ").collect::<Vec<&str>>().try_into().unwrap();
    let winning: HashSet<u64> = winning.split(' ').filter(|x| x != &"").map(|x| x.parse::<u64>().unwrap()).collect();
    let ours: Vec<u64> = ours.split(' ').filter(|x| x != &"").map(|x| x.parse::<u64>().unwrap()).collect();
    let point = ours.iter().fold(0, |sum, x| sum + (if winning.contains(x) {1} else {0}));
    points.push(point);
  }
  let mut amounts: Vec<u64> = vec![1; points.len()];
  for (i, point) in points.iter().enumerate() {
    for j in 1..=*point {
      amounts[i+(j as usize)] += amounts[i];
    }
  }
  let sum: u64 = amounts.iter().sum();
  println!("{}", sum);
}