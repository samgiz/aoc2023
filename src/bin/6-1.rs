use std::io;

fn read_vec_of_nums() -> Vec<u64> {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let [_, numbers]: [&str; 2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
  numbers.trim().split(' ').filter(|num| num != &"").map(|num| {
    num.parse::<u64>().unwrap()
  }).collect()
}

fn main() {
  let times = read_vec_of_nums();
  let distances = read_vec_of_nums();
  // We're going to assume there's always at least one way to win.
  let mut answer = 1;
  for (i, time) in times.iter().enumerate() {
    let distance = distances[i];
    let mut current_race_answer = 0; 
    for time_waiting in 0..=*time {
      let time_driving = time - time_waiting;
      let new_distance = time_waiting * time_driving;
      if new_distance > distance {
        current_race_answer += 1;
      }
    }
    answer *= current_race_answer;
  }
  println!("{}", answer);
}