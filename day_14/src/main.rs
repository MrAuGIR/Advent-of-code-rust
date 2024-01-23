use process::calcul_total_load;
use reader::read_lines;
use transformer::transforme_entries;
use std::time::Instant;

use crate::{process::{cycle, get_ref_caractere}, hash::map_to_string};


mod reader;
mod transformer;
mod process;
mod hash;

fn main() {
    let start = Instant::now();

    let input_path = String::from("./input/calibration.txt");

    let mut map: Vec<char> = Vec::new();

    let nb_cycles: usize =  1usize;

    let WIDTH = 10usize;
    let HEIGHT =10usize;

    if let Ok(lines) = read_lines(input_path) {
        transforme_entries(lines,&mut map);

        // calcul move position
        for i in 0..nb_cycles {
            println!("boucle {:?}",i);
            
            let hash = map_to_string(map.clone());
            cycle(&mut map,hash);
        }

        let count = calcul_total_load(&map);


        println!("{:?}",count);


        for i in 0..10 {

            let mut line:Vec<char> = Vec::new();
            for j in 0..10 {
                let chara = get_ref_caractere(&map,i,j);
                line.push(chara.clone());
            }
            println!("{:?}",line);
        }
        
      
        let duration = start.elapsed();
        println!("Time elapsed  is: {:?}", duration);
    }
}
