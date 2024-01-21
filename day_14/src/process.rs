use memoize::memoize;


#[memoize(Ignore: map)]
pub fn cycle(map:Vec<Vec<char>>, hash: String) -> Vec<Vec<char>> {

    let mut map = map;
    tilt_north(&mut map);
    //display_map(map);
    
    tilt_west(&mut map);
    //  display_map(map);

    tilt_south(&mut map);
    //  display_map(map);

    tilt_east(&mut map);
    //  display_map(map);
    return map;
}
   


pub fn tilt_north(map: &mut Vec<Vec<char>>) {

    let mut num_map: Vec<Vec<u16>> = Vec::new();
    let map_tmp = map.clone();

    for (i,line) in map_tmp.iter().enumerate() {

        let mut num_line: Vec<u16> = Vec::new();
        for (j, caractere) in line.iter().enumerate() {
            
            match caractere {
                '.' => {
                    // incremente de 1 la valeur sur la meme colonne de la ligne precedente
                    let mut prev_value = 0;
                    if i > 0 {
                        prev_value = num_map.get(i - 1).unwrap().get(j).unwrap().clone();
                    } 
                    num_line.push(prev_value + 1u16);
                },
                '#' => {
                    num_line.push(0u16);
                },
                'O' => {
                    let mut prev_value = 0;
                    if i > 0 {
                        prev_value = num_map.get(i - 1).unwrap().get(j).unwrap().clone();
                    } 

                    move_rounded_rock(map, (i,j), (i-prev_value as usize,j));

                    num_line.push(prev_value.clone())

                },
                _ => {
                    panic!("unknow caractere");
                }
            }
        }
        num_map.push(num_line);
    } 
}


pub fn tilt_west(map: &mut Vec<Vec<char>>) {
    
    let map_tmp = map.clone();
    
    for (i,line) in map_tmp.iter().enumerate() {

        let mut num_line: Vec<u16> = Vec::new();
        for (j, caractere) in line.iter().enumerate() {
            
            match caractere {
                '.' => {
                    // incremente de 1 la valeur sur la meme colonne de la ligne precedente
                    let mut prev_value = 0;
                    if  j > 0{
                        prev_value = num_line.get(j - 1).unwrap().clone();
                    } 
                    num_line.push(prev_value + 1u16);
                },
                '#' => {
                    num_line.push(0u16);
                },
                'O' => {
                    let mut prev_value = 0;
                    if j > 0 {
                        prev_value = num_line.get(j - 1).unwrap().clone();
                    } 

                    move_rounded_rock(map, (i,j), (i,j - prev_value as usize));
                    //calcul_amount_of_load(i-prev_value  as usize, &mut count, nb_line);

                    num_line.push(prev_value.clone());
                },
                _ => {
                    panic!("unknow caractere");
                }
            }
        }
    } 
}


pub fn tilt_south(map: &mut Vec<Vec<char>>) {
    let mut num_map: Vec<Vec<u16>> = Vec::new();
    let map_tmp = map.clone();
    let nb_lines = map.len();
    
    // on inverse le sens
    for (i,line) in map_tmp.iter().rev().enumerate() {

        let real_i = nb_lines - 1 - i; // real index
        
        let mut num_line: Vec<u16> = Vec::new();
        for (j, caractere) in line.iter().enumerate() {
            
            match caractere {
                '.' => {
                    // incremente de 1 la valeur sur la meme colonne de la ligne precedente
                    let mut prev_value = 0;
                    if i > 0 {
                        prev_value = num_map.get(i - 1).unwrap().get(j).unwrap().clone();
                    } 
                    num_line.push(prev_value + 1u16);
                },
                '#' => {
                    num_line.push(0u16);
                },
                'O' => {
                    let mut prev_value = 0;
                    if i > 0 {
                        prev_value = num_map.get(i - 1).unwrap().get(j).unwrap().clone();
                    } 

                    move_rounded_rock(map, (real_i,j), (real_i + prev_value as usize,j));

                    num_line.push(prev_value.clone());
                },
                _ => {
                    panic!("unknow caractere");
                }
            }
        }
        num_map.push(num_line);
    }
}


pub fn tilt_east(map: &mut Vec<Vec<char>>) {

    let map_tmp = map.clone();
    
    for (i,line) in map_tmp.iter().enumerate() {

        let mut num_line: Vec<u16> = Vec::new();
        let len_line = line.len();
        // on inverse
        for (j, caractere) in line.iter().rev().enumerate() {

            let real_j = len_line - 1 - j;
            
            match caractere {
                '.' => {
                    // incremente de 1 la valeur sur la meme colonne de la ligne precedente
                    let mut prev_value = 0;
                    if  j > 0{
                        prev_value = num_line.get(j - 1).unwrap().clone();
                    } 
                    num_line.push(prev_value + 1u16);
                },
                '#' => {
                    num_line.push(0u16);
                },
                'O' => {
                    let mut prev_value = 0;
                    if j > 0 {
                        prev_value = num_line.get(j - 1).unwrap().clone();
                    } 

                    move_rounded_rock(map, (i,real_j), (i,real_j + prev_value as usize));
                    //calcul_amount_of_load(i-prev_value  as usize, &mut count, nb_line);

                    num_line.push(prev_value.clone());
                },
                _ => {
                    panic!("unknow caractere");
                }
            }
        }
    } 
}

pub fn move_rounded_rock(map: &mut Vec<Vec<char>>,origin: (usize,usize), destination: (usize,usize)) {

    let (line_origin,col_origin_) = origin;
    let (line_dest, col_dest) = destination;

    *map.get_mut(line_origin).unwrap().get_mut(col_origin_).unwrap() = '.';
    *map.get_mut(line_dest).unwrap().get_mut(col_dest).unwrap() = 'O';
}


pub fn calcul_total_load(map: &Vec<Vec<char>>) -> usize {
    let mut count = 0usize;
    let nb_line = map.len();
    for (i,line) in map.iter().enumerate() {
        for (_,caractere) in line.iter().enumerate() {

            if *caractere == 'O' {
                calcul_amount_of_load(i, &mut count, nb_line);
            }
        }
    }
    count
}

pub fn calcul_amount_of_load(line: usize,count: &mut usize, nb_line: usize) {
    *count += nb_line - line
}

pub fn display_map(map: &Vec<Vec<char>>) {
    for line in map  {
        println!("___________");
        println!("{:?}",line);
        println!("___________");
    }
}