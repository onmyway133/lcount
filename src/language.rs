use std::collections::HashMap;
use std::sync::OnceLock;

pub type LangName = &'static str;

#[derive(Debug, Clone)]
pub struct LanguageDef {
    pub name: LangName,
    pub extensions: &'static [&'static str],
    pub line_comment: &'static [&'static str],
    pub block_comment: Option<(&'static str, &'static str)>,
    pub multiline_string: &'static [&'static str],
}

pub static LANGUAGES: &[LanguageDef] = &[
    LanguageDef {
        name: "JavaScript",
        extensions: &["js", "mjs", "cjs"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &["`"],
    },
    LanguageDef {
        name: "TypeScript",
        extensions: &["ts", "tsx"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &["`"],
    },
    LanguageDef {
        name: "Python",
        extensions: &["py", "pyw"],
        line_comment: &["#"],
        block_comment: None,
        multiline_string: &["\"\"\"", "'''"],
    },
    LanguageDef {
        name: "Rust",
        extensions: &["rs"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "Go",
        extensions: &["go"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &["`"],
    },
    LanguageDef {
        name: "Java",
        extensions: &["java"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "C",
        extensions: &["c", "h"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "C++",
        extensions: &["cpp", "cc", "cxx", "hpp", "hxx"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "Swift",
        extensions: &["swift"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "Kotlin",
        extensions: &["kt", "kts"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "HTML",
        extensions: &["html", "htm"],
        line_comment: &[],
        block_comment: Some(("<!--", "-->")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "CSS",
        extensions: &["css", "scss", "sass"],
        line_comment: &["//"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "Shell",
        extensions: &["sh", "bash", "zsh"],
        line_comment: &["#"],
        block_comment: None,
        multiline_string: &[],
    },
    LanguageDef {
        name: "Ruby",
        extensions: &["rb", "rake"],
        line_comment: &["#"],
        block_comment: Some(("=begin", "=end")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "PHP",
        extensions: &["php"],
        line_comment: &["//", "#"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
    LanguageDef {
        name: "Markdown",
        extensions: &["md", "markdown"],
        line_comment: &[],
        block_comment: None,
        multiline_string: &[],
    },
    LanguageDef {
        name: "JSON",
        extensions: &["json"],
        line_comment: &[],
        block_comment: None,
        multiline_string: &[],
    },
    LanguageDef {
        name: "YAML",
        extensions: &["yml", "yaml"],
        line_comment: &["#"],
        block_comment: None,
        multiline_string: &[],
    },
    LanguageDef {
        name: "TOML",
        extensions: &["toml"],
        line_comment: &["#"],
        block_comment: None,
        multiline_string: &[],
    },
    LanguageDef {
        name: "SQL",
        extensions: &["sql"],
        line_comment: &["--"],
        block_comment: Some(("/*", "*/")),
        multiline_string: &[],
    },
];

static EXT_INDEX: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();

fn ext_index() -> &'static HashMap<&'static str, usize> {
    EXT_INDEX.get_or_init(|| {
        let mut map = HashMap::new();
        for (i, lang) in LANGUAGES.iter().enumerate() {
            for ext in lang.extensions {
                map.entry(*ext).or_insert(i);
            }
        }
        map
    })
}

pub fn lang_by_extension(ext: &str) -> Option<&'static LanguageDef> {
    let idx = *ext_index().get(ext)?;
    Some(&LANGUAGES[idx])
}

pub fn lang_by_shebang(first_line: &str) -> Option<&'static LanguageDef> {
    if !first_line.starts_with("#!") {
        return None;
    }
    let line = first_line.to_lowercase();
    if line.contains("python") {
        return lang_by_extension("py");
    }
    if line.contains("ruby") {
        return lang_by_extension("rb");
    }
    if line.contains("node") {
        return lang_by_extension("js");
    }
    if line.contains("bash") {
        return lang_by_extension("bash");
    }
    if line.contains("zsh") {
        return lang_by_extension("zsh");
    }
    if line.contains("/sh") || line.ends_with(" sh") {
        return lang_by_extension("sh");
    }
    None
}

pub fn detect(path: &std::path::Path) -> Option<&'static LanguageDef> {
    let ext = path.extension()?.to_str()?;
    if let Some(lang) = lang_by_extension(ext) {
        return Some(lang);
    }
    // Shebang fallback for extensionless files
    let content = std::fs::read_to_string(path).ok()?;
    let first_line = content.lines().next()?;
    lang_by_shebang(first_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_resolves_to_language() {
        assert_eq!(lang_by_extension("rs").map(|l| l.name), Some("Rust"));
        assert_eq!(lang_by_extension("py").map(|l| l.name), Some("Python"));
        assert_eq!(lang_by_extension("go").map(|l| l.name), Some("Go"));
        assert_eq!(lang_by_extension("ts").map(|l| l.name), Some("TypeScript"));
        assert_eq!(lang_by_extension("java").map(|l| l.name), Some("Java"));
        assert_eq!(lang_by_extension("swift").map(|l| l.name), Some("Swift"));
        assert_eq!(lang_by_extension("kt").map(|l| l.name), Some("Kotlin"));
        assert_eq!(lang_by_extension("html").map(|l| l.name), Some("HTML"));
        assert_eq!(lang_by_extension("css").map(|l| l.name), Some("CSS"));
        assert_eq!(lang_by_extension("sh").map(|l| l.name), Some("Shell"));
        assert_eq!(lang_by_extension("rb").map(|l| l.name), Some("Ruby"));
        assert_eq!(lang_by_extension("php").map(|l| l.name), Some("PHP"));
        assert_eq!(lang_by_extension("md").map(|l| l.name), Some("Markdown"));
        assert_eq!(lang_by_extension("yml").map(|l| l.name), Some("YAML"));
        assert_eq!(lang_by_extension("toml").map(|l| l.name), Some("TOML"));
        assert_eq!(lang_by_extension("sql").map(|l| l.name), Some("SQL"));
        assert!(lang_by_extension("xyz").is_none());
    }

    #[test]
    fn all_languages_have_at_least_one_extension() {
        for lang in LANGUAGES {
            assert!(
                !lang.extensions.is_empty(),
                "{} has no extensions",
                lang.name
            );
        }
    }

    #[test]
    fn shebang_python() {
        assert_eq!(
            lang_by_shebang("#!/usr/bin/env python3").map(|l| l.name),
            Some("Python")
        );
    }

    #[test]
    fn shebang_bash() {
        assert_eq!(
            lang_by_shebang("#!/bin/bash").map(|l| l.name),
            Some("Shell")
        );
    }

    #[test]
    fn shebang_node() {
        assert_eq!(
            lang_by_shebang("#!/usr/bin/env node").map(|l| l.name),
            Some("JavaScript")
        );
    }

    #[test]
    fn shebang_ruby() {
        assert_eq!(
            lang_by_shebang("#!/usr/bin/env ruby").map(|l| l.name),
            Some("Ruby")
        );
    }

    #[test]
    fn non_shebang_returns_none() {
        assert!(lang_by_shebang("fn main() {}").is_none());
    }
}
