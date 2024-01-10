use std::fs::File;
use std::io::{self, Lines};

use crate::component::Sequence;

pub fn transforme_entries(lines: Lines<io::BufReader<File>>,sequences: &mut Vec<Sequence>) {

    for (_, line) in  lines.enumerate() {

        if let Ok(line) = line {

            
            let mut data = line.split_whitespace();
            let mut work_data = data.clone();

            let groups: Vec<usize> = data.nth(1).unwrap().split(',').map(|s| s.trim().parse().unwrap()).collect();

            
            let sequence = Sequence::new(
                work_data.next().unwrap().to_string(),
                 groups
            );

            sequences.push(sequence);
        }
    }
}