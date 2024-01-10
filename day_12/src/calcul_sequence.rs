

pub fn remplir_sequence_corrompue(sequence: String, indices: &mut Vec<usize>) -> usize {
    // Génération de toutes les combinaisons possibles
    let mut count = 0;
    let mut sequence = sequence;

  //  println!("sequence en cours {:?} ",sequence);
   // println!("indices en cours {:?}",indices);

    // Cas de base : la séquence est vide et il n'y a plus d'indices
    if sequence.is_empty() && indices.is_empty() {
        return 1;
    }

    // Cas où la séquence est vide mais il reste des indices
    if sequence.is_empty() && !indices.is_empty() {
        return 0;
    }

    // Cas où la séquence n'est pas vide mais il n'y a plus d'indices
    if indices.is_empty() {
      //  println!("indice vide reste sequence {:?}",sequence);
        return 1;
    }

    let group_length = indices[0];

    match sequence.chars().next() {
        Some('.') => {
            sequence = sequence[1..].to_string();
            count += remplir_sequence_corrompue(sequence, &mut indices.clone());
        },
        Some('?') => {
            let mut second_check_sequence = sequence.clone();
            
            // Option 1 : On remplace '?' par '#'
            sequence.remove(0);
            sequence.insert(0, '#');
            count += remplir_sequence_corrompue(sequence.clone(), &mut indices.clone());

            // Option 2 : On remplace '?' par '.'
            second_check_sequence.remove(0);
            second_check_sequence.insert(0, '.');
            count += remplir_sequence_corrompue(second_check_sequence, &mut indices.clone());
        },
        Some('#') => {
          //  println!("sequence avec un # de départ {:?}",sequence);
            // On vérifie si les n premiers caractères sont des '#', n étant le group_length en cours
            let echantillon = sequence.chars().take(group_length).collect::<String>();
            let caractere_next = sequence.clone().chars().nth(group_length).unwrap_or('-');
          //  println!("caractere suivant {:?}",caractere_next);
            if echantillon == create_cmp_echantillon(group_length) {
                if caractere_next != '#' || caractere_next == '-' {
                       // On enlève l'échantillon de la séquence, supprime le premier groupe des indices, et continue récursivement
                    sequence = sequence.chars().skip(group_length).collect();
                    //  println!("sequence apres un skip {:?}",sequence);
                    indices.remove(0);
  
                    if !sequence.is_empty() && sequence.chars().next().unwrap() == '?' {
                        sequence.remove(0);
                        sequence.insert(0, '.');
                        // println!("sequence apres avoir trouver un groupe {:?}",sequence);
                    }
                    count += remplir_sequence_corrompue(sequence, &mut indices.clone());
                }
             

                
            } else {
                
                for (index,caractere) in sequence.chars().enumerate() {

                    if caractere == '.' {
                        break;
                    }

                    if caractere == '#' {
                        continue;
                    }

                    if caractere == '?' {
                        let mut sequence = sequence.clone();
                        sequence.remove(index);
                        sequence.insert(index, '#');
                        count += remplir_sequence_corrompue(sequence, &mut indices.clone());
                        break;
                    }
                }
            }
        },
        _ => {}
    }

    count
}

fn create_cmp_echantillon(length: usize) -> String {

    let echantillon = "#".repeat(length);

    return echantillon;
}