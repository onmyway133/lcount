use std::path::PathBuf;
use serde::Serialize;
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
