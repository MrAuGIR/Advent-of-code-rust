use crate::component::List;
use crate::reader::read_lines;
use std::fs::File;
use std::io::{self, Lines};


pub fn transform_line_to_lists(path: &String, listes: &mut Vec<List>) {

    if let Ok(lines) = read_lines(path) {
        create_listes(lines,listes);
    }
}

fn create_listes(lines: Lines<io::BufReader<File>>, listes: &mut Vec<List>) {
    for line in lines {


        if let Ok(line) = line {

            let data: Vec<&str> = line.split_whitespace().collect();

            for (index, value) in data.iter().enumerate() {
               
                if index >= listes.len() {
                    listes.push(List::new());
                }

                match value.trim().parse::<u64>() {
                    Ok(parsed_value) => listes[index].numbers.push(parsed_value),
                    Err(_e) => eprintln!("erreur de conversion {}", value)
                }
            }
        }
    }
}