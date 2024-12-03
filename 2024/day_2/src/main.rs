use component::Report;
use transformer::transform_line_to_report;

mod reader;
mod component;
mod transformer;

fn main() {
    let path = String::from("./input/input.txt");

    let mut reports: Vec<Report> = transform_line_to_report(&path);

    let mut count = 0;
    for (index,report) in reports.iter_mut().enumerate() {
        report.levels.calcul_is_safe();

        if (report.levels.is_safe == true) {
            count = count + 1;
        } else {
            println!("{:?} is NOT safe", report.levels.values);
        }
    }

    println!("report safe {:?}", count);
}
