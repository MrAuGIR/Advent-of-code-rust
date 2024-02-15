#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug,Clone,Eq, PartialEq, Hash)]
pub struct Bloc {
    pub x: usize,
    pub y: usize,
    pub lost: u32,
    pub color: String,
    pub neighbors: Vec<(usize,usize)>
}

impl Bloc {
    
    pub fn new(x: usize, y:usize, lost: u32) -> Bloc {
        Bloc {
            x,
            y,
            lost,
            color: "WHITE".to_string(),
            neighbors: Vec::new()
        }
    }
}