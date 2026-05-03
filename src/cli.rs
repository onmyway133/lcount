use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cloc",
    version,
    about = "Count blank, comment, and code lines per language",
)]
pub struct Args {
    /// Paths to count (files or directories)
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Output as JSON
    #[arg(long, conflicts_with = "csv")]
    pub json: bool,

    /// Output as CSV
    #[arg(long, conflicts_with = "json")]
    pub csv: bool,

    /// Show per-file breakdown
    #[arg(long)]
    pub by_file: bool,

    /// Sort output column
    #[arg(long, value_enum, default_value_t = SortKey::Code)]
    pub sort: SortKey,

    /// Skip directories with these names (repeatable)
    #[arg(long = "exclude-dir", value_name = "DIR")]
    pub exclude_dirs: Vec<String>,

    /// Skip files with these extensions (repeatable)
    #[arg(long = "exclude-ext", value_name = "EXT")]
    pub exclude_exts: Vec<String>,

    /// Only count these languages (repeatable)
    #[arg(long = "include-lang", value_name = "LANG")]
    pub include_langs: Vec<String>,

    /// Skip files larger than this many bytes
    #[arg(long, value_name = "BYTES")]
    pub max_file_size: Option<u64>,

    /// Use git ls-files to collect files (respects .gitignore)
    #[arg(long)]
    pub git: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Lines,
    Code,
    Blank,
    Comment,
    Name,
}
