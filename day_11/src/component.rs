

#[derive(Debug, Clone)]
pub struct Galaxy{
    pub row: usize,
    pub col: usize,
}

impl Galaxy{

    pub fn new(row: usize, col: usize) -> Galaxy {
        Galaxy{
            row,col
        }
    }
}