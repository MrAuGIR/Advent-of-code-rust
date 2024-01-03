use std::collections::HashSet;

use crate::component::Galaxy;



pub fn calcul_path_between_galaxies(g1: &Galaxy, g2: &Galaxy) -> usize {

    if g1.col == g2.col {
        // Même colonne, calculer la distance en termes de lignes
        g1.row.max(g2.row) - g1.row.min(g2.row)
    } else if g1.row == g2.row {
        // Même ligne, calculer la distance en termes de colonnes
        g1.col.max(g2.col) - g1.col.min(g2.col)
    } else {
        // Utiliser le théorème de Pythagore
        //let d1 = (g1.col as isize - g2.col as isize).abs() as usize;
        //let d2 = (g1.row as isize - g2.row as isize).abs() as usize;
        //let c = (d1.powi(2) + d2.powi(2)).sqrt();

        let delta_col = (g1.col as isize - g2.col as isize).abs() as usize;
        //println!("delta col : {:?}", delta_col);
        let delta_row = (g1.row as isize - g2.row as isize).abs() as usize;
        //println!("delta row : {:?}", delta_row);

        delta_col.max(delta_row) + delta_col.min(delta_row)
    }
}

pub fn explore_all_paires(map: &Vec<Galaxy>, paths_length: &mut Vec<usize>) {

    // les paires de galaxies parcourues
    let mut paires: HashSet<(usize, usize)> = HashSet::new();


    for (index, galaxy) in map.iter().enumerate() {

        for (j, galaxy2) in map.iter().enumerate() {

            if index == j {
                continue;
            }

            if index != j && !paires.contains(&(index, j)) && !paires.contains(&(j, index)) {
                
                let path = calcul_path_between_galaxies(galaxy, galaxy2);
                paths_length.push(path);
                println!("Chemin entre galaxie {} et galaxie {}: {:?}", index + 1, j + 1, path);

                // Ajoutez la paire à l'ensemble pour éviter de la traiter à nouveau
                paires.insert((index, j));
            }
        }

    }
}