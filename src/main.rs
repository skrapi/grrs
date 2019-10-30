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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let file = File::open(&args.path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line?);
        }
    }

    Ok(())
}
