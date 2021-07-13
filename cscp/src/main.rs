use std::time::Instant;
//use rand::{distributions::Uniform, Rng};
use rand::distributions::{Distribution, Uniform};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use ndarray::prelude::*;

//use rand::{Rng
mod graph;
mod path_planner;
mod environment;
mod visualize;

fn main() {

    // PARAMS
    let seed: u64 = 52;
    let np = 10;
    let area: f64 = 100.0;
    let res: usize = 100;
    let ng = res.pow(2);
    let dim: f64 = area.sqrt();
    let step: f64 = dim/((res as f64)-1.);

    let g = graph::Graph::new(ng as i64, 0, ng as i64 - 1, 4);
    let mut r = StdRng::seed_from_u64(seed);
    let env = environment::Environment::new(&mut r, np, area, g.start, g.goal, res);


    let true_field = env.get_true_field();
    let distances = Array::from_vec(env.get_distances());
    let true_weights = (Array::from_vec(true_field) + distances).to_vec();

    //Compute the True path and costs
    let (path, path_cost) = path_planner::dijkstra(&g, &true_weights, step);
    let true_opt_path = env.workspace.select(Axis(0), &path[..]);
    visualize::field_map(true_weights, area, res, true, true_opt_path);
    
}
