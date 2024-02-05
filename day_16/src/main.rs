use reader::read_lines;
use transformer::get_map;

use crate::process::process_part_one;


mod reader;
mod transformer;
mod component;
mod process;

fn main() {
    let input_path = "./input/calibration.txt";

    let content = read_lines(input_path);

    let array = get_map(content.as_str());

    process_part_one(&array);

    println!("{:#?}",array);
}
