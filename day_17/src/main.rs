use process::parcours_a_star;
use reader::read_lines;

use crate::transformer::{get_map_bloc, init_neighbors};



mod reader;
mod transformer;
mod component;
mod process;

fn main() {
    let input_path = "./input/calibration.txt";

    let content = read_lines(input_path);

    let mut map  = get_map_bloc(content.as_str());

    init_neighbors(&mut map);

    let start = map.get(0, 0).unwrap();
    let (max_x, max_y) = (map.num_columns(), map.num_rows());
    let end = map.get(max_y - 1, max_x - 1).unwrap();

    
    let travel = parcours_a_star(&map, start.clone(), end.clone());

    println!("{:?}",travel);
}
