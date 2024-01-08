use std::io;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
extern crate openblas_src;
// We know p_i, v_i for each hail stone
// We're trying to find p, v for our stone
// t_i >= 0: p_i + v_i * t_i = p + v * t_i
// (p - p_i) = t_i * (v_i - v)
// => (p - p_i) x (v_i - v) = 0 as the vectors point in the same direction
// 0 = (p - p_i)[1] * (v_i - v)[2] - (p - p_i)[2] * (v_i - v)[1]
// 0 = (p - p_i)[2] * (v_i - v)[0] - (p - p_i)[0] * (v_i - v)[2]
// 0 = (p - p_i)[0] * (v_i - v)[1] - (p - p_i)[1] * (v_i - v)[0]

// 0 = p[1] * v_i[2] - p[1] * v[2] - p_i[1] * v_i[2] + p_i[1] * v[2] - 
//   - p[2] * v_i[1] + p[2] * v[1] + p_i[2] * v_i[1] - p_i[2] * v[1]

// p x v = p x v_i - p_i x v_i + p_i x v
// RHS[0] = p[1] * v_i[2] - p_i[1] * v_i[2] + p_i[1] * v[2] - 
//        - p[2] * v_i[1] + p_i[2] * v_i[1] - p_i[2] * v[1]
// RHS[1] = p[2] * v_i[0] - p_i[2] * v_i[0] + p_i[2] * v[0] - 
//        - p[0] * v_i[2] + p_i[0] * v_i[2] - p_i[0] * v[2]
// RHS[2] = p[0] * v_i[1] - p_i[0] * v_i[1] + p_i[0] * v[1] - 
//        - p[1] * v_i[0] + p_i[1] * v_i[0] - p_i[1] * v[0]

fn main() {
  let mut p_i = Vec::new();
  let mut v_i = Vec::new();
  io::stdin().lines().for_each(|line| {
    let line = line.unwrap();
    let [p, v]: [&str; 2] = line.split(" @ ").collect::<Vec<_>>().try_into().unwrap();
    let [px, py, pz]: [f64; 3] = p.split(", ").map(|x|x.trim().parse::<f64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let [vx, vy, vz]: [f64; 3] = v.split(", ").map(|x|x.trim().parse::<f64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();

    p_i.push([px, py, pz]);
    v_i.push([vx, vy, vz]);
  });
  // x = p[0], p[1], p[2], v[0], v[1], v[2]
  
  let A: Array2<f64> = array![
// RHS[0] = p[1] * v_i[2] - p_i[1] * v_i[2] + p_i[1] * v[2] - 
//        - p[2] * v_i[1] + p_i[2] * v_i[1] - p_i[2] * v[1]
//        = p[1] * v_j[2] - p_j[1] * v_j[2] + p_j[1] * v[2] - 
//        - p[2] * v_j[1] + p_j[2] * v_j[1] - p_j[2] * v[1]
// p[1] * (v_i[2] - v_j[2]) + p[2] * (v_j[1] - v_i[1]) + v[1] * (p_j[2] - p_i[2]) + v[2] * (p_i[1] - p_j[1])
// = - p_i[2] * v_i[1] + p_i[1] * v_i[2] - p_j[1] * v_j[2] + p_j[2] * v_j[1]
    [0.0, v_i[0][2] - v_i[1][2], v_i[1][1] - v_i[0][1], 0.0, p_i[1][2] - p_i[0][2], p_i[0][1] - p_i[1][1]],
    [v_i[1][2] - v_i[0][2], 0.0, v_i[0][0] - v_i[1][0], p_i[0][2] - p_i[1][2], 0.0, p_i[1][0] - p_i[0][0]],
    [v_i[0][1] - v_i[1][1], v_i[1][0] - v_i[0][0], 0.0, p_i[1][1] - p_i[0][1], p_i[0][0] - p_i[1][0], 0.0],
    [0.0, v_i[0][2] - v_i[2][2], v_i[2][1] - v_i[0][1], 0.0, p_i[2][2] - p_i[0][2], p_i[0][1] - p_i[2][1]],
    [v_i[2][2] - v_i[0][2], 0.0, v_i[0][0] - v_i[2][0], p_i[0][2] - p_i[2][2], 0.0, p_i[2][0] - p_i[0][0]],
    [v_i[0][1] - v_i[2][1], v_i[2][0] - v_i[0][0], 0.0, p_i[2][1] - p_i[0][1], p_i[0][0] - p_i[2][0], 0.0],
  ];
  let b: Array1<f64> = array![
    -(p_i[0][2] * v_i[0][1] - p_i[0][1] * v_i[0][2] + p_i[1][1] * v_i[1][2] - p_i[1][2] * v_i[1][1]), 
    -(p_i[0][0] * v_i[0][2] - p_i[0][2] * v_i[0][0] + p_i[1][2] * v_i[1][0] - p_i[1][0] * v_i[1][2]),
    -(p_i[0][1] * v_i[0][0] - p_i[0][0] * v_i[0][1] + p_i[1][0] * v_i[1][1] - p_i[1][1] * v_i[1][0]),
    -(p_i[0][2] * v_i[0][1] - p_i[0][1] * v_i[0][2] + p_i[2][1] * v_i[2][2] - p_i[2][2] * v_i[2][1]), 
    -(p_i[0][0] * v_i[0][2] - p_i[0][2] * v_i[0][0] + p_i[2][2] * v_i[2][0] - p_i[2][0] * v_i[2][2]),
    -(p_i[0][1] * v_i[0][0] - p_i[0][0] * v_i[0][1] + p_i[2][0] * v_i[2][1] - p_i[2][1] * v_i[2][0]),
  ];
  dbg!(A.clone());
  dbg!(b.clone());
  
  let x = A.solve_into(b.clone()).unwrap();
  println!("{}", x[0] + x[1] + x[2]);
  // This solution gives me an answer of 540355811503156.25
  // The actual answer is 540355811503157 
  // I have no idea how this should be done to give an accurate answer
  // I tried using mpmath and got back the same result, which is even more confusing, I thought that's supposed to be accurate.
  // I tried using someone else's (supposedly correct) rust solution and got 540355811503159???
  // Someone's solution with sympy then gave me the correct answer.
  // This was a diabolical problem and I hate my life.
  // And because it's so easy to make mistakes, I was never sure if I'm doing the right thing or not.
}
