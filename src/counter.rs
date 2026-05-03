use std::path::Path;
use crate::language::LanguageDef;
use crate::stats::FileStats;

pub fn count_file(
    path: &Path,
    lang: &'static LanguageDef,
    max_bytes: Option<u64>,
) -> Option<FileStats> {
    let metadata = std::fs::metadata(path).ok()?;
    let bytes = metadata.len();
    if let Some(max) = max_bytes {
        if bytes > max {
            return None;
        }
    }
    let content = std::fs::read_to_string(path).ok()?;
    let counts = count_content(&content, lang);
    Some(FileStats {
        path: path.to_path_buf(),
        language: lang.name,
        blank: counts.0,
        comment: counts.1,
        code: counts.2,
        bytes,
    })
}

pub fn count_content(content: &str, lang: &LanguageDef) -> (u64, u64, u64) {
    (0, 0, 0)
}
