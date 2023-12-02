use std::fs;
use clap::Parser;

// END IMPORT


/// definition of ARGS 
// e.g.: `cargo run -- --file input/simple.ltl` --extended
//       `main.exe -f input/simple.ltl -e`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .ltl file to read
    #[arg(short, long)]
    file: std::path::PathBuf,

    /// Use the extended algorithm, or the regular one
    #[arg(short, long, default_value_t=false)]
    extended: bool,
}


fn main() {
    let args: Args = Args::parse();

    read_ltl_file(args.file);

    println!("\nTerminated Succesfully");
}


/**
 * Read ltl file and convert to DataType
 */
fn read_ltl_file(file_path: std::path::PathBuf) {
    if "ltl" != file_path.extension().unwrap() {
        panic!("File should have been of type .ltl");
    }

    println!("Unpacking file {:?}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
