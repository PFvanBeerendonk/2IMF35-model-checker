use std::fs;
use clap::Parser;
use std::collections::HashSet;

// local imports
mod solver;
mod types;

use solver::{execute, execute_improved, find_formula_statistics};
use types::ltl::Ltl;
use types::formula::Formula;
use std::path::PathBuf;
use walkdir::WalkDir;
// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- --aut-file ../input/part2-1/dining_2.aut --mcf-file ../input/part2-1/invariantly_inevitably_eat.mcf --improved`
//       `main.exe --aut-file ../input/part2-1/dining_2.aut --mcf-file ../input/part2-1/invariantly_inevitably_eat.mcf -i`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .aut file
    #[structopt(short, long, conflicts_with = "folder")]
    aut_file: Option<PathBuf>,

    /// The path to the .mcf file
    #[structopt(short, long, conflicts_with = "folder")]
    mcf_file: Option<PathBuf>,

    /// The path to the folder containing aut and mcf files
    #[structopt(short, long)]
    folder: Option<PathBuf>,

    /// Use the improved algorithm, or the regular one
    #[arg(short, long, default_value_t=false)]
    improved: bool,

    /// Print intermediate output
    #[arg(short, long, default_value_t=false)]
    debug: bool,

    /// Print statistics
    #[arg(short, long, default_value_t=false)]
    statistics: bool,

    /// Test if state `test_state` is in the output
    #[arg(short, long, default_value_t=-1)]
    test_state: i64,
}


fn main() {
    let args: Args = Args::parse();


    let mut benchmarks = "".to_string();
    if args.folder.is_some() {
        run_folder(&args, &mut benchmarks);
        fs::write("./benchmarks.txt", benchmarks).expect("Unable to write file");
    }
    else if args.mcf_file.is_some() && args.aut_file.is_some() {
        let f: Formula = read_mcf_file(args.mcf_file.unwrap(), args.debug);
        let ltl: Ltl = read_aut_file(args.aut_file.unwrap(), args.debug);
        if args.statistics {
            let (nesting_depth, alteration_depth, dependent_alteration_depth) = find_formula_statistics(&f.root_node);
            print!("The nesting depth for this formula is: {}\n", nesting_depth);
            print!("The alteration depth for this formula is: {}\n", alteration_depth);
            print!("The dependent alteration depth for this formula is: {}\n", dependent_alteration_depth);
        }

        run_and_print(f, ltl, args.improved, args.test_state, args.statistics);

        println!("\nTerminated Succesfully");
    }
    else {
        println!("Please specify the required files with --aut-file and --mcf-file");
    }

}

fn run_folder(args: &Args, benchmarks: &mut String) {
    if let Some(folder) = &args.folder {
        for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
            if let Some(aut_file_name) = entry.file_name().to_str() {
                if aut_file_name.ends_with(".aut") {
                    let original_ltl = read_aut_file(entry.path().to_path_buf(), args.debug);
                    for mcf_entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
                        let ltl = original_ltl.clone();
                        if let Some(mcf_file_name) = mcf_entry.file_name().to_str() {
                            if mcf_file_name.ends_with(".mcf") {
                                let f = read_mcf_file(mcf_entry.path().to_path_buf(), args.debug);
                                let (nesting_depth, alteration_depth, dependent_alteration_depth) = find_formula_statistics(&f.root_node);
                                let now = std::time::Instant::now();
                                let elapsed: std::time::Duration;
                                run_and_print(f, ltl, args.improved, args.test_state, args.statistics);
                                elapsed = now.elapsed();
                                
                                benchmarks.push_str(&format!(
                                    "Running {mcf_file_name} on {aut_file_name} took {:.2?}. Statistics: {}, {}, {}\r\n",
                                    elapsed, nesting_depth, alteration_depth, dependent_alteration_depth)
                                );
                                println!(
                                    "Running {mcf_file_name} on {aut_file_name} took {:.2?}. Statistics: {}, {}, {}",
                                    elapsed, nesting_depth, alteration_depth, dependent_alteration_depth);
                            }
                        }
                    }
                    println!("");
                }
            }
        }
    }
}

fn run_and_print(f: Formula, ltl: Ltl, improved: bool, test_state: i64, statistics:bool) {
    if improved {
        let (result_set, iterations) = execute_improved(f, ltl);
        print_set(result_set, iterations, test_state, statistics);
    } else {
        let (result_set, iterations) = execute(f, ltl);
        print_set(result_set, iterations, test_state, statistics);
    }
}

fn print_set(set: HashSet<i64>, iterations: i64, test_state: i64, statistics:bool) {
    print!("Resulting set: ");
    print!("{{");
    for (i, el) in set.iter().enumerate()  {
        print!("{}", el);
        if i != set.len()-1 {
            print!(",");
        }
    }
    println!("}}");
    if test_state != -1 {
        println!("The state {} is in the resulting set: {}", test_state, set.contains(&test_state));
    }
    if statistics {
        println!("Total number of fixpoint iterations: {}", iterations);
    }
}


/**
 * Read .aut file and convert to DataType
 */
fn read_aut_file(file_path: std::path::PathBuf, debug: bool) -> Ltl {
    if !file_path.exists() {
        panic!("File {:?} does not exist", file_path);
    }

    if "aut" != file_path.extension().unwrap() {
        panic!("File {:?} should have been of type .aut", file_path);
    }

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.lines();

    // Initialize header, e.g. "des (123,456,789)     "
    let first_line = lines.nth(0)
        .expect("File cannot be empty");

    let (des, last) = first_line.split_at(5);
    if "des (" != des {
        panic!(".aut file should contain `des (...)` as first line, file started with {}", des);
    }

    let seconds: Vec<&str> = last.split(")").collect();
    let nums: Vec<&str> = seconds[0].split(",").collect();

    let mut ltl: Ltl = Ltl::new(
        to_int64(nums[0]),
        to_int64(nums[1]), 
        to_int64(nums[2])
    );

    // initialize transitions

    for part in lines.skip(1) {
        let (start, last) = part.split_at(1);
        if "(" != start {
            panic!("Line '{}' did not start with '('", part)
        }
        let seconds: Vec<&str> = last.split(")").collect();
        let nums: Vec<&str> = seconds[0].split(",").collect();
        let label: &str = nums[1].split("\"").nth(1)
            .expect("No label found in {part}");

        ltl.add_transition(
            to_int64(nums[0]), 
            label, 
            to_int64(nums[2]),
            debug
        );
    }

    return ltl;
}

fn to_int64(f: &str) -> i64 {
    return f.parse::<i64>().unwrap()
}

/**
 * Read .mcf file and convert to DataType
 */
fn read_mcf_file(file_path: std::path::PathBuf, debug: bool) -> Formula {
    if !file_path.exists() {
        panic!("File {:?} does not exist", file_path);
    }

    if "mcf" != file_path.extension().unwrap() {
        panic!("File {:?} should have been of type .mcf", file_path);
    }


    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let f: Formula = Formula::new(contents, debug);
    return f; 
}
