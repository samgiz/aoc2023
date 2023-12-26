use std::io;

fn read_vec_of_nums() -> u64 {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let [_, numbers]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
  let stringified_num = numbers.trim().split(' ').filter(|num| num != &"").map(String::from).reduce(|acc, s| format!("{acc}{s}")).unwrap_or_default();
  stringified_num.parse::<u64>().unwrap()
}

fn main() {
  let time = read_vec_of_nums();
  let distance = read_vec_of_nums();
  // We're going to assume there's always at least one way to win.
  let mut answer = 0;
  for time_waiting in 0..=time {
    let time_driving = time - time_waiting;
    let new_distance = time_waiting * time_driving;
    if new_distance > distance {
      answer += 1;
    }
  }
  println!("{}", answer);
}