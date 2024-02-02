use std::io;

// 5  10  13  14  13  10  5
//   5   3   1   -1   -3  -5
//    -2   -2   -2   -2   -2
//       0   0   0   0

fn main() {
  let lines = io::stdin().lines();
  let mut answer = 0;
  for line in lines {
    let line = line.unwrap();
    let mut sum = 0;
    let mut current: Vec<_> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut flag = 1;
    while !current.is_empty() && !current.iter().all(|&x| x == 0) {
      sum += current[0] * flag;
      flag *= -1;
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
