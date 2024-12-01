use std::fs::File;
use std::io::{self, Lines};

use crate::component::Sequence;

pub fn transforme_entries(lines: Lines<io::BufReader<File>>,sequences: &mut Vec<Sequence>) {

    for (_, line) in  lines.enumerate() {

        if let Ok(line) = line {

            
            let mut data = line.split_whitespace();
            let mut work_data = data.clone();

            let mut spring_x5 = String::new();
            let spring_record = work_data.next().unwrap().to_string();

            let mut groups_x5: Vec<usize> = Vec::new();

            let groups: Vec<usize> = data.nth(1).unwrap().split(',').map(|s| s.trim().parse().unwrap()).collect();

            for i in 0..5 {

                spring_x5.push_str(&spring_record);
                if i != 4 {
                    spring_x5.push('?');
                }
                groups_x5.extend(groups.iter().cloned());
            }

            
            let sequence = Sequence::new(
                spring_x5,
                groups_x5
            );

            sequences.push(sequence);
        }
    }
}