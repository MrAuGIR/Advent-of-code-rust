use array2d::Array2D;



pub fn get_map(input: &str) -> Array2D<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();

    for row in rows {
        let vec_row: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
        array.push(vec_row);
    }

    Array2D::from_rows(&array).unwrap()
}