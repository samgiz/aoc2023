use std::{io, collections::HashMap, thread::current};

struct Workflow {
  rules: Vec<Rule>
}

impl Workflow {
  fn get_next_label(&self, part: &Part) -> Vec<u8> {
    for rule in &self.rules {
      if part.satisfies(&rule) {
        return rule.next_label.clone();
      }
    }
    panic!("Could not find satisfying label");
  }
}

struct Part {
  x: i64,
  m: i64,
  a: i64,
  s: i64
}

impl Part {
  fn satisfies(&self, rule: &Rule) -> bool {
    match &rule.condition {
      None => true,
      Some(condition) => {
        let value = match condition.category {
          Category::m => self.m,
          Category::a => self.a,
          Category::x => self.x,
          Category::s => self.s
        };
        match condition.operator {
          LessThan => value < condition.amount,
          GreaterThan => value > condition.amount
        }
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
  let parts = lines.map(|line| {
    let line = line.unwrap();
    let line = &line[1..line.len()-1];
    let categories: Vec<_> = line.split(',').map(|x| x[2..].parse::<i64>().unwrap()).collect();
    Part {
      x: categories[0],
      m: categories[1],
      a: categories[2],
      s: categories[3]
    }
  });
  let scores = parts.map(|part| {
    let mut current_label = vec!(b'i', b'n');
    while current_label != b"A" && current_label != b"R" {
      current_label = workflows[&current_label].get_next_label(&part);
      // dbg!(current_label.iter().map(|&x| x as char).collect::<String>());
    }
    if current_label == b"A" {
      part.x + part.m + part.a + part.s
    } else {
      0
    }
  });
  let answer: i64 = scores.sum();
  println!("{}", answer);
}