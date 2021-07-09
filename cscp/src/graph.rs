#[derive(Debug)]
pub struct Graph {
    pub start: i64,
    pub goal: i64,
    pub edges: Vec<Vec<i64>>
}

impl Graph { 
    fn connect_four(indx: i64, &ng: &i64) -> Vec<i64> {
        let mut edges = Vec::new();

        let dim = (ng as f64).sqrt() as i64;

        let left: i64 = indx - dim;
        let right: i64 = indx + dim;
        let up: i64 = indx + 1;
        let down: i64 = indx - 1;
        
        //Check Left:
        if left >= 0 {
            edges.push(left);
        }
        if right <= (ng-1){
            edges.push(right);
        }
        if (up % dim) > (indx % dim){
            edges.push(up);
        }
        if (indx % dim) > 0 {
            edges.push(down);
        }
        
        return edges;
    }

    pub fn new(ng: i64, start: i64, goal: i64, connection: u8) -> Graph {
        let mut edges = Vec::new();

        //let dim = (ng as f64).sqrt() as i64;
        
        for i in 0..ng {
            match connection {
                4 => edges.push(Graph::connect_four(i, &ng)),
                8 => edges.push(vec![i]),
                _ => println!("This is not supported")
            }
        }
      

        Graph {start: start, goal: goal, edges: edges}
    }
}

