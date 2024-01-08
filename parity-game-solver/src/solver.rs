use crate::types::progress_measure::ProgressMeasure;
use crate::types::vertex::Vertices;


pub fn main_algo(progress_measure: ProgressMeasure, vertices: &Vertices, d:i64, random_lifting: bool) {
    // if args.random_lifting is not set, we follow  order of `vertices`. Otherwise we will use a random seed based function

    let mut pm: ProgressMeasure = progress_measure;

    // let mut v: Vertex = vertices[0];
    let mut id: i64 = 0;
    let mut did_update: bool;
    let mut did_update_this_master_loop: bool = false;
    let mut result;

    loop {
        result = pm.lift_v(id, &vertices, d);
        pm = result.0;
        did_update = result.1;

        // check if ϱ < Liftv (ϱ) for vertex v
        if did_update {
            did_update_this_master_loop = true;
        }
        id += 1;

        // final master loop termination
        if id == vertices.len() as i64 {
            id = 0; // return to start of the loop

            // we did not update ANYTHING this id loop; so we have reached a stable point
            if ! did_update_this_master_loop {
                break;
            } else {
                did_update_this_master_loop = false;
            }
        }
    }
}

