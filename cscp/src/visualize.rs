extern crate itertools;
extern crate itertools_num;

use plotly::common::{ColorScale, ColorScalePalette, Mode};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Scatter, Plot};
use std::f64::consts::PI;
use itertools_num::linspace;
use ndarray::prelude::*;

pub fn basic_heat_map(show: bool) {
    let z = vec![vec![f64::NAN, f64::NAN, 30.], vec![20., 64., 60.], vec![30., 60., 1.]];
    let x = vec![10., 20., 30.];
    let y = vec![0.1, 0.2, 0.9];
    let trace = HeatMap::new(x, y, z).connect_gaps(false).zsmooth("best");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    //println!("{}", plot.to_inline_html(Some("basic_heat_map")));
}

pub fn field_map(field: Vec<f64>, area: f64, res: usize, show_path: bool, path: Array2<f64>) {

    let dim = area.sqrt();
    let x = Array::linspace(0.0, dim, res).to_vec();
    let y = Array::linspace(0.0, dim, res).to_vec();
    let z: Vec<Vec<f64>> = field.chunks(res).map(|x| x.into()).collect();
    //z = (0..z[0].len()).map(|i| z.iter().map(|inner| inner[i].clone()).collect::<Vec<f64>>()).collect();
    //println!("{:?}", z);
    //let trace = HeatMap::new(x, y, z).connect_gaps(false).zsmooth("best").transpose(true).color_scale(ColorScale::Palette(ColorScalePalette::Portland));
    let trace = Contour::new(x, y, z).connect_gaps(false).transpose(true).color_scale(ColorScale::Palette(ColorScalePalette::Portland));
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show_path {
        let x_col = path.slice(s![.., 0]).to_vec();
        let y_col = path.slice(s![.., 1]).to_vec();
        println!("{:?}", x_col);

        let path = Scatter::new(x_col, y_col).mode(Mode::LinesMarkers);
        plot.add_trace(path);
      
    }
    plot.show();
} 