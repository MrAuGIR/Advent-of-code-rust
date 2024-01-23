use std::fs::File;
use std::io::{self, Lines};

pub fn transforme_entries(lines: Lines<io::BufReader<File>>,map: &mut Vec<char>) {

    for (_,line) in lines.enumerate() {

        if let Ok(line) = line {

            let mut caracteres: Vec<char> = line.chars().collect();

            map.append(&mut caracteres);
        }
    }

}