use std::collections::HashMap;
use std::path::PathBuf;
use serde::Serialize;
use crate::cli::SortKey;
use crate::language::LangName;

#[derive(Debug, Clone, Default, Serialize)]
pub struct FileStats {
    pub path: PathBuf,
    pub language: LangName,
    pub blank: u64,
    pub comment: u64,
    pub code: u64,
    pub bytes: u64,
}

impl FileStats {
    pub fn total_lines(&self) -> u64 {
        self.blank + self.comment + self.code
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LangStats {
    pub language: LangName,
    pub files: u64,
    pub blank: u64,
    pub comment: u64,
    pub code: u64,
}

impl LangStats {
    pub fn merge(&mut self, f: &FileStats) {
        self.files += 1;
        self.blank += f.blank;
        self.comment += f.comment;
        self.code += f.code;
    }

    pub fn total_lines(&self) -> u64 {
        self.blank + self.comment + self.code
    }
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub by_lang: Vec<LangStats>,
    pub by_file: Option<Vec<FileStats>>,
    pub totals: LangStats,
}

pub fn aggregate(
    mut file_stats: Vec<FileStats>,
    include_by_file: bool,
    sort: SortKey,
) -> Report {
    let mut lang_map: HashMap<LangName, LangStats> = HashMap::new();

    for fs in &file_stats {
        let entry = lang_map.entry(fs.language).or_insert_with(|| LangStats {
            language: fs.language,
            ..Default::default()
        });
        entry.merge(fs);
    }

    let mut totals = LangStats {
        language: "SUM",
        ..Default::default()
    };
    let mut by_lang: Vec<LangStats> = lang_map.into_values().collect();

    sort_lang_stats(&mut by_lang, sort);

    for ls in &by_lang {
        totals.files += ls.files;
        totals.blank += ls.blank;
        totals.comment += ls.comment;
        totals.code += ls.code;
    }

    let by_file = if include_by_file {
        sort_file_stats(&mut file_stats, sort);
        Some(file_stats)
    } else {
        None
    };

    Report { by_lang, by_file, totals }
}

fn sort_lang_stats(stats: &mut Vec<LangStats>, key: SortKey) {
    stats.sort_by(|a, b| match key {
        SortKey::Code    => b.code.cmp(&a.code),
        SortKey::Lines   => b.total_lines().cmp(&a.total_lines()),
        SortKey::Blank   => b.blank.cmp(&a.blank),
        SortKey::Comment => b.comment.cmp(&a.comment),
        SortKey::Name    => a.language.cmp(b.language),
    });
}

fn sort_file_stats(stats: &mut Vec<FileStats>, key: SortKey) {
    stats.sort_by(|a, b| match key {
        SortKey::Code    => b.code.cmp(&a.code),
        SortKey::Lines   => b.total_lines().cmp(&a.total_lines()),
        SortKey::Blank   => b.blank.cmp(&a.blank),
        SortKey::Comment => b.comment.cmp(&a.comment),
        SortKey::Name    => a.path.cmp(&b.path),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_file(lang: LangName, blank: u64, comment: u64, code: u64) -> FileStats {
        FileStats {
            path: PathBuf::from(format!("fake.{lang}")),
            language: lang,
            blank,
            comment,
            code,
            bytes: 0,
        }
    }

    #[test]
    fn aggregates_same_language() {
        let files = vec![
            make_file("Rust", 1, 2, 10),
            make_file("Rust", 0, 1, 5),
        ];
        let report = aggregate(files, false, SortKey::Code);
        assert_eq!(report.by_lang.len(), 1);
        let ls = &report.by_lang[0];
        assert_eq!(ls.files, 2);
        assert_eq!(ls.blank, 1);
        assert_eq!(ls.comment, 3);
        assert_eq!(ls.code, 15);
    }

    #[test]
    fn aggregates_multiple_languages() {
        let files = vec![
            make_file("Rust", 1, 0, 10),
            make_file("Python", 2, 3, 5),
        ];
        let report = aggregate(files, false, SortKey::Code);
        assert_eq!(report.by_lang.len(), 2);
        assert_eq!(report.totals.files, 2);
        assert_eq!(report.totals.blank, 3);
        assert_eq!(report.totals.code, 15);
    }

    #[test]
    fn sort_by_name_ascending() {
        let files = vec![
            make_file("Rust", 0, 0, 5),
            make_file("Python", 0, 0, 10),
            make_file("Go", 0, 0, 1),
        ];
        let report = aggregate(files, false, SortKey::Name);
        let names: Vec<_> = report.by_lang.iter().map(|l| l.language).collect();
        assert_eq!(names, vec!["Go", "Python", "Rust"]);
    }

    #[test]
    fn by_file_is_populated_when_requested() {
        let files = vec![make_file("Rust", 1, 1, 1)];
        let report = aggregate(files, true, SortKey::Code);
        assert!(report.by_file.is_some());
        assert_eq!(report.by_file.unwrap().len(), 1);
    }

    #[test]
    fn by_file_is_none_when_not_requested() {
        let files = vec![make_file("Rust", 1, 1, 1)];
        let report = aggregate(files, false, SortKey::Code);
        assert!(report.by_file.is_none());
    }
}
