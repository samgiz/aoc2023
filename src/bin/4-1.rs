use std::collections::HashSet;
use std::io;

fn main() {
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  for line in lines {
    let line = line.unwrap();
    let [_, cards]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
    let [winning, ours]: [&str; 2] = cards
      .split(" | ")
      .collect::<Vec<&str>>()
      .try_into()
      .unwrap();
    let winning: HashSet<u64> = winning
      .split(' ')
      .filter(|x| x != &"")
      .map(|x| x.parse::<u64>().unwrap())
      .collect();
    let ours: Vec<u64> = ours
      .split(' ')
      .filter(|x| x != &"")
      .map(|x| x.parse::<u64>().unwrap())
      .collect();
    let amount_winning = ours
      .iter()
      .fold(0, |sum, x| sum + (if winning.contains(x) { 1 } else { 0 }));
    if amount_winning > 0 {
      sum += 1 << (amount_winning - 1);
    }
  }
  println!("{}", sum);
}
