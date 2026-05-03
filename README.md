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
cloc src/
```

Count multiple paths:

```sh
cloc src/ tests/ scripts/
```

Count a single file:

```sh
cloc main.rs
```

## Output formats

Default is a pretty table. Use `--json` or `--csv` for machine-readable output:

```sh
cloc src/ --json
cloc src/ --csv
```

## Per-file breakdown

```sh
cloc src/ --by-file
```

## Sorting

Sort by any column (default: `code`):

```sh
cloc src/ --sort lines
cloc src/ --sort comment
cloc src/ --sort name
```

Available values: `lines`, `code`, `blank`, `comment`, `name`.

## Filtering

Skip directories by name:

```sh
cloc . --exclude-dir vendor --exclude-dir node_modules
```

Skip file extensions:

```sh
cloc . --exclude-ext json --exclude-ext lock
```

Count only specific languages:

```sh
cloc . --include-lang Rust --include-lang Python
```

Skip files larger than a size limit (in bytes):

```sh
cloc . --max-file-size 100000
```

## Git-aware mode

Use `git ls-files` to collect files, which respects `.gitignore`:

```sh
cloc . --git
```

## Supported languages

JavaScript, TypeScript, Python, Rust, Go, Java, C, C++, Swift, Kotlin, HTML, CSS, Shell, Ruby, PHP, Markdown, JSON, YAML, TOML, SQL.

## License

MIT
