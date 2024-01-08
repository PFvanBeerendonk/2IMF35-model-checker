use crate::types::progress_measure::ProgressMeasure;
use crate::types::vertex::Vertex;


pub fn main_algo(progress_measure: ProgressMeasure, vertices: Vec<Vertex>, random_lifting: bool) {
    // if args.random_lifting is not set, we follow  order of `vertices`. Otherwise we will use a random seed based function

    let mut pm: ProgressMeasure = progress_measure;

    // let mut v: Vertex = vertices[0];
    let mut last_updated_id: i64 = -1;
    let mut id: i64 = 0;
    let mut did_update: bool;
    while id != last_updated_id {
        // let mut result = pm.lift_v(vertices[id as usize], vertices);
        // pm = result.0;
        // did_update = result.1;
        // // check if ϱ < Liftv (ϱ) for vertex v
        // if did_update {
        //     last_updated_id = id;
        // }
        // id += 1;
    }
}

