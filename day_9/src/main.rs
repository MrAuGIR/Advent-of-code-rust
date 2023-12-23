mod reader;
mod components;
mod transformer;

use crate::components::History;
use crate::reader::*;
use crate::transformer::*;

fn main() {

    let mut histories: Vec<History> = Vec::new();

    let path_histories = String::from("./input/data.txt");

    if let Ok(lines) = read_lines(path_histories) {
        create_history(lines, &mut histories);
    }

    let mut counter_last = 0i32;
    let mut counter_first = 0i32;

    for history in histories.iter_mut() {
        history.make_sequences();
        history.calcul_last_value();
        history.calcul_first_value();

        counter_last += history.get_last_value();
        counter_first += history.get_first_value();
        //println!("{:?}",history.sequences);
    }

    println!("result : {:?}",counter_last); // resultat partie 1
    println!("result : {:?}",counter_first); // resultat partie 2
}
