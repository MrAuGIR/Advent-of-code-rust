use array2d::Array2D;



pub fn get_map(input: &str) -> Array2D<char> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();

    for row in rows {
        let vec_row: Vec<char> = row.chars().collect();
        array.push(vec_row);
    }

    Array2D::from_rows(&array).unwrap()
}