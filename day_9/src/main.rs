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

    let mut counter = 0i32;

    for history in histories.iter_mut() {
        history.make_sequences();
        history.calcul_last_value();

        counter += history.get_last_value();

        //println!("{:?}",history.sequences);
    }

    println!("result : {:?}",counter);
}
