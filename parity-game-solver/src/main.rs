use std::fs;
use clap::Parser;
use core::cmp::max;

mod types;
mod solver;

use solver::{main_algo};

use types::progress_measure::ProgressMeasure;
use types::vertex::Vertex;
use types::vertex::Vertices;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::borrow::Cow;
// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- -g input/dining_games/dining_2.invariantly_inevitably_eat.gm`
//       `main.exe --gm-file input/dining_games/dining_2.invariantly_inevitably_eat.gm`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .gm file
    #[arg(short, long)]
    gm_file: Option<std::path::PathBuf>,

    /// The path to the folder with all the .gm files
    #[arg(short, long, conflicts_with="gm_file")]
    folder: Option<std::path::PathBuf>,

    /// Only if there is a folder specified; use all the folowing lifting strategies
    /// Use multiple with -m 0 -m 1 -m 2 -m 3 -m 4 -m 5
    #[arg(short, long, default_values_t=vec![0,1,2,3,4], conflicts_with="lifting_strategy", verbatim_doc_comment)]
    multiple_lifting_strategies: Vec<i64>,

    /// Where the output folder is located, emtpy if there should be no output
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    /// Which lifting strategy is used.
    /// 0 for "input order" lifting strategy
    /// 1 for "random order" lifting strategy
    /// 2 for "least successors" lifting strategy
    /// 3 for fourth lifting strategy TODO
    #[arg(short, long, default_value_t=0, verbatim_doc_comment, conflicts_with="multiple_lifting_strategies")]
    lifting_strategy: i64,

    /// Give a default seed, only used if `lifting_strategy` is 1 (random order)
    #[arg(short, long, default_value_t=1234)]
    seed: i64,

    /// Whether the debug output should be printed
    #[arg(short, long, default_value_t=false)]
    debug: bool,
}


fn main() {
    // Parse the arguments:
    let args: Args = Args::parse();
    if args.gm_file.is_some() {
        let result = read_gm_file(args.gm_file.unwrap());
        
        if args.debug {
            println!("\n###   Finished Construction   ###\n");
        }
        let pm:ProgressMeasure = result.0;
        let vertices: Vertices = result.1;
        let d: i64 = result.2;
    
        // set a seed
        let seed;
        if args.lifting_strategy == 1 {
            seed = Some(args.seed);
        } else {
            seed = None
        }
    
        main_algo(pm, &vertices, d, args.lifting_strategy, seed, None, args.debug);
    
        if args.debug {
            println!("\n###   Terminated Succesfully   ###\n");
        }
    } else if args.folder.is_some() {
        run_folder(&args);
    }
}


/**
 * Read file_path and create the ProgressMeasure and a list of all vertices;
 * 
 * basically handles line 1 in the algo of lecture 8, slide 20/43
 */
fn read_gm_file(file_path: std::path::PathBuf) -> (ProgressMeasure, Vertices, i64) {
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


fn run_folder(args: &Args) {
    let mut folder_name = "".to_string();
    if let Some(folder) = &args.folder {
        for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
            if let Some(gm_file) = entry.file_name().to_str() {
                if gm_file.ends_with(".gm") {
                    println!("gm file {}", gm_file);
                    // read the gm file
                    let result = read_gm_file(entry.path().to_path_buf());
                    if args.debug {
                        println!("\n###   Finished Construction   ###\n");
                    }
                    let pm: ProgressMeasure = result.0;
                    let vertices: Vertices = result.1;
                    let d: i64 = result.2;
                
                    // set a seed
                    let seed;
                    seed = Some(args.seed);

                    // loop over all the lifting strategies
                    for lifting_strat in args.multiple_lifting_strategies.iter() {
                        
                        let output = PathBuf::from(
                            [
                                args.output.clone().unwrap_or_default().to_string_lossy().into(),
                                folder_name.clone(),
                                "/".to_string(),
                                Cow::Borrowed(&gm_file[0..gm_file.len() - 3]).to_string(),
                                "-lifting-".to_string(),
                                lifting_strat.to_string(),
                                ".txt".to_string(),
                            ]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<String>(),
                        );
                        if !std::path::Path::new(&output).exists() {
                            let _ = fs::create_dir_all([
                                args.output.clone().unwrap_or_default().to_string_lossy().into(),
                                folder_name.clone(),
                                ]
                                .iter().map(|s| s.to_string()).collect::<String>());
                        }

                        main_algo(pm.clone(), &(vertices.clone()), d, *lifting_strat, seed, Some(output.clone()), args.debug);
                        if args.debug {
                            println!("\n###   Terminated Succesfully   ###\n");
                        }
                    }
                } else {
                    folder_name = entry.file_name().to_string_lossy().to_string();
                }
            }
        }
    }
}
