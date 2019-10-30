use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    println!("The pattern to look for is \"{}\"", args.pattern);
    println!("The file is {:?}", args.path);

    let file = File::open(&args.path).expect("could not read file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while reader.buffer().is_empty() == false {
        let _ = reader.read_line(&mut line).expect("error reading line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
