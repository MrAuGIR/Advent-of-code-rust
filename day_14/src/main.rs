use reader::read_lines;
use transformer::transforme_entries;

use crate::process::tilt_north;

mod reader;
mod transformer;
mod process;

fn main() {
    let input_path = String::from("./input/data.txt");

    let mut map: Vec<Vec<char>> = Vec::new();
    

    if let Ok(lines) = read_lines(input_path) {
        transforme_entries(lines,&mut map);

        // calcul move position
        let count = tilt_north(&mut map);

        println!("{:?}",count);
    }
}
