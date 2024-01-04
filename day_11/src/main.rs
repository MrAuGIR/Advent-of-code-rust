use component::Galaxy;
use reader::read_lines;
use transformer::*;

use crate::path::explore_all_paires;

mod reader;
mod component;
mod transformer;
mod path;
fn main() {
    
    let input_path = "./input/data.txt".to_string();
    let output_path = "./input/extend_universe.txt".to_string();
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut paths_length: Vec<usize> = Vec::new();

    // nombre de ligne a duplique quand on a une row ou col sans galaxie
    let offset = 10; // ne pas utiliser le 1000000 cela crée un fichier trop volumineux

    // Etape 1 - extension de l'univers - les row
    if let Err(err) = expand_universe_raw(input_path.as_str(), output_path.as_str(),offset) {
        eprintln!("Erreur : {}", err);
    } else {
        println!("L'univers a été étendu avec succès !");
    }

    // Etape 2 - extension de l'univers - les colonnes
    if let Err(err) = expand_universe_col(&output_path.as_str(), output_path.as_str(), offset) {
        eprintln!("Erreur : {}", err);
    } else {
        println!("L'univers a été étendu avec succès !");
    }

    //  Etape 3 - lecture du fichier d'entrées univers étendu
    if let Ok(input_content) = read_lines(output_path) {

        galaxies_map_transformer(input_content, &mut galaxies);

        explore_all_paires(&galaxies, &mut paths_length)

    }

    let somme: usize = paths_length.iter().sum();
    println!("somme des chemins {:?}",somme );
    // println!("{:#?}",galaxies);
}
