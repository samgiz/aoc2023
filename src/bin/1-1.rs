use std::io;

fn main() {
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  for line in lines {
    let line = line.unwrap();
    let filtered: Vec<u8> = line.bytes().into_iter().filter(|x| x >= &b'0' && x <= &b'9').collect();
    let first_digit = filtered[0] - b'0';
    let second_digit = filtered.last().unwrap() - b'0';
    sum += u64::from(first_digit * 10 + second_digit);
  }
  println!("{}", sum);
}