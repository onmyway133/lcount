use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;
use crate::cli::Args;
use crate::language::detect;

pub fn collect_files(args: &Args) -> Vec<PathBuf> {
    let mut files = if args.git {
        collect_git_files(&args.paths)
    } else {
        collect_walk_files(args)
    };

    // Post-filters that apply in both modes
    files.retain(|p| {
        !is_excluded_by_ext(p, &args.exclude_exts)
            && !is_excluded_by_lang(p, &args.include_langs)
            && !exceeds_size(p, args.max_file_size)
    });

    files
}

fn collect_walk_files(args: &Args) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for root in &args.paths {
        if root.is_file() {
            files.push(root.clone());
            continue;
        }

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Always skip .not and .nod directories
                if name == ".not" || name == ".nod" {
                    return false;
                }
                // Skip user-supplied --exclude-dir names
                if e.file_type().is_dir() && args.exclude_dirs.iter().any(|d| d == name.as_ref()) {
                    return false;
                }
                true
            })
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            files.push(entry.into_path());
        }
    }

    files
}

fn collect_git_files(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for root in roots {
        let output = Command::new("git")
            .args(["ls-files", "--cached", "--others", "--exclude-standard"])
            .current_dir(root)
            .output();

        if let Ok(out) = output {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let path = root.join(line);
                if path.is_file() {
                    files.push(path);
                }
            }
        }
    }

    files
}

fn is_excluded_by_ext(path: &PathBuf, exclude_exts: &[String]) -> bool {
    if exclude_exts.is_empty() {
        return false;
    }
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        return exclude_exts.iter().any(|x| x == ext);
    }
    false
}

fn is_excluded_by_lang(path: &PathBuf, include_langs: &[String]) -> bool {
    if include_langs.is_empty() {
        return false;
    }
    match detect(path) {
        Some(lang) => !include_langs.iter().any(|l| l.eq_ignore_ascii_case(lang.name)),
        None => true,
    }
}

fn exceeds_size(path: &PathBuf, max: Option<u64>) -> bool {
    match max {
        None => false,
        Some(limit) => std::fs::metadata(path).map(|m| m.len() > limit).unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_args(root: &std::path::Path) -> Args {
        Args {
            paths: vec![root.to_path_buf()],
            json: false,
            csv: false,
            by_file: false,
            sort: crate::cli::SortKey::Code,
            exclude_dirs: vec![],
            exclude_exts: vec![],
            include_langs: vec![],
            max_file_size: None,
            git: false,
        }
    }

    #[test]
    fn collects_files_recursively() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(dir.path().join("a.rs"), "fn main() {}").unwrap();
        fs::write(sub.join("b.py"), "x = 1").unwrap();

        let files = collect_files(&make_args(dir.path()));
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn skips_not_directory() {
        let dir = TempDir::new().unwrap();
        let not_dir = dir.path().join(".not");
        fs::create_dir(&not_dir).unwrap();
        fs::write(not_dir.join("secret.rs"), "fn secret() {}").unwrap();
        fs::write(dir.path().join("visible.rs"), "fn visible() {}").unwrap();

        let files = collect_files(&make_args(dir.path()));
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("visible.rs"));
    }

    #[test]
    fn skips_exclude_dir() {
        let dir = TempDir::new().unwrap();
        let vendor = dir.path().join("vendor");
        fs::create_dir(&vendor).unwrap();
        fs::write(vendor.join("lib.rs"), "").unwrap();
        fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();

        let mut args = make_args(dir.path());
        args.exclude_dirs = vec!["vendor".into()];
        let files = collect_files(&args);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn skips_excluded_extension() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("a.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("b.json"), "{}").unwrap();

        let mut args = make_args(dir.path());
        args.exclude_exts = vec!["json".into()];
        let files = collect_files(&args);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("a.rs"));
    }

    #[test]
    fn max_file_size_filters_large_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("small.rs"), "x").unwrap();
        fs::write(dir.path().join("big.rs"), "x".repeat(1000)).unwrap();

        let mut args = make_args(dir.path());
        args.max_file_size = Some(100);
        let files = collect_files(&args);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("small.rs"));
    }
}
