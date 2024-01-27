
use reader::read_lines;

use crate::{process::calcul_hash, transformer::transforme_entries};

mod reader;
mod transformer;
mod process;

fn main() {
    let input_file = String::from("./input/data.txt");

    let mut caracteres: Vec<char> = Vec::new();

    let mut total_sum = 0usize;

    if let Ok(content) = read_lines(input_file) {

        transforme_entries(content, &mut caracteres);

        calcul_hash(&mut caracteres, &mut total_sum);
    }

    
    println!("{:?}",total_sum);
}
