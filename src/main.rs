mod cli;
mod counter;
mod language;
mod output;
mod stats;
mod walker;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    let files = walker::collect_files(&args);
    let _ = (files, args);
}
