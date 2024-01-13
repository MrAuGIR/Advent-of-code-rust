use crate::component::Pattern;



pub fn analyze_pattern(pattern: &mut  Pattern) -> usize {

    let mut count = 0usize;

    count += analyse_horizontal_reflection(pattern.lines.clone());
    count += analyze_vertical_reflection(pattern.lines.clone());

    return count;
}

pub fn analyse_horizontal_reflection(lines: Vec<Vec<char>>) -> usize {
    let mut lines = lines.clone();
    let nb_row = lines.len();
    let lines_tmp = lines.clone();
    let mut is_reflection = false;

    for (index,line) in lines.iter_mut().enumerate() {

        let line_str: String = line.iter().collect();

        if index < nb_row - 1 {

            let next_line = index + 1;
            let next_line_str: String = lines_tmp.clone().get(next_line).unwrap().iter().collect();


            // comparaison
            if line_str == next_line_str {
                
                is_reflection = true;

                // si deux lignes correspondes on compare les lignes autour
                // on calcul la plage 
                let offset = (nb_row - next_line).min(index);

                for s in 1..=offset {

                    // on verifie que les index sont bien dans la plage 
                    let index_down = next_line + s;
                    let index_up = index - s;

                    if (0..nb_row).contains(&index_down) && (0..nb_row).contains(&index_up) {

                        let line_up: String = lines_tmp.get(index_up).unwrap().iter().collect();
                        let line_down: String = lines_tmp.get(index_down).unwrap().iter().collect();

                        if line_up != line_down {
                            is_reflection = false;
                            break;
                        }

                    } else {
                        break;
                    }

                }


                if is_reflection {
                    println!("is reflection ligne n°{:?} <> ligne n°{:?}",index, next_line);
                    return 100 * (index + 1usize);
                }
                
            }

        }

    }

    0
}


pub fn analyze_vertical_reflection(lines: Vec<Vec<char>>) -> usize {
    
    let nb_col = lines.get(0).unwrap().len();
    
    let mut is_reflection = false;


    for col_index in 0..nb_col - 1 {

        let next_col = col_index + 1;

        // on recupère la premiere colonne
        let col_a = get_col_to_str(lines.clone(), col_index);

        // on récupère la second colonne
        let col_b = get_col_to_str(lines.clone(), next_col);

        if col_a == col_b {
            is_reflection = true;

            let offset = col_index;

            for s in 1..offset {
                let index_right = next_col + s;
                let index_left = col_index - s;

                if (0..nb_col).contains(&index_right) && (0..nb_col).contains(&index_left) {

                    let col_a = get_col_to_str(lines.clone(), index_right);
                    let col_b = get_col_to_str(lines.clone(), index_left);

                    if col_a != col_b {
                        is_reflection = false;
                        break;
                    } else {
                        break;
                    }

                } else {

                    break;
                }
            }

            if is_reflection {
                println!("is reflection col n°{:?} vv col n°{:?}",col_index, col_index + 1);
                return col_index + 1usize;
            }
        }

    }


    return 0;
}

pub fn get_col_to_str(lines: Vec<Vec<char>>,col_index: usize) -> String {

    let mut column_str = String::new();

    for row_index in 0..lines.len() {
        
        let caractere = lines.get(row_index).unwrap().get(col_index).unwrap();
        
        column_str.push(caractere.clone());
    }
    column_str
}