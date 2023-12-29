use std::io;

#[derive(Debug, Clone)]
struct Lens {
  label: Vec<u8>,
  strength: u64
}

fn hash(x: &[u8]) -> usize {
  let mut value: u16 = 0;
  for &i in x {
    value = (value + i as u16) * 17 % 256;
  }
  value as usize
}

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let steps = line.trim().as_bytes().split(|&x|x == b',');
  // https://stackoverflow.com/a/77104684/8791653
  let mut boxes: [Vec<Lens>; 256] = vec![Vec::new(); 256].try_into().expect("static");
  for step in steps {
    if step.last().unwrap() == &b'-' {
      let label = &step[..step.len()-1];
      let bucket_id = hash(label);
      boxes[bucket_id].retain(|x|x.label != label);
    } else {
      let strength = (step[step.len()-1] - b'0') as u64;
      let label = &step[..step.len()-2];
      let bucket_id = hash(label);
      let lens = boxes[bucket_id].iter_mut().find(|x|x.label == label);
      match lens {
        None => {
          boxes[bucket_id].push(Lens{label: label.to_vec(), strength});
        }
        Some(lens) => {
          lens.strength = strength;
        }
      }
    }
  }
  let mut answer = 0;
  for (box_id, b) in boxes.iter().enumerate() {
    for (lens_id, lens) in b.iter().enumerate() {
      answer += (box_id + 1) as u64 * (lens_id + 1) as u64 * lens.strength;
    }
  }
  println!("{}", answer);
}