use std::fs;
use clap::Parser;

// local imports
mod emerson_lei;
mod types;

use emerson_lei::{execute, execute_improved};
use types::ltl::Ltl;
use types::formula::Formula;
// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- --aut-file ../input/part2-1/dining_2.aut --mcf-file ../input/part2-1/invariantly_inevitably_eat.mcf --improved
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
}


fn main() {
    let args: Args = Args::parse();

    let f: Formula = read_mcf_file(args.mcf_file);
    let ltl: Ltl = read_aut_file(args.aut_file);

    
    if args.improved {
        execute_improved(f, ltl);

    } else {
        execute(f, ltl);
    }

    println!("\nTerminated Succesfully");

}


/**
 * Read .aut file and convert to DataType
 */
fn read_aut_file(file_path: std::path::PathBuf) -> Ltl {
    if "aut" != file_path.extension().unwrap() {
        panic!("File {:?} should have been of type .aut", file_path);
    }

    println!("Unpacking file {:?}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");


    // TODO: return LTL data struct
    let x: Ltl = Ltl {temp: 1};
    return x;
}

/**
 * Read .mcf file and convert to DataType
 */
fn read_mcf_file(file_path: std::path::PathBuf) -> Formula {
    if "mcf" != file_path.extension().unwrap() {
        panic!("File {:?} should have been of type .mcf", file_path);
    }


    let f: Formula = Formula {temp:11};
    return f; 
}
