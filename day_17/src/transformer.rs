
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
        .map(|(x,c)| Bloc::new(x,y,c.to_digit(10).unwrap()) ).collect();
        
        array.push(vec_rows);
    }

    Array2D::from_rows(&array).unwrap()
}

pub fn init_neighbors(map: &mut Array2D<Bloc>) {

    let (max_x, max_y) = (map.num_columns(), map.num_rows());

    for y in 0..max_y {
        for x in 0..max_x {
            let mut neighbors = Vec::new();

            if x > 0 {
                neighbors.push((x - 1, y));
            }
            
            if x < max_x - 1 {
                neighbors.push((x + 1, y));
            }
            
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            
            if y < max_y - 1 {
                neighbors.push((x, y + 1));
            }

            // y rows, x columns
            map[(y, x)].neighbors = neighbors;
        }
    }


}