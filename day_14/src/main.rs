use process::calcul_total_load;
use reader::read_lines;
use transformer::transforme_entries;

use crate::process::cycle;

mod reader;
mod transformer;
mod process;

fn main() {
    let input_path = String::from("./input/calibration.txt");

    let mut map: Vec<Vec<char>> = Vec::new();

    let nb_cycles: usize = 1000000000usize;
    

    if let Ok(lines) = read_lines(input_path) {
        transforme_entries(lines,&mut map);

        // calcul move position
        for i in 0..nb_cycles {
            println!("boucle {:?}",i);
            cycle(&mut map);
        }

        let count = calcul_total_load(&map);

        println!("{:?}",count);
    }
}
