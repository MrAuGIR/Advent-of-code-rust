use std::collections::{BinaryHeap, HashMap};

use array2d::Array2D;

use crate::component::Bloc;


 pub fn parcours_a_star(map: &Array2D<Bloc>, start: Bloc, end: Bloc) -> Option<Vec<Bloc>> {
    let mut start = start.clone();
    let mut queue: BinaryHeap<Bloc> = BinaryHeap::new();

    let travel_from: HashMap<Bloc, Bloc> = HashMap::new();
    let mut g_scores: HashMap<Bloc, usize> = HashMap::new();

    start.h_score = 0; // score heuristique du départ

    queue.push(start.clone()); // ajout a la file

    g_scores.insert(start, 0);

    while let Some(current_bloc) = queue.pop() {

        // si on est au point d'arrivé
        if current_bloc == end {
            return Some(build_path(travel_from, end));
        }

        // on parcours les voisins
        for neighbor in current_bloc.neighbors {
            if let Some(neighbor) = map.get(neighbor.1, neighbor.0) {

                let mut neighbor = neighbor.clone();
                let tentative_g_score = g_scores[&neighbor] + 1; // mouvement supplementaire dans la grille

                // on verifie que le voisin n'a pas déjà été visité ou que le score de deplacement est inferieur au score enregistré
                if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {

                    g_scores.insert(neighbor.clone(), tentative_g_score);

                    
                    neighbor.h_score = calcul_heuristic(neighbor.clone(), end.clone());
                    // determine la priorité dans la file;
                    neighbor.f_score = tentative_g_score + neighbor.h_score;
                    

                    queue.push(neighbor.clone());
                }
            }

        }
    }
    None
 }

 fn build_path(travel_from: HashMap<Bloc,Bloc>,end: Bloc) -> Vec<Bloc> {
    let mut current = end;
    let mut path = vec![current.clone()];

    while let Some(prev) = travel_from.get(&current) {
        
        path.push(prev.clone());
        current = prev.clone();
    }

    path.reverse();
    path
 }

 fn calcul_heuristic(a: Bloc, b: Bloc) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
 }

/* 
 fn get_neighbors(bloc: Bloc, map: Array2D<u32>) -> Vec<Bloc> {

    let x_sub = bloc.x.checked_sub(1);
    let y_sub = bloc.y.checked_sub(1);
    let x_add = bloc.x.checked_add(1);
    let y_add = bloc.y.checked_add(1);

    let neighbor_left = x_sub.
        and_then(|x| map.get(bloc.y,x))
        .map(|u| Bloc {x: bloc.x - 1, y:bloc.y,lost:*u,color: "white".to_string()});

    let neighbor_top = y_sub
        .and_then(|y| map.get(y,bloc.x))
        .map(|u| Bloc {x: bloc.x, y: bloc.y - 1, lost: *u, color: "white".to_string()});

    let neighbor_right = map.get(bloc.x,bloc.y + 1).map(|u| Bloc {x: bloc.x, y: bloc.y +1,lost: *u, color: "White".to_string()});
 }*/