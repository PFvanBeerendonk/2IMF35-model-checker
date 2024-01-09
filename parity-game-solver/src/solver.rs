use crate::types::progress_measure::ProgressMeasure;
use crate::types::vertex::Vertices;


use hashed_permutation::HashedIter;
use std::num::NonZeroU32;

pub fn main_algo(progress_measure: ProgressMeasure, vertices: &Vertices, d:i64, seed: Option<i64>) {
    // if seed is not set, we follow  order of `vertices`. Otherwise we will use a random seed based function

    let mut pm: ProgressMeasure = progress_measure;

    // let mut v: Vertex = vertices[0];
    let mut id: i64 = 0;
    let mut did_update: bool;
    let mut did_update_this_master_loop: bool = false;
    let mut result;
    let mut loop_end: bool = false;

    let mut iter: Option<HashedIter>;
    let length: u32 = vertices.len().try_into().unwrap();
    // if a seed is provided, we will make a hashed iterator
    if ! seed.is_none() {
        iter = Some(HashedIter::new_with_seed(
            NonZeroU32::new(length).unwrap(), 
            seed.unwrap().try_into().unwrap()
        ));

        id = iter.as_mut().unwrap().next().unwrap() as i64;
    } else {
        iter = None;
    }

    loop {
        result = pm.lift_v(id, &vertices, d);
        pm = result.0;
        did_update = result.1;

        // check if ϱ < Liftv (ϱ) for vertex v
        if did_update {
            did_update_this_master_loop = true;
        }
        if seed.is_none() {
            id += 1;
            if id == vertices.len() as i64 {
                id = 0;
                loop_end = true; // check final loop termination
            }
        } else if ! iter.is_none() {
            match iter.as_mut().unwrap().next() {
                Some(x) => id = x as i64,
                None => {
                    // hard reset the iterator
                    iter = Some(HashedIter::new_with_seed(
                        NonZeroU32::new(length).unwrap(), 
                        seed.unwrap().try_into().unwrap()
                    ));
            
                    id = iter.as_mut().unwrap().next().unwrap() as i64;
                    loop_end = true; // check final loop termination
                },
            }
        }

        // final master loop termination
        if loop_end {
            loop_end = false;

            // we did not update ANYTHING this id loop; so we have reached a stable point
            if ! did_update_this_master_loop {
                break;
            } else {
                did_update_this_master_loop = false;
            }
        }
    }


    // checking stuff
    println!("\n###   Finished Calculations   ###\n");

    let non_t_count = pm.data.iter().flatten().count();
    let size = pm.data.iter().count();
    println!("Size:         {}", size);
    println!("Nr of non-T:  {}", non_t_count);
    println!("Nr of T:      {}", size - non_t_count);



    println!("\n\n{: <10} | ϱ()", "ID");
    println!("-----------+------------------------------------");
    for (i, row) in pm.data.iter().enumerate()  {
        if row.is_none() {
            println!("{: <10} | T", i);
        } else {
            println!("{: <10} | {:?}", i, row.clone().unwrap());
        }
    }
}
