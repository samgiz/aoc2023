use std::{io, collections::HashMap};

struct Workflow {
  rules: Vec<Rule>
}

impl Workflow {
  fn get_next_label(&self, mut part: Part, parts: &mut Vec<(Part, Vec<u8>)>) {
    for rule in &self.rules {
      let (left, right) = part.satisfies(&rule);
      match right {
        Some(right) => parts.push((right, rule.next_label.clone())),
        None => ()
      }
      match left {
        None => return,
        Some(left) => part = left
      }
    }
    panic!("Could not find satisfying label");
  }
}

#[derive(Copy, Clone)]
struct Part {
  x: (i64, i64),
  m: (i64, i64),
  a: (i64, i64),
  s: (i64, i64)
}

impl Part {
  fn satisfies(&self, rule: &Rule) -> (Option<Part>, Option<Part>) {
    match &rule.condition {
      None => (None, Some(*self)),
      Some(condition) => {
        let (low, high) = match condition.category {
          Category::m => self.m,
          Category::a => self.a,
          Category::x => self.x,
          Category::s => self.s
        };
        let part1_values = match condition.operator {
          LessThan => {
            // get everything greater or equal to amount
            let low = condition.amount;
            if low < high {
              Some((low, high))
            } else {
              None
            }
          }
          GreaterThan => {
            // get everything less or equal to amount
            let high = condition.amount+1;
            if low < high {
              Some((low, high))
            } else {
              None
            }
          }
        };
        let part2_values = match condition.operator {
          LessThan => {
            // get everything less than amount
            let high = condition.amount;
            if low < high {
              Some((low, high))
            } else {
              None
            }
          }
          GreaterThan => {
            // get everything greater than amount
            let low = condition.amount+1;
            if low < high {
              Some((low, high))
            } else {
              None
            }
          }
        };
        let part1 = match part1_values {
          None => None,
          Some(values) => {
            Some(match condition.category {
              Category::m => Part {
                x: self.x,
                m: values,
                a: self.a,
                s: self.s
              },
              Category::a => Part {
                x: self.x,
                m: self.m,
                a: values,
                s: self.s
              },
              Category::x => Part {
                x: values,
                m: self.m,
                a: self.a,
                s: self.s
              },
              Category::s => Part {
                x: self.x,
                m: self.m,
                a: self.a,
                s: values
              },
            })
          }
        };
        let part2 = match part2_values {
          None => None,
          Some(values) => {
            Some(match condition.category {
              Category::m => Part {
                x: self.x,
                m: values,
                a: self.a,
                s: self.s
              },
              Category::a => Part {
                x: self.x,
                m: self.m,
                a: values,
                s: self.s
              },
              Category::x => Part {
                x: values,
                m: self.m,
                a: self.a,
                s: self.s
              },
              Category::s => Part {
                x: self.x,
                m: self.m,
                a: self.a,
                s: values
              },
            })
          }
        };
        (part1, part2)
      }
    }
  }
}

enum Category {
  x,
  m,
  a,
  s
}

impl From<u8> for ComparisonOperator {
  fn from(value: u8) -> Self {
    match value {
      b'<' => LessThan,
      b'>' => GreaterThan,
      _ => panic!("Invalid value passed to ComparisonOperator::from: {value}")
    } 
  }
}

enum ComparisonOperator {
  LessThan,
  GreaterThan
}
impl From<u8> for Category {
  fn from(value: u8) -> Self {
    match value {
      b'x' => Category::x,
      b'm' => Category::m,
      b'a' => Category::a,
      b's' => Category::s,
      _ => panic!("Invalid value passed to Category::from: {value}")
    } 
  }
}

use ComparisonOperator::*;

struct Condition {
  amount: i64,
  operator: ComparisonOperator,
  category: Category
}

struct Rule {
  next_label: Vec<u8>,
  condition: Option<Condition>
}

fn main() {
  let mut lines = io::stdin().lines();
  let mut workflows = HashMap::new();
  loop {
    let line = lines.next().unwrap().unwrap().as_bytes().to_vec();
    if line.is_empty() {
      break;
    }
    let [label, rest]: [&[u8]; 2] = line.split(|&x| x == b'{').collect::<Vec<&[u8]>>().try_into().unwrap();
    let workflow = &rest[..rest.len()-1];
    let rules = workflow.split(|&x|x == b',');
    // dbg!(rules.clone().map(|x|x.iter().map(|&x| x as char).collect::<String>()).collect::<Vec<_>>());
    let rules = rules.map(|rule| {
      if rule.split(|&x| x == b':').count() == 1 {
        return Rule {
          next_label: rule.to_vec(),
          condition: None
        }
      }
      let [condition_string, label]: [&[u8]; 2] = rule.split(|&x| x == b':').collect::<Vec<&[u8]>>().try_into().unwrap();
      let category = Category::from(condition_string[0]);
      let comparison_operator = ComparisonOperator::from(condition_string[1]);
      let amount = condition_string[2..].iter().map(|&x|x as char).collect::<String>().parse::<i64>().unwrap();
      Rule {
        next_label: label.to_vec(),
        condition: Some(Condition {
          amount,
          category,
          operator: comparison_operator
        })
      }
    }).collect();
    workflows.insert(label.to_vec(), Workflow {rules});
  }
  let mut parts = vec!((Part {
    m: (1, 4001),
    x: (1, 4001),
    a: (1, 4001),
    s: (1, 4001)
  }, vec!(b'i', b'n')));
  let mut answer: i64 = 0;
  while !parts.is_empty() {
    let (part, label) = parts.pop().unwrap();
    if label == b"R" {
      continue;
    } else if label == b"A" {
      answer += (part.x.1 - part.x.0) * (part.m.1 - part.m.0) * (part.a.1 - part.a.0) * (part.s.1 - part.s.0);
      continue;
    }
    workflows[&label].get_next_label(part, &mut parts);
  }
  println!("{}", answer);
}