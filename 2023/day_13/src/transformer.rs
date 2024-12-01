use std::fs::File;
use std::io::{self, Lines};

use crate::component::Pattern;


pub fn transforme_entries(lines: Lines<io::BufReader<File>>,patternes: &mut Vec<Pattern>) {

    let mut vec_lines: Vec<Vec<char>> = Vec::new();

    for line in  lines.flatten() {

        if line.is_empty() && !vec_lines.is_empty() {

            let pattern = Pattern::new(vec_lines);

            patternes.push(pattern);


            vec_lines = Vec::new();

        } else {
            
            vec_lines.push(line.chars().collect());
        }
    }

    if !vec_lines.is_empty() {
        patternes.push(
            Pattern::new(vec_lines)
        );
    }

}

/* let lines: Vec<Vec<char>> = lines
                .filter_map(|line| line.ok())
                .map(|line| line.chars().collect())
                .collect(); */
