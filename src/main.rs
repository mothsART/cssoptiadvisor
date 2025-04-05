use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

use cssoptiadvisor::parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut results: HashSet<String> = HashSet::new();
    let _ = parse(&cli.path, &mut results);

    for result in results {
        println!("{}", result.white().on_red());
    }
}
