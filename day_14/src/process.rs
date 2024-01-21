

pub fn cycle(map: &mut Vec<Vec<char>>) {

    tilt_north(map);
}


pub fn tilt_north(map: &mut Vec<Vec<char>>) -> usize {

    let mut num_map: Vec<Vec<u16>> = Vec::new();
    let map_tmp = map.clone();
    let mut count = 0;
    let nb_line = map.len();

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
                    calcul_amount_of_load(i-prev_value  as usize, &mut count, nb_line);

                    num_line.push(prev_value.clone())

                },
                _ => {
                    panic!("unknow caractere");
                }
            }
        }
        num_map.push(num_line);
    } 
    count
}

pub fn tilt_west(map: &mut Vec<Vec<char>>) {

}


pub fn move_rounded_rock(map: &mut Vec<Vec<char>>,origin: (usize,usize), destination: (usize,usize)) {

    let (line_origin,col_origin_) = origin;
    let (line_dest, col_dest) = destination;

    *map.get_mut(line_origin).unwrap().get_mut(col_origin_).unwrap() = '.';
    *map.get_mut(line_dest).unwrap().get_mut(col_dest).unwrap() = 'O';
}

pub fn calcul_amount_of_load(line: usize,count: &mut usize, nb_line: usize) {
    *count += nb_line - line
}