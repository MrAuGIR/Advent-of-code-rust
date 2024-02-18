use std::collections::{BinaryHeap, HashMap};

use array2d::Array2D;

use crate::{component::{Bloc, Direction}, dijkstra::{self, Dijkstra}};

pub fn solve(map: &Array2D<Bloc>, start: Bloc, end: Bloc) {


    let adjacency = |bloc: Bloc| -> Vec<Bloc> {
        let vec: Vec<Bloc> = if bloc.forward_count < 3 {
            let mut vec = Vec::new();
            if let Some(neighbor) = bloc.node_left() {
                vec.push(neighbor);
            }
            if let Some(neighbor) = bloc.node_right() {
                vec.push(neighbor);
            }
            if let Some(neighbor) = bloc.node_forward() {
                vec.push(neighbor);
            }
            let vec = vec.iter_mut().map(|c| map.get(c.1,c.0).unwrap().clone()).collect();
            vec
            
        } else {
            bloc.neighbors.iter().map(|c| map.get(c.1, c.0).unwrap().clone()).collect()
        };
        vec
    };

    let is_end = |bloc: Bloc| -> bool { bloc.forward_count >= 4 && bloc == end};

    
    let starts = vec![
        start
    ];

    let cost = |bloc: Bloc| bloc.score as usize;

    let dijkstra = Dijkstra::new(&adjacency, &cost, &is_end);

    dijkstra.cost(starts).unwrap().to_string()
}


 pub fn parcours_a_star(map: &Array2D<Bloc>, start: Bloc, end: Bloc) -> Option<Vec<Bloc>> {
    let mut start = start.clone();
    let mut queue: BinaryHeap<Bloc> = BinaryHeap::new();

    let mut travel_from: HashMap<Bloc, Bloc> = HashMap::new();
    let mut g_scores: HashMap<Bloc, usize> = HashMap::new();

    let mut current_step_in_a_direction = 0usize;
    let mut current_direction: Direction = Direction::None;

    start.h_score = 0; // score heuristique du départ

    queue.push(start.clone());
    
    let mut prev_bloc = start.clone(); // ajout a la file
    let mut prev_direction = Direction::None;

    g_scores.insert(start, 0);

    
    while let Some(current_bloc) = queue.pop() {

        //println!("current bloc (x:{:?}, y:{:?})",current_bloc.x,current_bloc.y);
        // si on est au point d'arrivé
        if current_bloc == end {
            return Some(build_path(travel_from, end));
        }

        let direction_parent = determine_direction(&current_bloc, &prev_bloc);

        println!("direction parent {:?} <=> prev direction {:?}",direction_parent, prev_direction);
        if direction_parent != prev_direction  {
            current_step_in_a_direction = 0;
        }

        // on parcours les voisins
        for neighbor in get_neighbors(&current_bloc) {
            if let Some(neighbor) = map.get(neighbor.1, neighbor.0) {

                let mut neighbor = neighbor.clone();
                let direction = determine_direction(&neighbor, &current_bloc);

                if direction ==  direction_parent && current_step_in_a_direction > 2 {
                   
                   continue;
                }
                    
                let tentative_g_score = match g_scores.get(&neighbor) {
                    Some(score) => *score + 1,
                    None => 0
                }; // mouvement supplementaire dans la grille

                // on verifie que le voisin n'a pas déjà été visité ou que le score de deplacement est inferieur au score enregistré
                if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {

                    g_scores.insert(neighbor.clone(), tentative_g_score);

                    
                    neighbor.h_score = calcul_heuristic(neighbor.clone(), end.clone());
                    // determine la priorité dans la file;
                    neighbor.f_score = tentative_g_score + neighbor.h_score;
                    
                    queue.push(neighbor.clone());

                    travel_from.insert(neighbor.clone(), current_bloc.clone());
                
                }
            }

        }
        prev_bloc = current_bloc;
        prev_direction = direction_parent;
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
    0
   // let distance =  ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize;
   // let heat_lost = a.score;

   // distance * heat_lost as usize
    //distance.checked_sub(a.score as usize).unwrap_or(0)
 }

 fn get_neighbors(parent: &Bloc) -> Vec<(usize,usize)> {
    parent.neighbors.clone()
 }

 pub fn display_travel(travel: &Vec<Bloc>, heat_lost: &mut usize){
    
    for bloc in travel {
        *heat_lost += bloc.score as usize;
        println!("(x:{:?}, y:{:?})",bloc.x,bloc.y)
    }
 }

 pub fn determine_direction(neighbor: &Bloc, current_bloc: &Bloc) -> Direction {

    if neighbor.x != current_bloc.x {
        if neighbor.x > current_bloc.x {
            return Direction::Right;
        } 
        return Direction::Left;
    } else {
        if neighbor.y > current_bloc.y {
            return Direction::Bottom;
        }
        return Direction::Top;
    }

 }

 pub fn update_info_direction(direction: &Direction, current_direction: &Direction,step_direction: &mut usize)
 {
    if direction == current_direction {
        *step_direction += 1;
    } else {
        *step_direction = 0;
    }
    
 }