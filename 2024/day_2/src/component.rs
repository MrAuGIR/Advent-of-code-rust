use crate::checker::{get_diff,is_between_1_and_3,has_single_direction,with_remove_single_one};

#[derive(Clone,Debug)]
pub struct Report {
    pub levels: Level
}

#[derive(Clone,Debug)]
pub struct Level {
    pub values: Vec<u64>,
    pub is_safe: bool
}

impl Report {

    pub fn new(levels: Level) -> Report {
        Report {
            levels
        }
    }
}

impl Level {

    pub fn calcul_is_safe(&mut self) {
        
        let diff: Vec<u64> = get_diff(&self.values);

        let is_in_range = is_between_1_and_3(&diff);

        let has_one_direction = has_single_direction(&self.values);

        let with_remove_single = with_remove_single_one(&self.values);

        self.is_safe = (is_in_range && has_one_direction) || with_remove_single;

       // println!("{:?}  is in range {} and has one direction {}",self.values,is_in_range,has_one_direction)

    }
}