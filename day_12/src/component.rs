

#[derive(Debug, Clone)]
pub struct Sequence{
    pub spring_record: String,
    pub groups: Vec<usize>,
}

impl Sequence {

    pub fn new(spring_record: String, groups: Vec<usize>) -> Sequence {
        Sequence{
            spring_record,
            groups,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group{
    pub position: usize,
    pub length: usize,
    pub min_index: usize,
    pub max_index: usize,
}

impl Group{
    
   pub fn new(position: usize, length: usize, min_index: usize, max_index: usize) -> Group {
    Group {
        position,
        length,
        min_index,
        max_index
    }
   }
}