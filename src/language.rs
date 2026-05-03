pub type LangName = &'static str;

#[derive(Debug, Clone)]
pub struct LanguageDef {
    pub name: LangName,
    pub extensions: &'static [&'static str],
    pub line_comment: &'static [&'static str],
    pub block_comment: Option<(&'static str, &'static str)>,
    pub multiline_string: &'static [&'static str],
}

pub static LANGUAGES: &[LanguageDef] = &[];

pub fn lang_by_extension(_ext: &str) -> Option<&'static LanguageDef> {
    None
}

pub fn lang_by_shebang(_first_line: &str) -> Option<&'static LanguageDef> {
    None
}
