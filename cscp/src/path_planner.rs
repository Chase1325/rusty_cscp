use crate::graph::Graph;
use std::collections::HashMap;

//Return the minimium node index
//pub fn get_minimum(&q: &Vec<(&i64, &f64)>) -> i64 {



//Returns the optimal path and path cost
pub fn dijkstra(g: &Graph, weights: &Vec<f64>, step:f64) -> (Vec<i64>, f64) {

    fn get_minimum(q: &[(i64, f64)]) -> i64 {
        let vec_iter = q.iter();
        let mut min_indx = 0;
        let mut min_val = f64::INFINITY;
        let mut indx = 0;

        for node in vec_iter {
            if node.1 < min_val {
                min_indx = indx;
                min_val = node.1;
            }
            indx += 1;
        }
        min_indx
    }

    let mut visited: HashMap<i64, bool> = HashMap::new();//vec![false; g.edges.len()];
    let mut costs = HashMap::new();
    let mut previous: HashMap<i64, i64> = HashMap::new();
    let mut queue = Vec::new();
    let mut qed = false;
    let mut path = vec![g.goal];
    let mut opt_cost = f64::INFINITY;

    costs.insert(g.start, 0.);
    queue.push((g.start, 0.));

    while qed == false {
        //println!("{:?}", &queue);
        //println!("{:?}", &costs);
        let min_vertex: i64 = get_minimum(queue.as_slice());
        //println!("{:?}", &min_node);
        let popped = queue.remove(min_vertex as usize);
        //visited[&min_node] = true; 
        let min_node = popped.0;
        visited.insert(min_node, true);

        if min_node == g.goal {
            opt_cost = popped.1;//costs[&min_node];
            let mut waypoint = previous[&g.goal];

            while waypoint != g.start { 
                path.push(waypoint);
                waypoint = previous[&waypoint];
            }
            path.push(g.start);

            qed = true;

        } else {
            let adjacent = (&g.edges[popped.0 as usize]).iter();
            for edge in adjacent { 
                let check_visit: bool = match visited.get(edge) {
                    Some(v) => *v,
                    None => false, 
                };
                if !check_visit {
                    let mut weight = costs[&min_node];
                    weight += step * weights[*edge as usize] + 1e-16;
                    let edgecost: f64 = match costs.get(edge) {
                        Some(e) => *e,
                        None => f64::INFINITY,
                    };
                    if weight < edgecost {
                        costs.insert(*edge, weight);
                        previous.insert(*edge, min_node);
                        queue.push((*edge, weight));
                    }
                }
            }

        }
    }
    //Safety if the while loop vails
    let path_len = path.len() as f64;
    (path, opt_cost-((path_len-1.)*1e-16))
}
