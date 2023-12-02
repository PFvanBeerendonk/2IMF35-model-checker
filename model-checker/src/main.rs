use std::fs;
use clap::Parser;

/// definition of ARGS 
// e.g.: `cargo run -- --file input/simple.ltl` --extended
//       `main.exe -f input/simple.ltl -e`
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the ltl file to read
    #[arg(short, long)]
    file: std::path::PathBuf,

    #[arg(short, long, default_value_t=false)]
    extended: bool,
}


fn main() {
    let args: Args = Args::parse();

    read_ltl_file(args.file);

    println!("\nTerminated Succesfully");
}

fn read_ltl_file(file_path: std::path::PathBuf) {
    // reads file of type ltl
    println!("In file {:?}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
