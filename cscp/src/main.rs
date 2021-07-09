use std::time::Instant;
use rand::{distributions::Uniform, Rng};
mod graph;
mod path_planner;

fn main() {
    let start = Instant::now();
    let g = graph::Graph::new(2500, 0, 2499, 4);
    let duration = start.elapsed();
    println!("{:?}", duration);
    //println!("{:?}", g);
    let adj = &g.edges[3];
    println!("{:?}", adj);

    let range = Uniform::from(0.1..1.0);
    let weights: Vec<f64> = rand::thread_rng().sample_iter(&range).take(2500).collect();
    let start = Instant::now();
    for _ in 0..100 {
        let (path, path_cost) = path_planner::dijkstra(&g, &weights, 0.1);
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
    //println!("{:?}, {}", path, path_cost);
}
