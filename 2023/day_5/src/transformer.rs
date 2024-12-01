
use std::fs::File;
use std::io::{self, Lines};

use crate::components::*;

pub fn transform_line_to_map<'a>(line_content:String) -> Result<Map, &'a str> {
    let data: Vec<&str> = line_content.split(" ").collect();

    let destination = data.get(0).expect("index 0 n'existe pas");
    let source = data.get(1).expect("index 1 n'existe pas");
    let length = data.get(2).expect("index 2 n'existe pas");

    return Map::new(destination, source, length);
}

pub fn transform_lines_to_mapper(lines: Lines<io::BufReader<File>>, index: usize, source:String, destination: String) -> Mapper {

    let seed: Category = Category{index: index ,name: source};
    let soil: Category = Category{index: index + 1usize ,name: destination};
    let mut mapper = Mapper::new(seed,soil);

    for line in lines {

        if let Ok(line_content) = line {

            let map = transform_line_to_map(line_content);
            mapper.push_maps(map.unwrap());

        }
    }

    return mapper;
}