use reader::read_lines;
use transformer::get_map;

use crate::component::{Bloc, Direction};


mod reader;
mod transformer;
mod component;
mod process;

fn main() {
    let input_path = "./input/calibration.txt";

    let content = read_lines(input_path);

    let map  = get_map(content.as_str());

    let start = Bloc {x:0, y: 0, lost: map.get(0, 0),direction: Direction::Left};

    println!("{:?}",map);
}
