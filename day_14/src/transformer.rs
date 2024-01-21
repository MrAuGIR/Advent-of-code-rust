use std::fs::File;
use std::io::{self, Lines};

pub fn transforme_entries(lines: Lines<io::BufReader<File>>,map: &mut Vec<Vec<char>>) {

    for (_,line) in lines.enumerate() {

        if let Ok(line) = line {

            let caracteres: Vec<char> = line.chars().collect();

            map.push(caracteres);
        }
    }

}