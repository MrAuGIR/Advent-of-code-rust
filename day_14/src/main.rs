use process::calcul_total_load;
use reader::read_lines;
use transformer::transforme_entries;
use std::{collections::HashMap, clone};
use std::time::{Duration, Instant};

use crate::{process::cycle, hash::map_to_string};


mod reader;
mod transformer;
mod process;
mod hash;

fn main() {
    let start = Instant::now();

    let input_path = String::from("./input/calibration.txt");

    let mut map: Vec<Vec<char>> = Vec::new();

    let nb_cycles: usize =  100000usize;

    if let Ok(lines) = read_lines(input_path) {
        transforme_entries(lines,&mut map);

        
        // calcul move position
        for i in 0..nb_cycles {
            println!("boucle {:?}",i);
            
            let hash = map_to_string(map.clone());
            map = cycle(map.clone(),hash);

        }

        let count = calcul_total_load(&map);


        println!("{:?}",count);
        let duration = start.elapsed();
        println!("Time elapsed  is: {:?}", duration);
    }
}
