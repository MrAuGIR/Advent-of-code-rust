use crate::component::Pattern;



pub fn analyze_pattern(pattern: &mut  Pattern) -> usize {

    
    let (index_line, index_caractere) = fix_horizontal_reflection(&mut pattern.lines);

    if index_line != 99 {
      
        println!("reflection horizontal a partir de la ligne {:?}",index_line);
        return 100 * (index_line + 1usize);
    }

    let (index_line, index_caractere) = fix_vertical_reflection(&mut pattern.lines);

    if index_line != 99 {
      
        println!("reflection vertical a partir de la colonne {:?}",index_caractere);
        return index_caractere + 1usize;
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


pub fn fix_horizontal_reflection(lines: &mut Vec<Vec<char>>) -> (usize, usize){

    let mut memo_diff_one = false;
    let mut memo_index_diff = 99;
    let nb_row = lines.len();
    

    for (index, line) in lines.iter().enumerate() {
        memo_diff_one = false;
        let line_str:String = line.iter().collect();

        if index < nb_row - 1 {

            let next_line = index + 1;
            let next_line_str: String = lines.get(next_line).unwrap().iter().collect();

            if line_str != next_line_str {

                let (result,index_diff) = cmp_two_lines(line, lines.get(next_line).unwrap().clone());
                memo_diff_one = true;    
                if result == true  {
                   println!("il y a un caractere different {:?} sur la ligne {:?}",index_diff, index);

                    //on check les autres lignes
                    let offset = (nb_row - next_line).min(index);

                    'check: for s in 1..=offset {

                        // on verifie que les index sont bien dans la plage 
                        let index_down = next_line + s;
                        let index_up = index - s;

                        if (0..nb_row).contains(&index_down) && (0..nb_row).contains(&index_up) {

                            let vec_line = lines.get(index_up).unwrap();
                            let line_up: String = vec_line.iter().collect();
                            
                            let vec_next_line = lines.get(index_down).unwrap();
                            let line_down: String = vec_next_line.iter().collect();

                            if line_up != line_down {
                                memo_diff_one = false;
                                break 'check;
                            }
                        } 
                    }
                    if memo_diff_one == true {
                        return (index,index_diff);
                    }
                }
            } else if line_str == next_line_str {
                
                 // si deux lignes correspondes on compare les lignes autour
                println!(" les deux ligne {:?} {:?} sont equivalent",index,next_line);
                // on calcul la plage 
                let offset = (nb_row - next_line).min(index);

                'check: for s in 1..=offset {

                    
                    // on verifie que les index sont bien dans la plage 
                    let index_down = next_line + s;
                    let index_up = index - s;
                    
                    println!("....... ligne {:?}", index_down);
                    println!("....... ligne {:?}", index_up);
                    if (0..nb_row).contains(&index_down) && (0..nb_row).contains(&index_up) {

                        let vec_line = lines.get(index_up).unwrap();
                        let line_up: String = vec_line.iter().collect();
                        
                        let vec_next_line = lines.get(index_down).unwrap();
                        let line_down: String = vec_next_line.iter().collect();

                        if line_up != line_down {
                            println!("...... les deux lignes sont differentes");
                            // on check si il y a une seule difference
                            if memo_diff_one == true {
                                println!("...... il y a plus de deux differences");
                                memo_diff_one = false;
                                break 'check;
                            }
                            
                            let (result,index_diff) = cmp_two_lines(vec_line, vec_next_line.clone());

                            if result == true {
                                println!("il y a une seule difference");
                                memo_diff_one = result;
                                memo_index_diff = index_diff;
                            }
                        }
                    } 
                }

                if memo_diff_one == true {
                    println!("...... il y a une seule differences");
                    return (index, memo_index_diff);
                }
            }
        }
    }
    return (99, 99);
}

pub fn fix_vertical_reflection(lines: &mut Vec<Vec<char>>) -> (usize, usize) {

    let nb_col = lines.get(0).unwrap().len();

    let mut memo_diff_one = false;
    let mut memo_index_diff = 99;

    for col_index in 0..nb_col - 1 {

        memo_diff_one = false;

        let next_col = col_index + 1;

        // on recupère la premiere colonne
        let col_a = get_col_to_str(lines.clone(), col_index);

        // on récupère la second colonne
        let col_b = get_col_to_str(lines.clone(), next_col);

        if col_a == col_b {
            println!("les colonnes {:?} et {:?} sont équivalent",col_index,next_col);
            let offset = (nb_col - next_col).min(col_index);//col_index;
            
            'check: for s in 1..=offset {

                let index_right = next_col + s;
                let index_left = col_index - s;

                println!(".........colonne {:?} et {:?}",index_left, index_right);

                if (0..nb_col).contains(&index_right) && (0..nb_col).contains(&index_left) {

                    let col_a = get_col_to_str(lines.clone(), index_right);
                    let col_b = get_col_to_str(lines.clone(), index_left);

                    
                    if col_a != col_b {
                        println!(".........les colonnes ne sont pas équivalentes");
                        if memo_diff_one == true {
                            memo_diff_one = false;
                            memo_index_diff = 99;
                            break 'check;
                        }

                        // on cherche a fixer
                        // on check si on peut corriger
                        
                        let vec_col_a = get_col_to_vec(lines.clone(), index_left);
                        let vec_col_b = get_col_to_vec(lines.clone(), index_right);

                        let (result,index_diff) = cmp_two_lines(&vec_col_a, vec_col_b.clone());

                        
                        if result == true {
                            memo_index_diff = index_diff;
                            memo_diff_one = result;
                        }
                    }else {
                        println!("dep mirror -> colonne {:?} et {:?} sont équivalente", index_left,index_right);
                    }

                } 
            }

            if memo_diff_one == true {
                return (memo_index_diff,col_index);
            }

        } else {

            let vec_col_a = get_col_to_vec(lines.clone(), col_index);
            let vec_col_b = get_col_to_vec(lines.clone(), next_col);

            let (result,index_diff) = cmp_two_lines(&vec_col_a, vec_col_b.clone());

            if result == true {
                memo_diff_one = result;
                // si colonne 0 ou dernière colonne
                if col_index == 0 || next_col == (nb_col - 1) {
                    return (index_diff, col_index);
                }

                let offset = (nb_col - next_col).min(col_index);//col_index;
            
                'check: for s in 1..=offset {
                    
                    let index_right = next_col + s;
                    let index_left = col_index - s;

                    if (0..nb_col).contains(&index_right) && (0..nb_col).contains(&index_left) {

                        let col_a = get_col_to_str(lines.clone(), index_right);
                        let col_b = get_col_to_str(lines.clone(), index_left);

                        
                        if col_a != col_b {
                            memo_diff_one = false;
                            break 'check;
                            
                        }
                    } 
                }
                if memo_diff_one == true {
                    return (index_diff, col_index);
                }
            }
        }
    }

    return (99,99);
} 

pub fn cmp_two_lines(a: &Vec<char>, b:Vec<char>) -> (bool,usize) {


    let mut diff_count = 0;
    let mut memo_index = 0;

    for (i, (&c1,&c2)) in a.iter().zip(b.iter()).enumerate() {
        
        if c1 != c2 {
            memo_index = i;
            diff_count += 1;
        }
        
        if diff_count > 1 {
            return (false, memo_index);
        }
    }

    if diff_count == 1 {
        println!("il n'y a qu'une difference on retourne true et l'index");
        return (true,memo_index);
    }
    (false,memo_index)
}

pub fn transform_caractere_line(line: &mut Vec<char>, index_diff:usize){
    if let Some(caractere ) = line.get_mut(index_diff) {

        match caractere {
            '.' => {
                *caractere = '#';
            },
            '#' => {
                *caractere = '.';
            },
            _ => {
                panic!("caractere étrange !")
            }
        }
    }
}

pub fn get_col_to_vec(lines: Vec<Vec<char>>, col_index: usize) -> Vec<char> {

    let mut column: Vec<char> = Vec::new();

    for row_index in 0..lines.len() {
        
        let caractere = lines.get(row_index).unwrap().get(col_index).unwrap();
        
        column.push(caractere.clone());
    }
    column
}