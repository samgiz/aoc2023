use std::io;

struct Cubes {
  red: u64,
  green: u64,
  blue: u64,
}

impl Cubes {
  fn must_contain(&mut self, other: &Cubes) {
    self.red = std::cmp::max(self.red, other.red);
    self.green = std::cmp::max(self.green, other.green);
    self.blue = std::cmp::max(self.blue, other.blue);
  }
  fn new() -> Cubes {
    Cubes {red:0, green:0, blue:0}
  }
}

fn main() {
  let lines = io::stdin().lines();
  let mut sum: u64 = 0;
  for line in lines {
    let line = line.unwrap();
    let split_on_colon: Vec<&str> = line.split(": ").collect();
    assert_eq!(split_on_colon.len(), 2);
    let cubes_description = split_on_colon[1];
    let cubes_descriptions: Vec<&str> = cubes_description.split("; ").collect();
    let mut main_cubes = Cubes::new();
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
      main_cubes.must_contain(&cubes);
    });
    sum += main_cubes.blue * main_cubes.green * main_cubes.red;
  }
  println!("{}", sum);
}