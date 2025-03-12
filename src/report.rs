use crate::analysis::DuplicateSet;
use prettytable::{row, Table};

pub struct VersionSuggestion {
    pub recommended_version: String,
    pub estimated_saving: usize,
}

pub fn print_report(duplicates: Vec<DuplicateSet>, suggestions: Vec<VersionSuggestion> ){
    let mut table = Table::new();
    table.add_row(row!["Package", "Versions", "Recommended Version", "Estimated Savings (KB)"]);
    for (dup, sug) in duplicates.iter().zip(suggestions) {
        table.add_row(row![
            dup.name,
            dup.versions.join(", "),
            sug.recommended_version,
            format!("{}", sug.estimated_saving)
        ]);
    }
    table.printstd();
}