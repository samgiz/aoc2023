use std::io;

struct Cubes {
  red: u64,
  green: u64,
  blue: u64,
}

impl Cubes {
  fn contains(&self, other: &Cubes) -> bool {
    return self.red >= other.red && self.blue >= other.blue && self.green >= other.green;
  }
  fn new() -> Cubes {
    return Cubes {red:0, green:0, blue:0};
  }
}

fn main() {
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  let main_cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
  };
  for line in lines {
    let line = line.unwrap();
    let split_on_colon: Vec<&str> = line.split(": ").collect();
    assert_eq!(split_on_colon.len(), 2);
    let game_text = split_on_colon[0];
    let game_split: Vec<&str> = game_text.split(' ').collect();
    assert_eq!(game_split.len(), 2);
    let game_id = game_split[1].parse::<u64>().unwrap();
    let cubes_description = split_on_colon[1];
    let cubes_descriptions: Vec<&str> = cubes_description.split("; ").collect();
    let mut contains = true;
    cubes_descriptions.iter().for_each(|desc| {
      let split_on_commas: Vec<&str> = desc.split(", ").collect();
      let mut cubes = Cubes::new();
      split_on_commas.iter().for_each(|desc| {
        let split: Vec<&str> = desc.split(' ').collect();
        assert_eq!(split.len(), 2);
        let key = split[1];
        let value = split[0].parse::<u64>().unwrap();
        match key {
          "blue" => cubes.blue = value,
          "red" => cubes.red = value,
          "green" => cubes.green = value,
          _ => panic!("Neither blue, red or green")
        }
      });
      contains &= main_cubes.contains(&cubes)
    });
    if contains {
      sum += game_id;
    }
  }
  println!("{}", sum);
}