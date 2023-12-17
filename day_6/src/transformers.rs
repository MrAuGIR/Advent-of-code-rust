
use std::fs::File;
use std::io::{self, Lines};

use crate::components::Race;

pub fn transform_line_distance(lines: Lines<io::BufReader<File>>) -> Vec<u32> {

    let mut distances: Vec<u32> = Vec::new();
    for line in lines {
        
        if let Ok(line) = line {

            distances = line.split_whitespace().map(|distance| {
                distance.parse::<u32>().expect("erreur de conversion")
            }).collect()
        }
    }

    distances
}

pub fn transform_line_times(lines: Lines<io::BufReader<File>>) -> Vec<u32> {

    let mut distances: Vec<u32> = Vec::new();
    for line in lines {
        
        if let Ok(line) = line {

            distances = line.split_whitespace().map(|distance| {
                distance.parse::<u32>().expect("erreur de conversion")
            }).collect()
        }
    }

    distances
}

pub fn transform_race(times: Vec<u32>, distances: Vec<u32>) -> Vec<Race> {

    let mut races: Vec<Race> = Vec::new();
    for (index, time) in times.iter().enumerate() {
        races.push(Race::new(time.clone(), distances.get(index).unwrap().clone()));
    }

    races
}