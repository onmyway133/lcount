mod cli;
mod counter;
mod language;
mod output;
mod stats;
mod walker;

use clap::Parser;
use rayon::prelude::*;

fn main() {
    let args = cli::Args::parse();

    let files = walker::collect_files(&args);

    let file_stats: Vec<stats::FileStats> = files
        .par_iter()
        .filter_map(|path| {
            let lang = language::detect(path)?;
            counter::count_file(path, lang, args.max_file_size)
        })
        .collect();

    let report = stats::aggregate(file_stats, args.by_file, args.sort);

    if args.json {
        output::json::render(&report);
    } else if args.csv {
        output::csv::render(&report);
    } else {
        output::table::render(&report);
    }
}
