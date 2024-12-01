use component::{Direction, Point};
use process::process_part_two;
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

    let start_point = Point { x: 0, y: 0, c: array.get(0, 0).unwrap().to_ascii_lowercase(), direction: Direction::Right };

    // part 1
    if let Some(result) = process_part_one(&array,start_point) {
        println!("{:?}",result);
    }

    // part 2 
    if let Some(result) = process_part_two(&array) {
        println!("{:?}",result);
    }   
}
