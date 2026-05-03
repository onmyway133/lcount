use crate::stats::Report;

pub fn render(report: &Report) {
    println!("{}", serde_json::to_string_pretty(report).expect("JSON serialization failed"));
}
