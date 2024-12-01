use process::parcours_a_star;
use reader::read_lines;

use crate::{process::display_travel, transformer::{get_map_bloc, init_neighbors}, write::write_output_file};



mod reader;
mod transformer;
mod component;
mod process;
mod write;
mod dijkstra;

fn main() {
    let input_path = "./input/calibration.txt";

    let content = read_lines(input_path);

    let mut map  = get_map_bloc(content.as_str());

    init_neighbors(&mut map);

    let start = map.get(0, 0).unwrap();
    let (max_x, max_y) = (map.num_columns(), map.num_rows());
    let end = map.get(max_y - 1, max_x - 1).unwrap();

    let mut heat_lost = 0 as usize;

    
    if let Some(travel) = parcours_a_star(&map, start.clone(), end.clone()) {
        
        display_travel(&travel, &mut heat_lost);
        write_output_file("./output/calibration_result.txt".to_string(), &map, &travel);
    }

    println!("heat lost {:?}",heat_lost);

   
}


