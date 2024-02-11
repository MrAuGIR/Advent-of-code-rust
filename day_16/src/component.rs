
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug,Clone,Eq, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub c: char,
    pub direction: Direction
}

#[derive(Debug,Clone,Eq,PartialEq, Hash)]
#[derive(Ord, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize
}