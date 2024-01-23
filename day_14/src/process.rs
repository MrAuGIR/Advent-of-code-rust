use memoize::memoize;


#[memoize(Ignore: map)]
pub fn cycle(map:&mut Vec<char>, hash: String) {

    tilt_north(map);
    //display_map(map);
    
    tilt_west(map);
    //  display_map(map);

    tilt_south(map);
    //  display_map(map);

    tilt_east(map);
    //  display_map(map);
}
   


pub fn tilt_north(map: &mut Vec<char>) {

    let mut num_map: Vec<Vec<u16>> = Vec::new();
    
    for i in 0..10 {

        let mut num_line: Vec<u16> = Vec::new();
        for j in 0..10{

            let caractere = get_ref_caractere(map,i,j);
            
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


pub fn tilt_west(map: &mut Vec<char>) {
    
    for i in 0..10 {

        let mut num_line: Vec<u16> = Vec::new();
        for j in 0..10 {

            let caractere = get_ref_caractere(map,i,j);
            
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


pub fn tilt_south(map: &mut Vec<char>) {
    let mut num_map: Vec<Vec<u16>> = Vec::new();
    let nb_lines = 10;
    
    // on inverse le sens
    for i in 0..10 {

        let real_i = nb_lines - 1 - i; // real index
        
        let mut num_line: Vec<u16> = Vec::new();
        for j in 0..10 {

            
            let caractere = get_ref_caractere(map,real_i,j);
            
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


pub fn tilt_east(map: &mut Vec<char>) {

    for i in 0..10 {

        let mut num_line: Vec<u16> = Vec::new();
        let len_line = 10;
        // on inverse
        for j in 0..10 {

            let real_j = len_line - 1 - j;
            

            let caractere = get_ref_caractere(map,i,real_j);
            
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

pub fn move_rounded_rock(map: &mut Vec<char>,origin: (usize,usize), destination: (usize,usize)) {

    let (line_origin,col_origin) = origin;
    let (line_dest, col_dest) = destination;

    let index = line_origin * 10 + col_origin;

    *map.get_mut(index).unwrap() = '.';

    let index = line_dest * 10 + col_dest;
    *map.get_mut(index).unwrap() = 'O';
}


pub fn calcul_total_load(map: &Vec<char>) -> usize {
    let mut count = 0usize;
    let nb_line = map.len() / 10;
    
    for (i,caractere) in map.iter().enumerate() {
        
        if *caractere == 'O' {
            let (ligne, _) = div_rem(i, 10);
            
            calcul_amount_of_load(ligne, &mut count, nb_line);
        }
    }
    
    count
}

// Fonction utilitaire pour la division entière et le reste
fn div_rem(n: usize, divisor: usize) -> (usize, usize) {
    (n / divisor, n % divisor)
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

pub fn get_ref_caractere<'a>(map: &'a Vec<char>,i: usize,j: usize) -> &'a char {
    let index = i * 10 + j;
    return map.get(index).unwrap()
}