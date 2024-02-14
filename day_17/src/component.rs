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
    pub direction: Direction
}