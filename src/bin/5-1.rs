use std::collections::BTreeMap;
use std::io;

fn remap(seed: i64, map: &BTreeMap<i64, (i64, i64)>) -> i64 {
  let smaller_elements = map.range(..=seed);
  match smaller_elements.max() {
    None => seed,
    Some((starting_before, (amount, starting_after))) => {
      if seed < starting_before + amount {
        starting_after + (seed - starting_before)
      } else {
        seed
      }
    }
  }
}

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  line.pop();
  let [_, seeds]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
  let mut seeds = seeds
    .split(' ')
    .map(|x| x.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
  let lines = io::stdin().lines();
  let mut map: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
  for line in lines {
    let line = line.unwrap();
    if line.ends_with('"') {
      continue;
    }
    if line.is_empty() {
      // We want to re-map our seeds with the map that we've constructed so far
      seeds = seeds.iter().map(|seed| remap(*seed, &map)).collect();
      map.clear();
    } else {
      // update the map with the new entry
      let [after, before, amount]: [i64; 3] = line
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .try_into()
        .unwrap();
      map.insert(before, (amount, after));
    }
  }
  let answer = seeds.iter().min().unwrap();
  println!("{}", answer);
}
