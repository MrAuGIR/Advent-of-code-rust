mod reader;
mod transformers;
mod components;


use crate::reader::*;
use crate::transformers::*;

fn main() {
    let path = String::from("./input/calibration.txt");

    if let Ok(lines) = read_lines(path.to_owned()) {

        let mut game = transform_to_list(lines);

        // calcul des occurances dans la liste
        game.check_occurence_in_cards();

        // preparation des vec des card a copié pour chaque card
        game.prepare_copies_card();

        let scores = game.get_all_scores();
        // le reste est la loop pour faire le process
        // ...
        println!("score_final {:?}",scores.len());
    }
}
