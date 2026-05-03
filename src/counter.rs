use std::path::Path;
use crate::language::LanguageDef;
use crate::stats::FileStats;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Code,
    InBlockComment,
    InMultilineString(usize), // index into lang.multiline_string
}

pub fn count_file(
    path: &Path,
    lang: &'static LanguageDef,
    max_bytes: Option<u64>,
) -> Option<FileStats> {
    let bytes = std::fs::metadata(path).ok()?.len();
    if let Some(max) = max_bytes {
        if bytes > max {
            return None;
        }
    }
    let content = std::fs::read_to_string(path).ok()?;
    let (blank, comment, code) = count_content(&content, lang);
    Some(FileStats {
        path: path.to_path_buf(),
        language: lang.name,
        blank,
        comment,
        code,
        bytes,
    })
}

pub fn count_content(content: &str, lang: &LanguageDef) -> (u64, u64, u64) {
    let mut blank = 0u64;
    let mut comment = 0u64;
    let mut code = 0u64;
    let mut state = State::Code;

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            blank += 1;
            continue;
        }

        match state {
            State::InBlockComment => {
                comment += 1;
                if let Some((_, close)) = lang.block_comment {
                    if line.contains(close) {
                        state = State::Code;
                    }
                }
            }

            State::InMultilineString(idx) => {
                code += 1;
                let delim = lang.multiline_string[idx];
                // Count open delimiters on this line; odd count means still open
                let count = line.matches(delim).count();
                if count % 2 != 0 {
                    // Closed (toggled back to Code)
                    state = State::Code;
                }
            }

            State::Code => {
                // Shebang lines are always code regardless of comment syntax
                if line.starts_with("#!") {
                    code += 1;
                    continue;
                }

                // 1. Line comment prefix at start
                let matched_line_comment = lang
                    .line_comment
                    .iter()
                    .any(|prefix| line.starts_with(prefix));
                if matched_line_comment {
                    comment += 1;
                    continue;
                }

                // 2. Block comment opens at start of line
                if let Some((open, close)) = lang.block_comment {
                    if line.starts_with(open) {
                        comment += 1;
                        // Check if it also closes on the same line
                        let tail = &line[open.len()..];
                        if !tail.contains(close) {
                            state = State::InBlockComment;
                        }
                        continue;
                    }
                }

                // 3. Block comment opens mid-line (code precedes it)
                if let Some((open, close)) = lang.block_comment {
                    if let Some(pos) = line.find(open) {
                        if pos > 0 {
                            // Code content before the comment
                            code += 1;
                            let tail = &line[pos + open.len()..];
                            if !tail.contains(close) {
                                state = State::InBlockComment;
                            }
                            continue;
                        }
                    }
                }

                // 4. Multiline string opens
                let mut ml_matched = false;
                for (idx, delim) in lang.multiline_string.iter().enumerate() {
                    if line.contains(delim) {
                        code += 1;
                        let count = line.matches(delim).count();
                        if count % 2 != 0 {
                            state = State::InMultilineString(idx);
                        }
                        ml_matched = true;
                        break;
                    }
                }
                if ml_matched {
                    continue;
                }

                // 5. Default: code
                code += 1;
            }
        }
    }

    (blank, comment, code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::lang_by_extension;

    fn py() -> &'static LanguageDef { lang_by_extension("py").unwrap() }
    fn rs() -> &'static LanguageDef { lang_by_extension("rs").unwrap() }
    fn c()  -> &'static LanguageDef { lang_by_extension("c").unwrap() }
    fn sh() -> &'static LanguageDef { lang_by_extension("sh").unwrap() }

    #[test]
    fn python_basic() {
        let src = r#"
def foo():
    # a comment
    """
    docstring
    """
    return 1

x = 1
"#;
        let (blank, comment, code) = count_content(src, py());
        assert_eq!(blank, 2);
        assert_eq!(comment, 1);
        // def, return, x=1, + docstring lines count as code (multiline string)
        assert!(code >= 3);
    }

    #[test]
    fn rust_line_and_block_comments() {
        let src = "// line comment\nfn main() {\n    /* block */\n    let x = 1;\n}\n";
        let (blank, comment, code) = count_content(src, rs());
        assert_eq!(blank, 0);
        assert_eq!(comment, 2); // line comment + block comment line
        assert_eq!(code, 3);    // fn main, let x, closing brace
    }

    #[test]
    fn c_multiline_block_comment() {
        let src = "int a;\n/*\n * block\n * comment\n */\nint b;\n";
        let (blank, comment, code) = count_content(src, c());
        assert_eq!(blank, 0);
        assert_eq!(comment, 4); // /*, * block, * comment, */
        assert_eq!(code, 2);    // int a, int b
    }

    #[test]
    fn shell_shebang_and_comments() {
        let src = "#!/bin/bash\n# a comment\n\necho hello\n";
        let (blank, comment, code) = count_content(src, sh());
        assert_eq!(blank, 1);
        assert_eq!(comment, 1); // only # a comment; shebang is code
        assert_eq!(code, 2);    // shebang line + echo hello
    }

    #[test]
    fn blank_only_file() {
        let src = "\n\n\n";
        let (blank, comment, code) = count_content(src, rs());
        assert_eq!(blank, 3);
        assert_eq!(comment, 0);
        assert_eq!(code, 0);
    }

    #[test]
    fn inline_block_comment_is_code() {
        // Code precedes the block comment opener → code line
        let src = "foo(); /* comment */\n";
        let (_, comment, code) = count_content(src, rs());
        assert_eq!(code, 1);
        assert_eq!(comment, 0);
    }

    #[test]
    fn single_line_block_comment_is_comment() {
        let src = "/* note */\n";
        let (_, comment, code) = count_content(src, rs());
        assert_eq!(comment, 1);
        assert_eq!(code, 0);
    }
}
