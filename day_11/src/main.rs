use component::Galaxy;
use reader::read_lines;
use transformer::galaxies_map_transformer;

use crate::path::explore_all_paires;

mod reader;
mod component;
mod transformer;
mod path;
fn main() {
    
    let input_path = "./input/data.txt".to_string();
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut paths_length: Vec<usize> = Vec::new();

    // lecture du fichier d'entrées
    if let Ok(input_content) = read_lines(input_path) {

        galaxies_map_transformer(input_content, &mut galaxies);

        explore_all_paires(&galaxies, &mut paths_length)

    }

    let somme: usize = paths_length.iter().sum();
    println!("somme des chemins {:?}",somme );
    // println!("{:#?}",galaxies);
}
