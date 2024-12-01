

#[derive(Debug,Clone)]
pub struct Pattern {
    pub lines: Vec<Vec<char>>
}


impl Pattern {

    pub fn new(lines: Vec<Vec<char>>) -> Pattern {
        Pattern {
            lines
        }
    }
}