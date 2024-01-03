use std::fs::File;
use std::io::{self, Lines};

use crate::component::Galaxy;


pub fn galaxies_map_transformer(lines: Lines<io::BufReader<File>>,galaxies: &mut Vec<Galaxy>){

    for (row, line) in lines.enumerate() {

        if let Ok(line_content) = line {

            

            for (col, caractere) in  line_content.chars().enumerate() {

                match caractere {
                    '#' =>  {
                        let galaxy = Galaxy::new(row, col);
                        galaxies.push(galaxy);
                    },
                    _ => {

                    }
                }
            }
        }
    }
}