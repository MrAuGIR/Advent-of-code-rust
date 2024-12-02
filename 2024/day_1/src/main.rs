mod reader;
mod transformer;
mod component;

use std::collections::HashMap;

use transformer::transform_line_to_lists;
use component::List;

fn main() {
    
    let input_path = String::from("./input/input.txt");

    let mut listes: Vec<List> = Vec::new();

    transform_line_to_lists(&input_path, &mut listes);

    let mut result: u64 = 0;

    let mut occurances: HashMap<u64,u64> = HashMap::new();

    let mut first_list = listes.get(0).unwrap().clone();
    first_list.sort_numbers();
    let mut second_list = listes.get(1).unwrap().clone();
    second_list.sort_numbers();

    let numbers_1 = first_list.get_numbers();
    let numbers_2 = second_list.get_numbers();

    for (index,number) in numbers_1.iter().enumerate() {

        get_occurences(index as u64,number, numbers_2, &mut occurances);
    }

    for (key, value) in occurances.into_iter() {
         
        println!("{} = {} ",key,value);
        result += value;
    }

    println!("List {}",result);
}

fn get_occurences(search_index: u64,search: &u64, list: &Vec<u64>, occurances: &mut HashMap<u64,u64>)
{   
    let mut count: u64 = 0;
    for (index,number) in list.iter().enumerate() {

        if (search.eq(number)) {
            count = count + 1;
        }
    }
    occurances.insert(search_index.clone(), count * search);
}