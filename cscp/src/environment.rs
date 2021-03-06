extern crate ndarray;
use ndarray::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

#[derive(Debug)]
pub struct Threat {
    a: f64,
    x: f64,
    y: f64,
    s_x: f64,
    s_y: f64
}

impl Threat { 
    pub fn get_threat(&self, x_pos: f64, y_pos: f64) -> f64 {
        self.a * (-0.5 * (((x_pos - self.x)/self.s_x).powf(2.) + ((y_pos - self.y)/self.s_y).powf(2.))).exp()
    }
}

#[derive(Debug)]
pub struct Environment {
    threats: Vec<Threat>, 
    start: i64,
    goal: i64,
    res: usize,
    area: f64,
    pub workspace: Array2<f64>
}

impl Environment {
    pub fn new(seed: &mut StdRng, np: usize, area: f64, start: i64, goal: i64, res: usize) -> Environment {

        let dim = area.sqrt();
        let step: f64 = dim/((res as f64)-1.);

        //Generate the random threats given the number of parameters, seed, and area
        let mut threats = Vec::<Threat>::with_capacity(np);
        let amp_range = Uniform::from(1.0..10.);
        let pos_range = Uniform::from(-dim/10.0..(dim+dim/10.0));
        let std_range = Uniform::from((dim/50.)..(dim/10.));
        for _ in 0..np {
            threats.push(Threat{a: amp_range.sample(seed), x: pos_range.sample(seed), y: pos_range.sample(seed), s_x: std_range.sample(seed), s_y: std_range.sample(seed)});
        }

        //Generate the workspace;
        let mut x_row = Vec::<f64>::new();
        let mut y_row = Vec::<f64>::new();

        for j in 0..res {
            for i in 0..res {
                let x = i as f64 * step;
                let y = j as f64 * step;
                x_row.push(y);
                y_row.push(x);
            }
        }
        //Create the workspace
        let mut wsp = Array2::<f64>::zeros((x_row.len(), 0));
        let x_view = ArrayView::from(&x_row).into_shape((x_row.len(), 1)).unwrap();
        let y_view = ArrayView::from(&y_row).into_shape((y_row.len(), 1)).unwrap();
        wsp.append(Axis(1), x_view).unwrap();
        wsp.append(Axis(1), y_view).unwrap();

        //println!("{:?}", wsp);
        //Environment::get_threat_value(1., 2.);
        Environment {threats: threats, start: start, goal: goal, res: res, area: area, workspace: wsp}
        
    }
    pub fn get_threat_value(&self, x_pos: f64, y_pos: f64) -> f64 {
        let mut sum: f64 = 0.;
        for i in 0..self.threats.len() {
            sum += self.threats[i].get_threat(x_pos, y_pos);
        }
        sum
    }

    pub fn get_true_field(&self) -> Vec<f64> {

        let iterator = self.workspace.axis_iter(Axis(0));
        //println!("{:?}", iterator.next());
        let true_field: Vec<f64> = iterator.map(|row| self.get_threat_value(row[0], row[1])).collect();
        true_field
    }

    pub fn get_distances(&self) -> Vec<f64> { 
        let iterator = self.workspace.axis_iter(Axis(0));
        let goal_pose = self.workspace.slice(s![self.goal as usize, ..]);

        println!("{:?}", goal_pose);
        let distances: Vec<f64> = iterator.map(|row| ((row[0]-goal_pose[0]).powf(2.) + (row[1]-goal_pose[1]).powf(2.)).sqrt()).collect();
        distances
    }

}