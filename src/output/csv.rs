use crate::stats::Report;

pub fn render(report: &Report) {
    println!("language,files,blank,comment,code");
    for ls in &report.by_lang {
        println!("{},{},{},{},{}", ls.language, ls.files, ls.blank, ls.comment, ls.code);
    }
    println!(
        "{},{},{},{},{}",
        report.totals.language,
        report.totals.files,
        report.totals.blank,
        report.totals.comment,
        report.totals.code,
    );

    if let Some(files) = &report.by_file {
        println!();
        println!("file,language,blank,comment,code");
        for fs in files {
            println!(
                "{},{},{},{},{}",
                fs.path.display(),
                fs.language,
                fs.blank,
                fs.comment,
                fs.code,
            );
        }
    }
}
