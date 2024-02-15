use std::vec;

use array2d::Array2D;

use crate::component::Bloc;



pub fn get_map(input: &str) -> Array2D<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();

    for row in rows {
        let vec_row: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
        array.push(vec_row);
    }

    Array2D::from_rows(&array).unwrap()
}

pub fn get_map_bloc(input: &str) -> Array2D<Bloc> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();

    for (y,row) in rows.iter().enumerate() {
        let vec_rows: Vec<Bloc> = row
        .chars()
        .enumerate()
        .map(|(x,c)| Bloc {x,y,lost:c.to_digit(10).unwrap(),color: "white".to_string()} ).collect();
        
        array.push(vec_rows);
    }

    Array2D::from_rows(&array).unwrap()
}