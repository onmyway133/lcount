use comfy_table::{
    presets::UTF8_FULL, Attribute, Cell, CellAlignment, ContentArrangement, Table,
};
use crate::stats::{FileStats, LangStats, Report};

pub fn render(report: &Report) {
    println!("{}", lang_table(&report.by_lang, &report.totals));

    if let Some(files) = &report.by_file {
        println!();
        println!("{}", file_table(files));
    }
}

fn lang_table(by_lang: &[LangStats], totals: &LangStats) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            header_cell("Language"),
            header_cell("Files"),
            header_cell("Blank"),
            header_cell("Comment"),
            header_cell("Code"),
        ]);

    for ls in by_lang {
        table.add_row(vec![
            Cell::new(ls.language),
            num_cell(ls.files),
            num_cell(ls.blank),
            num_cell(ls.comment),
            num_cell(ls.code),
        ]);
    }

    table.add_row(vec![
        Cell::new("SUM").add_attribute(Attribute::Bold),
        num_cell(totals.files).add_attribute(Attribute::Bold),
        num_cell(totals.blank).add_attribute(Attribute::Bold),
        num_cell(totals.comment).add_attribute(Attribute::Bold),
        num_cell(totals.code).add_attribute(Attribute::Bold),
    ]);

    table
}

fn file_table(files: &[FileStats]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            header_cell("File"),
            header_cell("Language"),
            header_cell("Blank"),
            header_cell("Comment"),
            header_cell("Code"),
        ]);

    for fs in files {
        table.add_row(vec![
            Cell::new(fs.path.display()),
            Cell::new(fs.language),
            num_cell(fs.blank),
            num_cell(fs.comment),
            num_cell(fs.code),
        ]);
    }

    table
}

fn header_cell(s: &str) -> Cell {
    Cell::new(s).add_attribute(Attribute::Bold)
}

fn num_cell(n: u64) -> Cell {
    Cell::new(n).set_alignment(CellAlignment::Right)
}
