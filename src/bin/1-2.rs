use std::{io, collections::HashMap};

fn find_first(string_to_digit: &HashMap<&str, u64>, text: &str) -> u64 {
  let mut return_value = 0;
  let mut earliest = text.len() + 1;
  for (key, value) in string_to_digit.into_iter() {
    let index = text.find(key);
    match index {
      Some(index) => {
        if earliest > index {
          earliest = index;
          return_value = *value;
        }
      }
      _ => ()
    }
  }
  return return_value;
}

fn find_last(string_to_digit: &HashMap<&str, u64>, text: &str) -> u64 {
  let mut return_value = 0;
  let mut latest = text.len() + 1;
  for (key, value) in string_to_digit.into_iter() {
    let index = text.rfind(key);
    match index {
      Some(index) => {
        if latest < index || latest == text.len() + 1 {
          latest = index;
          return_value = *value;
        }
      }
      _ => ()
    }
  }
  return return_value;
}

fn main() {
  let mut string_to_digit: HashMap<&str, u64> = HashMap::new();
  string_to_digit.insert("zero", 0);
  string_to_digit.insert("0", 0);
  string_to_digit.insert("one", 1);
  string_to_digit.insert("1", 1);
  string_to_digit.insert("two", 2);
  string_to_digit.insert("2", 2);
  string_to_digit.insert("three", 3);
  string_to_digit.insert("3", 3);
  string_to_digit.insert("four", 4);
  string_to_digit.insert("4", 4);
  string_to_digit.insert("five", 5);
  string_to_digit.insert("5", 5);
  string_to_digit.insert("six", 6);
  string_to_digit.insert("6", 6);
  string_to_digit.insert("seven", 7);
  string_to_digit.insert("7", 7);
  string_to_digit.insert("eight", 8);
  string_to_digit.insert("8", 8);
  string_to_digit.insert("nine", 9);
  string_to_digit.insert("9", 9);
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  for line in lines {
    let line = line.unwrap();
    sum += find_first(&string_to_digit, &line) * 10 + find_last(&string_to_digit, &line);
  }
  println!("{}", sum);
}