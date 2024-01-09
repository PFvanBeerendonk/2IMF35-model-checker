use std::fs;
use clap::Parser;
use core::cmp::max;

mod types;
mod solver;

use solver::{main_algo};

use types::progress_measure::ProgressMeasure;
use types::vertex::Vertex;
use types::vertex::Vertices;
// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- -g input/dining_games/dining_2.invariantly_inevitably_eat.gm`
//       `main.exe --gm-file input/dining_games/dining_2.invariantly_inevitably_eat.gm`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .gm file
    #[arg(short, long)]
    gm_file: std::path::PathBuf,

    /// Use the "random order" lifting strategy. Otherwise "input order" is used
    #[arg(short, long, default_value_t=false)]
    random_lifting: bool,



    /// Print debug data
    #[arg(short, long, default_value_t=false)]
    debug: bool,

    /// Test if state `test_state` is in the output
    #[arg(short, long, default_value_t=-1)]
    test_state: i64,
}


fn main() {
    // Parse the arguments:
    let args: Args = Args::parse();

    let result = read_gm_file(args.gm_file, args.debug);
    println!("\n###   Finished Construction   ###\n");

    let pm:ProgressMeasure = result.0;
    let vertices: Vertices = result.1;
    let d: i64 = result.2;

    // set a seed
    let seed;
    if args.random_lifting {
        seed = Some(1234);
    } else {
        seed = None
    }

    main_algo(pm, &vertices, d, seed);

    println!("\n###   Terminated Succesfully   ###\n");
}


/**
 * Read file_path and create the ProgressMeasure and a list of all vertices;
 * 
 * basically handles line 1 in the algo of lecture 8, slide 20/43
 */
fn read_gm_file(file_path: std::path::PathBuf, debug:bool) -> (ProgressMeasure, Vertices, i64) {
    if !file_path.exists() {
        panic!("File {:?} does not exist", file_path);
    }
    if "gm" != file_path.extension().unwrap() {
        panic!("File {:?} should have been of type .gm", file_path);
    }

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.lines();

    // Initialize header, e.g. "parity 118206;"
    let first_line = lines.nth(0)
        .expect("File cannot be empty");

    let (des, last) = first_line.split_at(7);
    if "parity " != des {
        panic!(".mg file should contain `parity ` as first line, file started with {}", des);
    }

    // DOCS: https://www.win.tue.nl/~timw/downloads/amc/pgsolver.pdf (page 35, chapter 3.5)
    //   "It should be the maximal identifier of a node in the game"
    let max_identifier: i64 = to_int64(last.split(";").collect::<Vec<&str>>()[0]);

    // start decoding the lines
    // e.g.     0 0 0 1 "[X.]  |= 0";
    //          0 0 1 6453,20561,20562,30562;
    let mut d: i64 = 0;

    // Place to store all vertices
    const NONE: Option<Vertex> = None;
    let mut vertices: Vertices = vec![NONE; (max_identifier+1) as usize];
    for part in lines.skip(0) {
        // remove ; and split into parts
        let part_split = part[0..part.len()-1].split(" ").collect::<Vec<&str>>();

        let identifier = to_int64(part_split[0]);   // int < max_identifier
        let priority = to_int64(part_split[1]);     // int
        let owner = to_int64(part_split[2]);        // 0 or 1
        let successors = part_split[3];             // int,int,int,...

        let successors_list = successors.split(",").map(|i| to_int64(i)).collect::<Vec<i64>>();

        // optional name (NOT NEEDED FOR NOW)
        // let mut name = part_split.get(4);               // string, bound by "", not containing "

        // calculate the maximum priority (needed for progressMeasure)
        d = max(d, priority);

        // add vertex
        vertices[identifier as usize] = Some(Vertex::new(identifier, priority, owner, successors_list));
    }
    // See lecture8, slide 12 ==> d = 1 + max{p(v) | v \in V}
    d += 1;
    let pm = ProgressMeasure::new(max_identifier+1, d);
    
    return (pm, vertices, d)
}

fn to_int64(f: &str) -> i64 {
    return f.parse::<i64>().unwrap()
}
