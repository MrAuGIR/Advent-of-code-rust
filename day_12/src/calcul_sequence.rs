

pub fn remplir_sequence_corrompue(sequence: &str, indices: &str) -> Vec<String> {
    let indices: Vec<usize> = indices.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let mut result = vec!["".to_string(); indices.len()];

    for (i, &len) in indices.iter().enumerate() {
        for j in 0..sequence.len() {
            if sequence[j..].starts_with(&"#".repeat(len)) {
                result[i] = format!("{}{}", &result[i], &sequence[j..j + len]);
            }
        }
    }

    // Génération de toutes les combinaisons possibles
    let mut combinations = vec![String::new(); 1];
    for group in result {
        let mut new_combinations = Vec::new();
        for combination in combinations {
            for c in &group.chars().collect::<Vec<_>>() {
                new_combinations.push(format!("{}{}", combination, c));
            }
        }
        combinations = new_combinations;
    }

    combinations
}