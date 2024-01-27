

pub fn calcul_hash(hash_line: &mut Vec<char>, total_sum: &mut usize) {

    let mut current_value = 0usize;
    let mut sequence = String::new();


    for letter in hash_line.iter() {
        let ascii = letter.to_string().as_bytes().get(0).unwrap().clone() as usize;
        match letter {
            ',' => {
                println!("sequence {:?} : {:?}",sequence,current_value);
                sequence = String::new();
                *total_sum += current_value;
                current_value = 0;
            },
            _ => {
               // println!("caractere {:?} value {:?}",letter,ascii);
                sequence.push(letter.clone());
                cycle_calcul_hash(ascii, &mut current_value)
            }
        }
    }

    if current_value > 0 {
        println!("sequence {:?} : {:?}",sequence,current_value);
        *total_sum += current_value;
    }
}


pub fn cycle_calcul_hash(ascii_value: usize, current_value: &mut usize) {
    *current_value += ascii_value;

    *current_value *= 17usize;

    *current_value %= 256;
}