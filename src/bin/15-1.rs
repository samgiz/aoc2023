use std::io;

fn hash(x: &[u8]) -> u64 {
  let mut value: u16 = 0;
  for &i in x {
    value = (value + i as u16) * 17 % 256;
  }
  value as u64
}

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let steps = line.trim().as_bytes().split(|&x|x == b',');
  let hashes = steps.map(|x| hash(x));
  let hash_sum: u64 = hashes.sum();
  println!("{}", hash_sum);
}