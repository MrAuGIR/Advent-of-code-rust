
#[derive(Debug)]
pub struct History
{
    pub sequences: Vec<Vec<i32>>
}

impl History {
    
    pub fn new(sequence: Vec<i32>) -> History {
        let mut sequences: Vec<Vec<i32>> = Vec::new();

        sequences.push(sequence);

        return History {
            sequences: sequences,
        };
    }

    pub fn make_sequences(&mut self) {
        let mut is_last_sequence = false;

        let mut current_sequence = self.sequences.first().unwrap().clone();

        while is_last_sequence != true {

            let (is_last, sequence) = self.create_sequences(&current_sequence);
    
            current_sequence = sequence.clone();
            
            is_last_sequence = is_last;
    
            self.sequences.push(sequence.clone());
        }
    }

    pub fn create_sequences(&mut self,sequence: &Vec<i32>) -> (bool,Vec<i32>) {

        let mut new_sequence = Vec::new();
        let mut is_last_sequence = true;

        for (index,i) in sequence.iter().enumerate() {
    
            if index == 0 {
                continue;
            }
    
            let value = i - sequence.get(index - 1).unwrap();
    
            if value != 0i32 {
                is_last_sequence = false;
            }
    
            new_sequence.push(value);
        }
    
        if is_last_sequence {
            new_sequence.push(0i32);
        }
    
        return (is_last_sequence, new_sequence);
    }

    pub fn calcul_last_value(&mut self) {

        let mut last_sequence: &Vec<i32> = &Vec::new();
    
        for (index,sequence) in self.sequences.iter_mut().rev().enumerate() {
            
            if index == 0 {
                last_sequence = sequence;
                continue;
            }
            let value = sequence.last().unwrap() + last_sequence.last().unwrap();
    
            sequence.push(value);
            last_sequence = sequence;
        }
    
    }

    pub fn calcul_first_value(&mut self) {

        let mut last_sequence: &Vec<i32> = &Vec::new();

        for (index, sequence) in self.sequences.iter_mut().rev().enumerate() {

            if index == 0 {
                sequence.insert(0, 0i32);
                last_sequence = sequence;
                continue;
            }
            let value = sequence.first().unwrap()  - last_sequence.first().unwrap();

            sequence.insert(0, value);
            last_sequence = sequence;
            
        }

    }

    pub fn get_last_value(&self) -> i32 {
        return *self.sequences.first().unwrap().last().unwrap();
    }

    pub fn get_first_value(&self) -> i32 {
        return *self.sequences.first().unwrap().first().unwrap();
    }
}