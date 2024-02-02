use std::{io, collections::HashMap};

fn find_path(residual_graph: &Vec<Vec<(usize, usize)>>, current: usize, sink: usize, visited: &mut Vec<bool>, path: &mut Vec<usize>) -> Option<Vec<usize>> {
  visited[current] = true;
  if current == sink {
    return Some(path.clone());
  }

  for &(neighbor, capacity) in residual_graph[current].iter() {
    if !visited[neighbor] && capacity > 0 {
      path.push(neighbor);
      match find_path(residual_graph, neighbor, sink, visited, path) {
        None => (),
        anything_else => return anything_else
      }
      path.pop();
    }
  }

  None
}

fn find_min_cut(residual_graph: &Vec<Vec<(usize, usize)>>, source: usize) -> u64 {
  let mut visited = vec![false; residual_graph.len()];
  mark_reachable_nodes(residual_graph, source, &mut visited)
}

fn mark_reachable_nodes(residual_graph: &Vec<Vec<(usize, usize)>>, current: usize, visited: &mut Vec<bool>) -> u64 {
  visited[current] = true;
  let mut answer = 1;
  for &(neighbor, capacity) in residual_graph[current].iter() {
    if !visited[neighbor] && capacity > 0 {
      answer += mark_reachable_nodes(residual_graph, neighbor, visited);
    }
  }
  answer
}

fn update_residual_graph(residual_graph: &mut [Vec<(usize, usize)>], path: &[usize]) {
  for window in path.windows(2) {
    let (from, to) = (window[0], window[1]);
    for i in 0..residual_graph[from].len() {
      if residual_graph[from][i].0 == to {
        residual_graph[from][i].1 -= 1;
      }
    }
    for i in 0..residual_graph[to].len() {
      if residual_graph[to][i].0 == from {
        residual_graph[to][i].1 += 1;
      }
    }
  }
}

fn find_cut(source: usize, sink: usize, edges: &Vec<Vec<usize>>) -> Option<u64> {
  // Initialize residual graph
  let mut residual_graph = edges.clone().iter().map(|x|x.iter().map(|&x|(x, 1)).collect()).collect();

  // Initialize flow and augmenting path
  let mut max_flow = 0;
  let mut visited = vec![false; edges.len()];
  let mut p = Vec::new();
  let mut path = find_path(&residual_graph, source, sink, &mut visited, &mut p);

  while let Some(p) = path {
    if max_flow >= 3 {
      return None
    }

    // Update residual capacities and reverse edges
    update_residual_graph(&mut residual_graph, &p);

    // Add path flow to overall flow
    max_flow += 1;

    // Try to find another augmenting path
    let mut visited = vec![false; edges.len()];
    let mut p = Vec::new();
    path = find_path(&residual_graph, source, sink, &mut visited, &mut p);
  }
  // Once no augmenting path is found, find and return the minimum cut
  let min_cut_edges = find_min_cut(&residual_graph, source);

  Some(min_cut_edges * (edges.len() as u64 - min_cut_edges))
}

fn main() {
  let mut name_to_index: HashMap<String, usize> = HashMap::new();
  let mut edges: Vec<Vec<usize>> = Vec::new();
  io::stdin().lines().for_each(|line| {
    let line = line.unwrap();

    let [lhs, rhs]: [&str; 2] = line.split(": ").collect::<Vec<_>>().try_into().unwrap();
    let rhs = rhs.split(' ').collect::<Vec<_>>();
    if !name_to_index.contains_key(lhs) {
      name_to_index.insert(lhs.to_string(), edges.len());
      edges.push(Vec::new());
    }
    for rhs in rhs {
      if !name_to_index.contains_key(rhs) {
        name_to_index.insert(rhs.to_string(), edges.len());
        edges.push(Vec::new());
      }
      edges[name_to_index[lhs]].push(name_to_index[rhs]);
      edges[name_to_index[rhs]].push(name_to_index[lhs]);
    }
  });
  let source = 0;
  for sink in 1..edges.len() {
    let answer = find_cut(source, sink, &edges);
    if let Some(answer) = answer {
      println!("{answer}");
      break;
    }
  }
}
