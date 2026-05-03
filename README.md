# lcount

Count blank, comment, and code lines per language across your codebase. A fast [cloc](https://github.com/aldanial/cloc) alternative written in Rust.

```
┌────────────┬───────┬───────┬─────────┬───────┐
│ Language   │ Files │ Blank │ Comment │  Code │
╞════════════╪═══════╪═══════╪═════════╪═══════╡
│ Swift      │    42 │   380 │     210 │  3201 │
│ TypeScript │    18 │   140 │      55 │  1420 │
│ Python     │     5 │    60 │      30 │   420 │
├────────────┼───────┼───────┼─────────┼───────┤
│ SUM        │    65 │   580 │     295 │  5041 │
└────────────┴───────┴───────┴─────────┴───────┘
```

## Install

```sh
cargo install lcount
```

## Usage

Count a directory:

```sh
lcount src/
```

Count multiple paths:

```sh
lcount src/ tests/ scripts/
```

Count a single file:

```sh
lcount main.rs
```

## Output formats

Default is a pretty table. Use `--json` or `--csv` for machine-readable output:

```sh
lcount src/ --json
lcount src/ --csv
```

## Per-file breakdown

```sh
lcount src/ --by-file
```

## Sorting

Sort by any column (default: `code`):

```sh
lcount src/ --sort lines
lcount src/ --sort comment
lcount src/ --sort name
```

Available values: `lines`, `code`, `blank`, `comment`, `name`.

## Filtering

Skip directories by name:

```sh
lcount . --exclude-dir vendor --exclude-dir node_modules
```

Skip file extensions:

```sh
lcount . --exclude-ext json --exclude-ext lock
```

Count only specific languages:

```sh
lcount . --include-lang Rust --include-lang Python
```

Skip files larger than a size limit (in bytes):

```sh
lcount . --max-file-size 100000
```

## Git-aware mode

Use `git ls-files` to collect files, which respects `.gitignore`:

```sh
lcount . --git
```

## Supported languages

JavaScript, TypeScript, Python, Rust, Go, Java, C, C++, Swift, Kotlin, HTML, CSS, Shell, Ruby, PHP, Markdown, JSON, YAML, TOML, SQL.

## License

MIT
