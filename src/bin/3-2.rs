use std::{io, collections::HashMap};
use regex::Regex;

struct Number {
  value: u64,
  row: i64,
  col: i64,
  length: usize
}

fn is_star(text: &Vec<String>, row: i64, col: i64) -> bool {
  return row >= 0 && 
         col >= 0 && 
         row < text.len().try_into().unwrap() && 
         col < text[0].len().try_into().unwrap() &&
         text[row as usize].as_bytes()[col as usize] == b'*'
}

impl Number {
  fn update_if_needed(&self, gears: &mut HashMap<(i64, i64), (u64, u64)>, key: (i64, i64)) {
    let current = gears.get(&key);
    match current {
      None => {
        gears.insert(key, (self.value, 1));
      }
      Some(&(value, amount)) => {
        gears.insert(key, (self.value * value, amount + 1));
      }
    }
  }
  fn annotate_possible_gears(&self, text: &Vec<String>, gears: &mut HashMap<(i64, i64), (u64, u64)>) {
    for i in 0..=(self.length+1) {
      if is_star(&text, self.row-1, self.col + (i as i64) - 1) {
        let key = (self.row-1, self.col + (i as i64) - 1);
        self.update_if_needed(gears, key)
      }
      if is_star(&text, self.row+1, self.col + (i as i64) - 1) {
        let key = (self.row+1, self.col + (i as i64) - 1);
        self.update_if_needed(gears, key)
      }
    }
    if is_star(&text, self.row, self.col - 1) {
      let key = (self.row, self.col - 1);
      self.update_if_needed(gears, key)
    }
    if is_star(&text, self.row, self.col + (self.length as i64)) {
      let key = (self.row, self.col + (self.length as i64));
      self.update_if_needed(gears, key)
    }
  }
}

fn find_numbers(text: &str, numbers: &mut Vec<Number>, row: i64) {
  let re: Regex = Regex::new(r"([0-9]+)").unwrap();
  re.captures_iter(text).for_each(|capture| {
    let capture = capture.get(0).unwrap();
    numbers.push(Number {
      value: capture.as_str().parse::<u64>().unwrap(),
      row: row, 
      col: (capture.start() as i64),
      length: capture.len()
    });
  });
}

fn main() {
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  let mut numbers: Vec<Number> = Vec::new();
  let mut text: Vec<String> = Vec::new();
  for (row, line) in lines.enumerate() {
    let line = line.unwrap();
    find_numbers(&line, &mut numbers, row as i64);
    text.push(line);
  }

  let mut gears = HashMap::new();
  numbers.iter().for_each(|num| {
    num.annotate_possible_gears(&text, &mut gears);
  });
  gears.iter().for_each(|(_, (value, amount))| {
    if amount >= &2 {
      sum += value;
    }
  });
  println!("{}", sum);
}