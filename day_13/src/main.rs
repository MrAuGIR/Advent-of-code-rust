use component::Pattern;
use reader::read_lines;
use transformer::transforme_entries;

use crate::process::analyze_pattern;


mod component;
mod reader;
mod transformer;
mod process;

fn main() {
    
    let input_file = String::from("./input/data.txt");

    let mut patterns: Vec<Pattern> = Vec::new();

    let mut sum = 0usize;

    if let Ok(lines) = read_lines(input_file) {

        transforme_entries(lines, &mut patterns);
    }

    

    // analyse des pattern
    for (index,pattern) in patterns.iter_mut().enumerate() {
        println!("pattern n°{:?}",index);
     //   println!("{:#?}",pattern.lines);

        sum += analyze_pattern(pattern);
    }

    println!("{:?}",sum)

}
