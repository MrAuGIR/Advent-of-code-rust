
#[derive(Debug, Clone)]
pub struct List {
   pub numbers: Vec<u64>
}

impl List {
    pub fn new() -> List {
      List {
         numbers: Vec::new()
      }
    }

    pub fn get_numbers(&self) -> &Vec<u64> {
      return &self.numbers;
    }

    pub fn sort_numbers(&mut self) {
      self.numbers.sort();
    }
}