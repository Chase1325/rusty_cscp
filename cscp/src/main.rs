use std::time::Instant;
//use rand::{distributions::Uniform, Rng};
use rand::distributions::{Distribution, Uniform};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

//use rand::{Rng
mod graph;
mod path_planner;
mod environment;

fn main() {

    // PARAMS
    let seed: u64 = 1;
    let np = 5;
    let area = 1.;


    let start = Instant::now();
    let g = graph::Graph::new(9, 0, 8, 4);
    let duration = start.elapsed();
    println!("{:?}", duration);

    let mut r = StdRng::seed_from_u64(seed);
    let range = Uniform::from(0.1..1.0);
    let weights: Vec<f64> = range.sample_iter(&mut r).take(9).collect();


    let start = Instant::now();
    for _ in 0..100 {
        let n: u16 = r.gen();
        println!("{}", n);
        let (path, path_cost) = path_planner::dijkstra(&g, &weights, 0.1);
    }
    println!("{}", "round 2");
    for _ in 0..100 {
        let n: u16 = r.gen();
        println!("{}", n);
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
    //println!("{:?}, {}", path, path_cost);

    //Finish establishing this
    let env = environment::Environment::new(&mut r, np, area);
    println!("{:?}", env);
}
