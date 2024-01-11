

#[derive(Clone)]
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