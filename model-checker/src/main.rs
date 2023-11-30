use std::fs;
use clap::Parser;

/// definition of ARGS 
// e.g.: `cargo run -- input/simple.ltl`
#[derive(Parser)]
struct Cli {
    /// The path to the ltl file to read
    ltl_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    read_ltl_file(args.ltl_file);

    println!("Terminated Succesfully");
}

fn read_ltl_file(file_path: std::path::PathBuf) {
    // reads file of type ltl
    println!("In file {:?}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
