//use std::time::Instant;
//use rand::{distributions::Uniform, Rng};
//use rand::distributions::{Distribution, Uniform};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use ndarray::prelude::*;

//use rand::{Rng
mod graph;
mod path_planner;
mod environment;
mod visualize;

fn add_field(a: &Array<f64, Ix1>, b: &Array<f64, Ix1>) -> Vec<f64> {
    let added = (a+b).to_vec();
    added

}

fn main() {

    // PARAMS
    let seed: u64 = 1964;
    let np = 52;
    let area: f64 = 25.0;
    let res: usize = 100;
    let ng = res.pow(2);
    let dim: f64 = area.sqrt();
    let step: f64 = dim/((res as f64)-1.);

    let g = graph::Graph::new(ng as i64, 0, ng as i64 - 1, 4);
    let mut r = StdRng::seed_from_u64(seed);
    let env = environment::Environment::new(&mut r, np, area, g.start, g.goal, res);


    let true_field = Array::from_vec(env.get_true_field());
    let distances = Array::from_vec(env.get_distances());
    let true_weights = add_field(&distances, &true_field);//(Array::from_vec(true_field) + distances).to_vec();

    //Compute the True path and costs
    let (path, path_cost) = path_planner::dijkstra(&g, &true_weights, step);
    let true_opt_path = env.workspace.select(Axis(0), &path[..]);
    visualize::field_map(&true_weights, area, res, true, &true_opt_path);


    //Start CSCP init
    let mut f: Array<f64, Ix1> = Array::zeros(ng);
    let mut i: Array<f64, Ix1> = Array::zeros(ng);
    println!("{:?}", f);
    let mut p: Array<f64, Ix2> = Array::eye(ng) * 10000.0;

    //Find initial path
    let (est_path, est_path_cost) = path_planner::dijkstra(&g, &distances.to_vec(), step);
    let est_opt_path = env.workspace.select(Axis(0), &est_path[..]);
    let est_weights = add_field(&distances, &f);
    visualize::field_map(&est_weights, area, res, true, &est_opt_path);
}
