

pub fn remplir_sequence_corrompue(sequence: String, indices: &mut Vec<usize>) -> Vec<String> {
   

    // Génération de toutes les combinaisons possibles
    let mut combinations = vec![String::new(); 1];
    let mut sequence  = sequence.to_string();
   // premier caractère de la sequence 

   let first_char = sequence.chars().next();
    println!("caractere analysé {:?}",first_char);

   let group_length = indices.get(0).unwrap();

   match first_char {
       Some('.') => {
        sequence = sequence[1..].to_string();
        combinations.append(&mut remplir_sequence_corrompue(sequence, indices));
       },
       Some('?') => {
        let mut second_check_sequence = sequence.clone();
        // on check avec un '#'
        sequence.remove(0);
        sequence.insert(0, '#');
        combinations.append(&mut remplir_sequence_corrompue(sequence, indices));

        // on check avec un '.'
        second_check_sequence.remove(0);
        second_check_sequence.insert(0, '.');
        remplir_sequence_corrompue(second_check_sequence, indices);

       },
       Some('#') => {
        // on check si les n premier caractere sont des '#', n étabt le group_length en cours
        let echantillon = sequence.chars().take(group_length.clone()).collect::<String>();
        if echantillon == create_cmp_echantillon(group_length.clone()) {

            println!("lechantillon correspond au group en cours");
            combinations.push(echantillon);
            // on enleve l'échantillon de la sequence
            sequence = sequence.chars().skip(group_length.clone()).collect();
            // on supprime le premier group des indices
            indices.remove(0);
            combinations.append(&mut remplir_sequence_corrompue(sequence, indices));
        }

       }
       _ => {

       }
   }
   println!("{:?}", combinations);
    combinations
}

fn create_cmp_echantillon(length: usize) -> String {

    let echantillon = "#".repeat(length);

    return echantillon;
}