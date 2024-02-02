use std::io;

struct Hail {
  px: f64,
  py: f64,
  vx: f64,
  vy: f64,
}

struct Box {
  min_x: i64,
  min_y: i64,
  max_x: i64,
  max_y: i64,
}

impl Hail {
  fn intersects(&self, other: &Hail, b: &Box) -> bool {
    // self.0 = (self.px, self.py)
    // self.1 = (self.px + self.vx, self.py + self.vy)
    let a1 = self.vy;
    let b1 = -self.vx;
    let c1 = a1 * self.px + b1 * self.py;

    let a2 = other.vy;
    let b2 = -other.vx;
    let c2 = a2 * other.px + b2 * other.py;

    let delta = a1 * b2 - a2 * b1;

    if delta == 0.0 {
      // Assuming that no two lines are on top of each other
      return false;
    }

    let x = (b2 * c1 - b1 * c2) / delta;
    let y = (a1 * c2 - a2 * c1) / delta;
    if (x < self.px && self.vx > 0.0) || (x > self.px && self.vx < 0.0) {
      return false;
    }
    if (x < other.px && other.vx > 0.0) || (x > other.px && other.vx < 0.0) {
      return false;
    }
    b.min_x as f64 <= x && x <= b.max_x as f64 && b.min_y as f64 <= y && y <= b.max_y as f64
  }
}

fn main() {
  let hails = io::stdin()
    .lines()
    .map(|line| {
      let line = line.unwrap();
      let [p, v]: [&str; 2] = line.split(" @ ").collect::<Vec<_>>().try_into().unwrap();
      let [px, py, _]: [f64; 3] = p
        .split(", ")
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
      let [vx, vy, _]: [f64; 3] = v
        .split(", ")
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
      // let mut gcd = gcd(vx, vy);
      // if vx / gcd < 0 || (vx == 0 && vy / gcd < 0) {
      //   gcd = -gcd;
      // }
      // let vx = vx / gcd;
      // let vy = vy / gcd;
      // let vz = vz / gcd;

      Hail { px, py, vx, vy }
    })
    .collect::<Vec<_>>();
  let mut answer: u64 = 0;
  let b = Box {
    min_y: 200000000000000,
    min_x: 200000000000000,
    max_y: 400000000000000,
    max_x: 400000000000000,
  };
  for i in 0..hails.len() {
    for j in (i + 1)..hails.len() {
      if hails[i].intersects(&hails[j], &b) {
        answer += 1;
      }
    }
  }
  println!("{answer}");
}
