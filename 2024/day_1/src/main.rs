mod reader;
mod transformer;
mod component;

use transformer::transform_line_to_lists;
use component::List;

fn main() {
    
    let input_path = String::from("./input/input.txt");

    let mut listes: Vec<List> = Vec::new();

    transform_line_to_lists(&input_path, &mut listes);

    let mut result: u64 = 0;

    let mut first_list = listes.get(0).unwrap().clone();
    first_list.sort_numbers();
    let mut second_list = listes.get(1).unwrap().clone();
    second_list.sort_numbers();

    let numbers_1 = first_list.get_numbers();
    let numbers_2 = second_list.get_numbers();

    for (index,number) in numbers_1.iter().enumerate() {

        let second_number = numbers_2.get(index).unwrap();
        let mut dif = number.max(second_number) - second_number.min(number);
        
        println!("index: {} -> substract {} - {}  = {} ",index,number, second_number, dif);
        result += dif;
        
    }
    println!("List {}",result);
}
