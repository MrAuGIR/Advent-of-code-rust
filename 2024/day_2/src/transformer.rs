use crate::component::{Report,Level};
use crate::reader::read_lines;
use std::fs::File;
use std::io::{self, Lines};

pub fn transform_line_to_report(path: &String) -> Vec<Report>  {
    let mut reports: Vec<Report> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        reports = create_reports(lines, &mut reports)
    }
    return reports;
}

fn create_reports(lines: Lines<io::BufReader<File>>, reports: &mut Vec<Report>) -> Vec<Report> {
    for line in lines {


        if let Ok(line) = line  {

            let data: Vec<u64> = line.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
            
            let mut report = Report::new(Level { values: data, is_safe: false});
            reports.push(report);
        }
        
        
    }
    return reports.clone();
}