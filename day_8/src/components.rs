
#[derive(Debug,Clone)]
pub struct Node {
    pub label: String,
    pub left: String,
    pub right: String
}

impl Node {

    pub fn new(label:String, left: String, right: String) -> Node {
        Node {
            label: label,
            left: left,
            right: right
        }
    }
}

pub struct InstructionIterator<'a> {
    instructions: &'a str,
    pub index: usize,
}

impl<'a> InstructionIterator<'a> {

    pub fn new(instructions: &'a str) -> Self {
        InstructionIterator{instructions, index: 0}
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instructions.is_empty() {
            return  None;
        }

        let currrent_char = self.instructions.chars().nth(self.index)?;
        self.index = (self.index + 1) % self.instructions.len();
        Some(currrent_char)
    }
}