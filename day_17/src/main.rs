use reader::read_lines;
use transformer::get_map;

use crate::{component::{Bloc, Direction}, transformer::get_map_bloc};


mod reader;
mod transformer;
mod component;
mod process;

fn main() {
    let input_path = "./input/data.txt";

    let content = read_lines(input_path);

    let map  = get_map_bloc(content.as_str());

    let start = map.get(0, 0);

    println!("{:?}",start);

    println!("{:?}",map);
}
