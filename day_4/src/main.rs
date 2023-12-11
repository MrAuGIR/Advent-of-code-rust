mod reader;
mod transformers;
mod components;

use crate::reader::*;
use crate::transformers::*;

fn main() {
    let path = String::from("./input/data.txt");

    if let Ok(lines) = read_lines(path.to_owned()) {

        let mut game = transform_to_list(lines);

        game.check_occurence_in_cards();
        game.get_all_scores();
    }
}
