use std::fs;
use clap::Parser;
use std::collections::HashSet;

// local imports
mod solver;
mod types;

use solver::{execute, execute_improved};
use types::ltl::Ltl;
use types::formula::Formula;
// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- --aut-file ../input/part2-1/dining_2.aut --mcf-file ../input/part2-1/invariantly_inevitably_eat.mcf --improved`
//       `main.exe --aut-file ../input/part2-1/dining_2.aut --mcf-file ../input/part2-1/invariantly_inevitably_eat.mcf -i`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .aut file
    #[arg(short, long)]
    aut_file: std::path::PathBuf,

    /// The path to the .mcf file
    #[arg(short, long)]
    mcf_file: std::path::PathBuf,

    /// Use the improved algorithm, or the regular one
    #[arg(short, long, default_value_t=false)]
    improved: bool,

    /// Print intermediate output
    #[arg(short, long, default_value_t=false)]
    debug: bool,

    /// Test if state `test_state` is in the output
    #[arg(short, long, default_value_t=-1)]
    test_state: i64,
}


fn main() {
    let args: Args = Args::parse();

    let f: Formula = read_mcf_file(args.mcf_file, args.debug);
    let ltl: Ltl = read_aut_file(args.aut_file, args.debug);

    // let mut result_set: HashSet<i64> = HashSet::new();
    if args.improved {
        let (result_set, iterations) = execute_improved(f, ltl);
        print_set(result_set, iterations, args.test_state);
    } else {
        let (result_set, iterations) = execute(f, ltl);
        print_set(result_set, iterations, args.test_state);
    }

    println!("\nTerminated Succesfully");

}

fn print_set(set: HashSet<i64>, iterations: i64, test_state: i64) {
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
    print!("Total number of fixpoint iterations: {}\n", iterations);
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
