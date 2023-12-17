mod reader;
mod transformers;
mod components;


use crate::reader::*;
use crate::transformers::*;

fn main() {

    let mut score = 1u32;

    let path_times = String::from("./input/data/times.txt");
    let mut times: Vec<u32> = Vec::new();

    if let Ok(times_str) = read_lines(path_times) {
        times = transform_line_times(times_str);
    }


    let path_distances = String::from("./input/data/distances.txt");
    let mut distances: Vec<u32> = Vec::new();

    if let Ok(distances_str) = read_lines(path_distances) {
        distances = transform_line_distance(distances_str);
    }

    let mut races = transform_race(times, distances);

    for race in &mut races {
        race.calculate_ways_to_win();
        score *= race.ways_to_win.len() as u32;
    }

    println!("scores {:#?}",score);
    
}