use crate::types::progress_measure::ProgressMeasure;
use crate::types::vertex::Vertices;
use crate::types::lifting_strategies::{least_successor_order, most_successor_order, FocusListLiftingStrategy, PredecessorLiftingStrategy};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use permutation_iterator::Permutor;
use std::collections::HashMap;
use std::time::Instant;
use std::cmp::max;

pub fn main_algo(progress_measure: ProgressMeasure, vertices: &Vertices, d: i64, lifting_strategy: i64, seed: Option<i64>, output: Option<PathBuf>, debug: bool) {
    let mut total_lifts = 0;
    let mut successful_lifts = 0;

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
    let mut most_successor_lifting_strat = Default::default();
    let mut predecessor_lifting_strat = Default::default();

    // measure the time that the program runs
    let now = Instant::now();
    // if a seed is provided, we will make a hashed iterator
    if lifting_strategy == 1 {
        iter = Some(Permutor::new_with_u64_key(length, seed.unwrap() as u64));

        id = iter.as_mut().unwrap().next().unwrap() as i64;
    } else if lifting_strategy == 2 {
        least_successor_lifting_strat = least_successor_order(&vertices).into_iter();
        id = least_successor_lifting_strat.next().unwrap() as i64;
        iter = None;
    } else if lifting_strategy == 3 {
        most_successor_lifting_strat = most_successor_order(&vertices).into_iter();
        id = most_successor_lifting_strat.next().unwrap() as i64;
        iter = None;
    } else if lifting_strategy == 4 {
        // The top hashmap always starts empty
        let top: HashMap<i64, bool> = HashMap::new();
        
        // Create an instance of PredecessorLiftingStrategy
        predecessor_lifting_strat = PredecessorLiftingStrategy::new(&vertices, &top);
    
        // Access the next vertex in the queue
        id = predecessor_lifting_strat.next().unwrap().identifier as i64;
        iter = None;
    } else {
        iter = None;
    }

    let pm_to_return; 

    if lifting_strategy == 5 {
        let mut strategy = FocusListLiftingStrategy::new();
        pm_to_return = strategy.run(
            &progress_measure,
            &vertices,
            d,
            max((length / 10).try_into().unwrap(), 1),
            length.try_into().unwrap(),
        );
        total_lifts = strategy.total_lifts;
        successful_lifts = strategy.total_successful_lifts;
    } else {
        loop {
            result = pm.lift_v(id, &vertices, d);
            total_lifts += 1;
            pm = result.0;
            did_update = result.1;
    
            // check if ϱ < Liftv (ϱ) for vertex v
            if did_update {
                did_update_this_master_loop = true;
                successful_lifts += 1;
            }
            // if the lifting strategy is based on the given input order
            if lifting_strategy == 0  {
                id += 1;
                if id == vertices.len() as i64 {
                    id = 0;
                    loop_end = true; // check final loop termination
                }
            } else if lifting_strategy == 2 {
                match least_successor_lifting_strat.next() {
                    Some(x) => id = x as i64,
                    None => {
                        // hard reset the iterator
                        least_successor_lifting_strat = least_successor_order(&vertices).into_iter();
                
                        id = least_successor_lifting_strat.next().unwrap() as i64;
                        loop_end = true; // check final loop termination
                    },
                }
            } else if lifting_strategy == 3 {
                match most_successor_lifting_strat.next() {
                    Some(x) => id = x as i64,
                    None => {
                        // hard reset the iterator
                        most_successor_lifting_strat = most_successor_order(&vertices).into_iter();
                
                        id = most_successor_lifting_strat.next().unwrap() as i64;
                        loop_end = true; // check final loop termination
                    },
                }
            } else if lifting_strategy == 4 {
                match predecessor_lifting_strat.next() {
                    Some(x) => id = x.identifier as i64,
                    None => {
                        // hard reset the iterator
                        let top: HashMap<i64, bool> = pm.data.iter().enumerate()
                            .filter_map(|(i, row)| if row.is_none() { Some((i as i64, true)) } else { None })
                            .collect();
                        predecessor_lifting_strat = PredecessorLiftingStrategy::new(&vertices, &top);
                        
                        let next_lift = predecessor_lifting_strat.next();
                        if next_lift.is_some() {
                            id = next_lift.unwrap().identifier;
                        }
                        loop_end = true; // check final loop termination
                    },
                }
            } else if ! iter.is_none() {
                match iter.as_mut().unwrap().next() {
                    Some(x) => id = x as i64,
                    None => {
                        // hard reset the iterator
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
                    pm_to_return = pm.clone();
                    break;
                } else {
                    did_update_this_master_loop = false;
                }
            }
        }
    }

    // checking stuff
    let elapsed = now.elapsed();
    

    let non_t_count = pm_to_return.data.iter().flatten().count();
    let size = pm_to_return.data.iter().count();
    if output.is_some() {
        println!("###   Finished Calculations   ### (for {} in {:.4?})", output.clone().expect("no output file").as_path().display().to_string(), elapsed);

        let file = File::create(output.expect("Output file name could not be read").as_path()).expect("Unable to create file");
        let mut f = BufWriter::new(file);

        
        write!(f, "Size:          {}\n", size).expect("unable to write");
        write!(f, "Nr of non-T:   {}\n", non_t_count).expect("unable to write");
        write!(f, "Nr of T:       {}\n", size - non_t_count).expect("unable to write");
        write!(f, "Time:          {:.4?}\n", elapsed).expect("unable to write");
        write!(f, "Nr of Lifts:   {}\n", total_lifts).expect("unable to write");
        write!(f, "Success lifts: {}\n", successful_lifts).expect("unable to write");
        write!(f, "\n\n{: <10} | ϱ()\n", "ID").expect("unable to write");
        write!(f, "-----------+------------------------------------\n").expect("unable to write");
        for (i, row) in pm_to_return.data.iter().enumerate()  {
            if row.is_none() {
                write!(f, "{: <10} | T\n", i).expect("unable to write");
            } else {
                write!(f, "{: <10} | {:?}\n", i, row.clone().unwrap()).expect("unable to write");
            }
        }
    }

    if debug {
        println!("\n###   Finished Calculations   ### (in {:.4?})\n", elapsed);
        println!("Size:         {}", size);
        println!("Nr of non-T:  {}", non_t_count);
        println!("Nr of T:      {}", size - non_t_count);



        println!("\n\n{: <10} | ϱ()", "ID");
        println!("-----------+------------------------------------");
        for (i, row) in pm_to_return.data.iter().enumerate()  {
            if row.is_none() {
                println!("{: <10} | T", i);
            } else {
                println!("{: <10} | {:?}", i, row.clone().unwrap());
            }
        }
    }
}
