use std::collections::HashMap;



pub fn calcul_hash(hash_line: &mut Vec<char>, total_sum: &mut usize) {

    let mut hash_box: HashMap<usize, Vec<(String,usize)>> = HashMap::new();
    let mut current_value = 0usize;
    let mut sequence = String::new();


    for (index,letter) in hash_line.iter().enumerate() {
        let ascii = letter.to_string().as_bytes().get(0).unwrap().clone() as usize;
        match letter {
            '=' => {
                println!("sequence {:?} : {:?}",sequence,current_value);

                if let Some(lenths) = hash_box.get_mut(&current_value) {
                    let mut update = false;
                    for (label,lens) in lenths.iter_mut() {
                        if *label == sequence {
                            
                            if let Some(caractere) = hash_line.get(index + 1) {
                                *lens = char_to_usize(caractere);
                            }
                            update = true;
                        }
                    }

                    if update == false {
                        
                        if let Some(lens) = hash_line.get(index + 1 ) {
                            let value = char_to_usize(lens);
                            lenths.push((sequence.clone(),value));
                        }

                    }


                } else {
                    let mut vec_lens: Vec<(String,usize)> = Vec::new();
                    if let Some(lens) =  hash_line.get( index + 1) {
                        let value = char_to_usize(lens);

                        vec_lens.push((sequence.clone(),value));
                    }
                    

                    hash_box.insert(current_value, vec_lens);
                }

                current_value = 0;
            },
            '-' => {
                println!("sequence {:?} : {:?}",sequence,current_value);
                
                if let Some(lenths) = hash_box.get_mut(&current_value) {
                    for (index,(label,_)) in lenths.iter().enumerate() {
                        if *label == sequence {
                            
                            lenths.remove(index);
                            break;
                        }
                    }
                }
                current_value = 0;
            },
            ',' => {
                sequence = String::new();
                current_value = 0;
            },
            _ => {
               // println!("caractere {:?} value {:?}",letter,ascii);
                sequence.push(letter.clone());
                cycle_calcul_hash(ascii, &mut current_value)
            }
        }
    }

    calcul_focusing_power(hash_box, total_sum);

    println!("{:?}",total_sum);
}


pub fn cycle_calcul_hash(ascii_value: usize, current_value: &mut usize) {
    *current_value += ascii_value;

    *current_value *= 17usize;

    *current_value %= 256;
}

fn char_to_usize(number: &char) -> usize {
    //println!("char to usize {:?}",number);
    number.to_string().parse::<usize>().unwrap()
}

fn calcul_focusing_power(boxes: HashMap<usize, Vec<(String,usize)>>, total_sum: &mut usize) {

    for (_,(index_box,slots)) in boxes.iter().enumerate() {
        calcul_power_box( index_box,slots,total_sum);
    }

}

fn calcul_power_box(index_box: &usize,slots: &Vec<(String,usize)>,total_sum: &mut usize) {

    for (index_slot,slot) in slots.iter().enumerate() {
        
        let value = (index_box + 1) * (index_slot + 1) * calcul_power_slot(slot);
        //println!("box {:?} * {:?} (slot) * {:?} (focal length) = {:?}",index_box + 1,index_slot + 1,slot.1,value);

        *total_sum += value;
    }
}

fn calcul_power_slot(slot: &(String,usize)) -> usize {
    slot.1
}