use crate::types::progress_measure::ProgressMeasure;
use crate::types::vertex::Vertices;
use crate::types::lifting_strategies::{least_successor_order, FocusListLiftingStrategy, PredecessorLiftingStrategy};

use permutation_iterator::Permutor;
use std::collections::HashMap;

pub fn main_algo(progress_measure: ProgressMeasure, vertices: &Vertices, d: i64, lifting_strategy: i64, seed: Option<i64>) {
    // if seed is not set, we follow  order of `vertices`. Otherwise we will use a random seed based function

    let mut pm: ProgressMeasure = progress_measure.clone();

    // let mut v: Vertex = vertices[0];
    let mut id: i64 = 0;
    let mut did_update: bool;
    let mut did_update_this_master_loop: bool = false;
    let mut result;
    let mut loop_end: bool = false;

    let mut iter: Option<Permutor>;
    let length = vertices.len() as u64;

    // the vertex identifiers sorted based on the least successor lifting strategy
    let mut least_successor_lifting_strat = Default::default();
    let mut predecessor_lifting_strat = Default::default();
    // if a seed is provided, we will make a hashed iterator
    if lifting_strategy == 1 {
        iter = Some(Permutor::new_with_u64_key(length, seed.unwrap() as u64));

        id = iter.as_mut().unwrap().next().unwrap() as i64;
    } else if lifting_strategy == 2 {
        least_successor_lifting_strat = least_successor_order(&vertices).into_iter();
        id = least_successor_lifting_strat.next().unwrap() as i64;
        iter = None;
    } else if lifting_strategy == 3 {
        let mut strategy = FocusListLiftingStrategy::new();

        let mut progress_measure_cp = progress_measure.clone();
        strategy.run(
            &mut progress_measure_cp,
            &vertices,
            d,
            length.try_into().unwrap(),
            length.try_into().unwrap(),
        );
        println!("{:?}", strategy);
        iter = None;
    } else if lifting_strategy == 4 {
        let top: HashMap<i64, bool> = HashMap::new();
        // Populate top HashMap with your data
        
        // Create an instance of PredecessorLiftingStrategy
        predecessor_lifting_strat = PredecessorLiftingStrategy::new(&vertices, &top);
    
        // Perform lifting for a vertex and its predecessors
        if let Some(some_vertex) = vertices[0].as_ref() {
            predecessor_lifting_strat.lifted(some_vertex, &vertices, &top);
        }
    
        // Access the next vertex in the queue
        id = predecessor_lifting_strat.next().unwrap().identifier as i64;
        iter = None;
    } else {
        iter = None;
    }

    loop {
        print!("{:?}, ", id);
        result = pm.lift_v(id, &vertices, d);
        pm = result.0;
        did_update = result.1;

        // check if ϱ < Liftv (ϱ) for vertex v
        if did_update {
            did_update_this_master_loop = true;
        }
        // if the lifting strategy is based on the given input order
        if lifting_strategy == 0 || lifting_strategy == 3 {
            id += 1;
            if id == vertices.len() as i64 {
                id = 0;
                println!("reset!");
                loop_end = true; // check final loop termination
            }
        } else if lifting_strategy == 2 {
            match least_successor_lifting_strat.next() {
                Some(x) => id = x as i64,
                None => {
                    // hard reset the iterator
                    println!("reset!");
                    least_successor_lifting_strat = least_successor_order(&vertices).into_iter();
            
                    id = least_successor_lifting_strat.next().unwrap() as i64;
                    loop_end = true; // check final loop termination
                },
            }
        } else if lifting_strategy == 4 {
            match predecessor_lifting_strat.next() {
                Some(x) => id = x.identifier as i64,
                None => {
                    // hard reset the iterator
                    println!("reset!");
                    let top: HashMap<i64, bool> = HashMap::new();
                    predecessor_lifting_strat = PredecessorLiftingStrategy::new(&vertices, &top);
            
                    id = predecessor_lifting_strat.next().unwrap().identifier;
                    loop_end = true; // check final loop termination
                },
            }
        } else if ! iter.is_none() {
            match iter.as_mut().unwrap().next() {
                Some(x) => id = x as i64,
                None => {
                    // hard reset the iterator
                    println!("reset!");
                    iter = Some(Permutor::new_with_u64_key(length, seed.unwrap() as u64));
            
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
