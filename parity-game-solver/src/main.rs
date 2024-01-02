use std::fs;
use clap::Parser;
use core::cmp::max;

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

    let f: String = read_gm_file(args.gm_file, args.debug);



    println!("\nTerminated Succesfully");

}


fn read_gm_file(file_path: std::path::PathBuf, debug:bool) -> String {
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
    let identifier: i64 = to_int64(last.split(";").collect::<Vec<&str>>()[0]);

    // TODO: init
    println!("{}\n", identifier);


    // start decoding the lines
    // e.g.     0 0 0 1 "[X.]  |= 0";
    //          0 0 1 6453,20561,20562,30562;
    
    let mut d: i64 = 0;
    for part in lines.skip(1) {
        let priority = to_int64(part.split(" ").collect::<Vec<&str>>()[1]);

        // calculate the maximum priority (needed for progressMeasure)
        d = max(d, priority);
        println!("{}", part);
    }

    println!("{}", d);

    return String::new()
}

fn to_int64(f: &str) -> i64 {
    return f.parse::<i64>().unwrap()
}