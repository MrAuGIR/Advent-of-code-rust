use component::Sequence;
use reader::read_lines;
use transformer::transforme_entries;

use crate::calcul_sequence::remplir_sequence_corrompue;

mod reader;
mod transformer;
mod component;
mod calcul_sequence;

fn main() {
    let input_path = "./input/calibration.txt".to_string();

    let mut sequences: Vec<Sequence> = Vec::new();

    let mut solutions = 0usize;

    if let Ok(content) = read_lines(input_path) {
        transforme_entries(content, &mut sequences);


        for sequence in &sequences {

            println!("sequence en cours {:?}",sequence);

            solutions += remplir_sequence_corrompue(sequence.spring_record.clone(), &mut sequence.groups.clone());
        }

        println!("{:?}", solutions);
    }

  //  println!("{:#?}", sequences);
}
