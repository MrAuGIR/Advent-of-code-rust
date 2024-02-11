use reader::read_lines;
use transformer::get_map;

use crate::process::process_part_one;


mod reader;
mod transformer;
mod component;
mod process;

fn main() {
    let input_path = "./input/data.txt";

    let content = read_lines(input_path);

    let array = get_map(content.as_str());

    if let Some(result) = process_part_one(&array) {
        println!("{:?}",result);
    }   
}
