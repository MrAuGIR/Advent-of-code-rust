
use std::fs::File;
use std::io::{self, Lines};

use crate::components::History;

pub fn create_history<'a>(lines: Lines<io::BufReader<File>>,histories: &mut Vec<History>) {

    for line in lines {

        if let Ok(line_content) = line {

            let sequence: Vec<i32> = line_content
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap() )
                .collect();

            let history = History::new(sequence);
            histories.push(history);
        }

    }

}