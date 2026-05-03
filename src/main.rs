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

    let paths_exist = args.paths.iter().all(|p| p.exists());
    if !paths_exist {
        let missing: Vec<_> = args.paths.iter().filter(|p| !p.exists()).collect();
        for p in &missing {
            eprintln!("error: path not found: {}", p.display());
        }
        std::process::exit(1);
    }

    let files = walker::collect_files(&args);

    if files.is_empty() {
        eprintln!("No source files found.");
        std::process::exit(0);
    }

    let file_stats: Vec<stats::FileStats> = files
        .par_iter()
        .filter_map(|path| {
            let lang = language::detect(path)?;
            counter::count_file(path, lang, args.max_file_size)
        })
        .collect();

    if file_stats.is_empty() {
        eprintln!("No recognized source files found.");
        std::process::exit(0);
    }

    let report = stats::aggregate(file_stats, args.by_file, args.sort);

    if args.json {
        output::json::render(&report);
    } else if args.csv {
        output::csv::render(&report);
    } else {
        output::table::render(&report);
    }
}
