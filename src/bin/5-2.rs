use std::io;
use std::collections::BTreeMap;

fn remap(mut seed: (i64, i64), map: &BTreeMap<i64, (i64, i64)>) -> Vec<(i64, i64)> {
  let smaller_elements = map.range(..=seed.0);
  let mut answer = Vec::new();
  let iter_over = match smaller_elements.max() {
    None => map.range(..),
    Some((earliest_before, _)) => map.range(earliest_before..)
  };
  for (before, (amount, after)) in iter_over {
    if *before > seed.0 + seed.1 || seed.1 == 0 {
      break;
    }
    if before + amount <= seed.0 {
      continue;
    }
    if *before > seed.0 {
      // seed.0 until before should end up in answer
      answer.push((seed.0, before - seed.0));
      seed = (*before, seed.1 - (before - seed.0));
    }
    //  seed.0 >= before
    if before + amount < seed.0 + seed.1 {
      answer.push((after + (seed.0 - before), before + amount - seed.0));
      seed = (before + amount, seed.1 - (before+amount - seed.0));
    } else { // before + amount >= seed.0 + seed.1
      answer.push((after + (seed.0 - before), seed.1));
      seed = (0, 0)
    }
  }
  if seed.1 > 0 {
    answer.push(seed);
  }
  return answer;
}

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  line.pop();
  let [_, ungrouped_seeds]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
  let ungrouped_seeds = ungrouped_seeds.split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let mut seeds = Vec::new();
  for i in (0..ungrouped_seeds.len()).step_by(2) {
    seeds.push((ungrouped_seeds[i], ungrouped_seeds[i+1]));
  }
  let lines = io::stdin().lines();
  let mut map: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
  for line in lines {
    let line = line.unwrap();
    if line.ends_with(":") {
      continue;
    }
    if line == "" {
      // We want to re-map our seeds with the map that we've constructed so far
      seeds = seeds.iter().flat_map(|seed| remap(*seed, &map)).collect();
      map.clear();
    } else {
      // update the map with the new entry
      let [after, before, amount]: [i64; 3] = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>().try_into().unwrap();
      map.insert(before, (amount, after));
    }
  }
  let answer = seeds.iter().min().unwrap().0;
  println!("{}", answer);
}