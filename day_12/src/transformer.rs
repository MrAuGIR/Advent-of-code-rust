use std::fs::File;
use std::io::{self, Lines};

use crate::component::{Sequence, Group};

pub fn transforme_entries(lines: Lines<io::BufReader<File>>,sequences: &mut Vec<Sequence>) {

    for (_, line) in  lines.enumerate() {

        if let Ok(line) = line {

            let mut groups_seq: Vec<Group> = Vec::new();

            let mut data = line.split_whitespace();
            let mut work_data = data.clone();

            
            let mut min_index = 0usize;
            let mut max_index = 0usize;
            let groups: Vec<usize> = data.nth(1).unwrap().split(',').map(|s| s.trim().parse().unwrap()).collect();

            // on parcours les groupes
            for (position,group) in groups.iter().enumerate() {
                max_index = max_index + group + 1usize;
                let group = Group::new(position, group.clone(), min_index, max_index);

                groups_seq.push(group);
                min_index = max_index;
            }

            let sequence = Sequence::new(
                work_data.next().unwrap().to_string(),
                work_data.next().unwrap().to_string(),
                 groups_seq
                );

            sequences.push(sequence);
        }
    }
}