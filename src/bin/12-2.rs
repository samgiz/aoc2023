use std::io;

fn get_chars(c: u8) -> Vec<usize> {
  match c {
    b'.' => vec![0],
    b'#' => vec![1],
    b'?' => vec![0, 1],
    _ => panic!("get_chars received an invalid character {c}"),
  }
}

fn main() {
  let mut answer = 0;
  let lines = io::stdin().lines();
  for line in lines {
    let line = line.unwrap();
    let [mut record, sizes]: [String; 2] = line
      .split(' ')
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .try_into()
      .unwrap();
    record.push('?');
    let mut record = record.repeat(5);
    record.pop();
    record.push('.');
    let record = record.as_bytes();
    let sizes: Vec<_> = sizes
      .split(',')
      .map(|x| x.parse::<u64>().unwrap())
      .collect();
    let sizes = sizes.repeat(5);
    // Do dynamic programming. State is represented by:
    // * Current position in the record
    // * How many sizes are fully to the left of the current position in the record
    // * The amount of consecutive # characters we've had
    // For each state we want to compute of the number of ways that state can be achieved.
    // The answer then becomes dp[record.len()][sizes.len()][Dot] + dp[record.len()][sizes.len()][Hash]
    let mut dp = vec![vec!(vec!(0; record.len() + 1); sizes.len() + 1); record.len() + 1];
    dp[0][0][0] = 1;
    for record_i in 0..record.len() {
      let current_chars = get_chars(record[record_i]);
      for size_i in 0..sizes.len() + 1 {
        for num_hashes in 0..record.len() {
          for &current_char in &current_chars {
            if current_char == 0 {
              // Dot
              if num_hashes == 0 {
                dp[record_i + 1][size_i][0] += dp[record_i][size_i][num_hashes];
              } else if size_i < sizes.len() && sizes[size_i] == num_hashes as u64 {
                dp[record_i + 1][size_i + 1][0] += dp[record_i][size_i][num_hashes];
              }
            } else {
              // Hash
              dp[record_i + 1][size_i][num_hashes + 1] += dp[record_i][size_i][num_hashes];
            }
          }
        }
      }
    }
    answer += dp[record.len()][sizes.len()][0];
  }
  println!("{}", answer);
}
