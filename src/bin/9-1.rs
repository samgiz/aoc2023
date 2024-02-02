use std::io;

fn main() {
  let lines = io::stdin().lines();
  let mut answer = 0;
  for line in lines {
    let line = line.unwrap();
    let mut sum = 0;
    let mut current: Vec<_> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
    while !current.is_empty() && !current.iter().all(|&x| x == 0) {
      sum += current.last().unwrap();
      current = current[..current.len() - 1]
        .iter()
        .zip(current[1..].iter())
        .map(|(l, r)| r - l)
        .collect();
    }
    answer += sum;
  }
  println!("{}", answer);
}
