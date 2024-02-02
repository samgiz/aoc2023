use std::io;
use regex::Regex;

struct Number {
  value: u64,
  row: i64,
  col: i64,
  length: usize
}

fn is_symbol(text: &Vec<String>, row: i64, col: i64) -> bool {
  return row >= 0 && 
         col >= 0 && 
         row < text.len().try_into().unwrap() && 
         col < text[0].len().try_into().unwrap() &&
         text[row as usize].as_bytes()[col as usize] != b'.' &&
         !text[row as usize].as_bytes()[col as usize].is_ascii_digit()
}

impl Number {
  fn is_part(&self, text: &Vec<String>) -> bool {
    for i in 0..=(self.length+1) {
      if is_symbol(text, self.row-1, self.col + (i as i64) - 1) {
        return true;
      }
      if is_symbol(text, self.row+1, self.col + (i as i64) - 1) {
        return true;
      }
    }
    if is_symbol(text, self.row, self.col - 1) {
      return true;
    }
    if is_symbol(text, self.row, self.col + (self.length as i64)) {
      return true;
    }
    false
  }
}

fn find_numbers(text: &str, numbers: &mut Vec<Number>, row: i64) {
  let re: Regex = Regex::new(r"([0-9]+)").unwrap();
  re.captures_iter(text).for_each(|capture| {
    let capture = capture.get(0).unwrap();
    numbers.push(Number {
      value: capture.as_str().parse::<u64>().unwrap(),
      row, 
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
  numbers.iter().for_each(|num| {
    if num.is_part(&text) {
      sum += num.value;
    }
  });
  println!("{}", sum);
}