use crate::component::Pattern;



pub fn analyze_pattern(pattern: &mut  Pattern) -> usize {

    let mut count = 0usize;

    let (index_line, index_caractere) = fix_horizontal_reflection(&mut pattern.lines);

    if index_line != 99 {
        let line = pattern.lines.get_mut(index_line).unwrap();
        transform_caractere_line(line,index_caractere);
    }


    let count_vertical = analyze_vertical_reflection(pattern.lines.clone());
    let count_horizontal = analyze_horizontal_reflection(pattern.lines.clone());
    
    if count_horizontal != 0 && count_vertical != 0 {
        println!("il y a deux reflection");
    }

    count += count_horizontal.max(count_vertical);

    return count;
}

pub fn analyze_horizontal_reflection(lines: Vec<Vec<char>>) -> usize {
    let mut result = 0usize;
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
                        
                    }
                }


                if is_reflection {
                    println!("is reflection ligne n°{:?} <> ligne n°{:?}",index, next_line);
                    result =  100 * (index + 1usize);
                    break;
                }
                
            }

        }

    }

    result
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
            
            let offset = (nb_col - next_col).min(col_index);//col_index;

            for s in 1..=offset {
                let index_right = next_col + s;
                let index_left = col_index - s;

                if (0..nb_col).contains(&index_right) && (0..nb_col).contains(&index_left) {

                    let col_a = get_col_to_str(lines.clone(), index_right);
                    let col_b = get_col_to_str(lines.clone(), index_left);

                    if col_a != col_b {
                        is_reflection = false;
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


pub fn fix_horizontal_reflection(lines: &mut Vec<Vec<char>>) -> (usize, usize){

    
    let nb_row = lines.len();
    

    for (index, line) in lines.iter().enumerate() {

        let line_str:String = line.iter().collect();

        if index < nb_row - 1 {

            let next_line = index + 1;
            let next_line_str: String = lines.get(next_line).unwrap().iter().collect();

            if line_str != next_line_str {

              //  println!("les lignes {:?} et {:?} sont différentes",index, next_line);

                let (result,index_diff) = cmp_two_lines(line, lines.get(next_line).unwrap().clone());

                if result == true {
                   // println!("on fixe le caractere index {:?} ligne {:?}",index_diff, index);
                   return (index, index_diff);
                }
            } else if line_str == next_line_str {
                println!("les lignes {:?} et {:?} sont équivalente",index, next_line);
                 // si deux lignes correspondes on compare les lignes autour
                // on calcul la plage 
                let offset = (nb_row - next_line).min(index);

                for s in 1..=offset {

                    // on verifie que les index sont bien dans la plage 
                    let index_down = next_line + s;
                    let index_up = index - s;

                    if (0..nb_row).contains(&index_down) && (0..nb_row).contains(&index_up) {

                        let vec_line = lines.get(index_up).unwrap();
                        let line_up: String = vec_line.iter().collect();
                        
                        let vec_next_line = lines.get(index_down).unwrap();
                        let line_down: String = vec_next_line.iter().collect();

                        if line_up != line_down {
                            // on check si on peut corriger
                            println!("les lignes {:?} et {:?} sont différentes",index_up, index_down);

                            let (result,index_diff) = cmp_two_lines(vec_line, vec_next_line.clone());

                            if result == true {
                                println!("on fixe le caractere index {:?} ligne {:?}",index_diff, index_down);

                                return (index_down, index_diff);
                            }

                        }
                    } 
                }
            }
        }
    }
    return (99, 99);
}

pub fn cmp_two_lines(a: &Vec<char>, b:Vec<char>) -> (bool,usize) {


    let mut diff_count = 0;
    let mut memo_index = 0;

    for (i, (&c1,&c2)) in a.iter().zip(b.iter()).enumerate() {
        println!("i : {:?}, c1: {:?}, c2: {:?}",i,c1,c2);
        if c1 != c2 {
            memo_index = i;
            diff_count += 1;
        }
        println!("diff count {:?}", diff_count);
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