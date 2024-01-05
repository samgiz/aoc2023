use std::{io, collections::{HashMap, VecDeque}};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Pulse {
  High,
  Low
}

use Pulse::*;

#[derive(Debug, Clone)]
struct FlipFlop {
  labels: Vec<String>,
  state: bool // on / off
}

#[derive(Debug, Clone)]
struct Conjunction {
  labels: Vec<String>,
  state: HashMap<String, Pulse>,
  num_active: u64,
  num_inputs: u64
}

#[derive(Debug, Clone)]
struct Broadcast {
  labels: Vec<String>
}

impl Module {
  fn receive(&mut self, signal: Pulse, from: &str) -> Vec<(&String, Pulse)> {
    match self {
      Module::Broadcast(broadcast) => {
        broadcast.labels.iter().map(|label| {
          (label, signal)
        }).collect()
      }
      Module::Conjunction(conjunction) => {
        let last_state = conjunction.state.get(from);
        if signal == High && (last_state == None || last_state.unwrap() == &Low) {
          conjunction.num_active += 1;
        } else if signal == Low && (last_state != None && last_state.unwrap() == &High) {
          conjunction.num_active -= 1;
        }
        conjunction.state.insert(from.to_string(), signal);
        // dbg!(conjunction.num_inputs);
        if conjunction.num_active == conjunction.num_inputs as u64 {
          conjunction.labels.iter().map(|label|(label, Low)).collect()
        } else {
          conjunction.labels.iter().map(|label|(label, High)).collect()
        }
      }
      Module::FlipFlop(flip_flop) => {
        if signal == Low {
          flip_flop.state = !flip_flop.state;
          match flip_flop.state {
            true => {
              flip_flop.labels.iter().map(|label|(label, High)).collect()
            },
            false => {
              flip_flop.labels.iter().map(|label|(label, Low)).collect()
            }
          }
        } else {
          Vec::new()
        }
      }
    }
    
  }
}

#[derive(Debug, Clone)]
enum Module {
  Broadcast(Broadcast),
  FlipFlop(FlipFlop),
  Conjunction(Conjunction)
}

fn parse_module(line: &str, amount_sent_to: &mut HashMap<String, u64>) -> (String, Module) {
  let [left, right]: [&str; 2] = line.split(" -> ").collect::<Vec<_>>().try_into().unwrap();
  let right = right.split(", ").map(|x|x.to_string()).collect::<Vec<String>>();
  right.iter().for_each(|label|{
    let previous = amount_sent_to.get(label);
    match previous {
      None => amount_sent_to.insert(label.clone(), 1),
      Some(amount) => amount_sent_to.insert(label.clone(), amount + 1)
    };
  });
  match left.as_bytes()[0] {
    b'b' => {
     (left.to_string(), Module::Broadcast(Broadcast {labels: right}))
    }
    b'%' => {
      let left = left[1..].to_string();
      (left, Module::FlipFlop(FlipFlop{labels: right, state: false}))
    }
    b'&' => {
      let left = left[1..].to_string();
      (left, Module::Conjunction(Conjunction{labels: right, state: HashMap::new(), num_active: 0, num_inputs: 0}))
    },
    _ => panic!("Bad formatting passed to parse_module {line}")
  }
}

fn main() {
  let mut modules = HashMap::new();
  let mut amount_sent_to = HashMap::new();
  io::stdin().lines().for_each(|line| {
    let (name, module) = parse_module(&line.unwrap(), &mut amount_sent_to);
    modules.insert(name, module);
  });
  amount_sent_to.iter().for_each(|(label, amount)| {
    let module_to_process = modules.get_mut(label);
    match module_to_process {
      Some(module) => {
        match module {
            Module::Conjunction(conjunction) => conjunction.num_inputs = *amount,
            _ => ()
        }
      }
      None => ()
    }
  });
  let mut num_lows: u64 = 0;
  let mut num_highs: u64 = 0;
  for _ in 0..1000 {
    // Perform a button push
    let mut to_visit: VecDeque<(String, String, Pulse)> = VecDeque::new();
    to_visit.push_back(("".to_string(), "broadcaster".to_string(), Low));
    while !to_visit.is_empty() {
      let (from_label, to_label, pulse) = to_visit.pop_front().unwrap();
      match pulse {
        Low => num_lows += 1,
        High => num_highs += 1
      }
      // let module = &modules.get_mut(to_label).unwrap();
      // dbg!(to_label.clone());
      let module_to_process = modules.get_mut(&to_label);
      match module_to_process {
        Some(module) => {
          // dbg!(module.clone());
          // dbg!(pulse);
          let new_to_visit = module.receive(pulse, &from_label);
          new_to_visit.iter().for_each(|(label, signal)| {
            to_visit.push_back((to_label.clone(), label.to_string(), *signal));
          });
          // dbg!(module.clone());
        }
        None => ()
      }
    }
  }
  println!("{} {} {}",num_lows, num_highs,  num_lows * num_highs);
}