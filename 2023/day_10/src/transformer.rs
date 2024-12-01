
use std::fs::File;
use std::io::{self, Lines};

use crate::component::{Node, Tiles, Map};

pub fn hydrate_map<'a>(lines: Lines<io::BufReader<File>>,map: &mut Map) {

    for (index_y,line) in lines.enumerate() {

        if let Ok(line_content) = line {

            let mut vec_line: Vec<Node> = Vec::new();

            for (index_x, c) in line_content.chars().enumerate() {

                let type_tile = char_to_tiles(c);
                let node = Node::new(index_x,index_y,type_tile);

               
                if node.tile == Tiles::Starting {
                    map.set_start_node(node.clone())
                }

                vec_line.push(node);
            }
            map.map.push(vec_line);
        }
    }
}

pub fn connect_neighbors(map: &mut Map) {

    let length = map.map.len();
    let tmp_map = map.clone();

    for (index, vec_nodes) in map.map.iter_mut().enumerate() {

        let length_x = vec_nodes.len();
        for (i,node) in vec_nodes.iter_mut().enumerate() {


            match node.tile {
                Tiles::Ground => continue,
                Tiles::Starting => {
                    let mut starting_neighbors: Vec<(usize,usize)> = Vec::new();
                    // voisin du point de départ
                    let mut new_x = node.x;
                    if new_x > 0 {
                        new_x -= 1;
                        if check_neighbors_starting_point((new_x, node.y),&tmp_map) {

                            starting_neighbors.push((new_x, node.y));
                        }
                    }

                    if node.x < (length_x - 1) {
                        new_x = node.x + 1;
                        if check_neighbors_starting_point((new_x, node.y),&tmp_map) {

                            starting_neighbors.push((new_x, node.y));
                        }
                    }

                    let mut new_y = node.y;
                    if new_y > 0 {
                        new_y -=1;
                        if check_neighbors_starting_point((node.x, new_y),&tmp_map) {

                            starting_neighbors.push((node.x, new_y));
                        }
                    }

                    if node.y < (length - 1) {
                        new_y = node.y + 1;
                        if check_neighbors_starting_point((node.x, new_y),&tmp_map) {

                            starting_neighbors.push((node.x, new_y));
                        }
                    }

                    node.neighbors = starting_neighbors;

                },
                _ => {
                    let neighbors = get_allowed_neighbors(node, length, length_x);

                    node.neighbors = neighbors;
                }
            }

        }
    }
}


pub fn get_allowed_neighbors(node: &Node, length: usize, length_x: usize) -> Vec<(usize, usize)> {

    let mut allowed_coordonne: Vec<(usize, usize)> = Vec::new();

    match node.tile {
        
        Tiles::VerticalPipe => {
            let mut new_y = node.y;
            if node.y > 0 {
                new_y -= 1;
            }
            let north = (node.x, new_y);

            let mut new_y = node.y;
            if node.y < (length - 1) {
                new_y += 1;
            }
            let south = (node.x, new_y);

            allowed_coordonne.push(north);
            allowed_coordonne.push(south);
            return allowed_coordonne;
        },
        Tiles::NorthToEast => {
            let mut new_y = node.y;
            if node.y > 0 {
                new_y -= 1;
            }
            let north = (node.x, new_y);

            let mut new_x = node.x;
            if node.x < (length_x - 1) {
                new_x += 1
            } 
            let east = (new_x,node.y);
            allowed_coordonne.push(north);
            allowed_coordonne.push(east);
            return allowed_coordonne;
        },
        Tiles::NorthToWest => {
            let mut new_y = node.y;
            if node.y > 0 {
                new_y -= 1;
            }
            let north = (node.x, new_y);

            let mut new_x = node.x;
            if node.x > 0 {
                new_x -= 1
            } 
            let west = (new_x,node.y);
            allowed_coordonne.push(north);
            allowed_coordonne.push(west);
            return allowed_coordonne;
        },
        Tiles::SouthToEast => {
            let mut new_y = node.y;
            if node.y < (length - 1) {
                new_y += 1;
            }
            let south = (node.x, new_y);

            let mut new_x = node.x;
            if node.x < (length_x - 1) {
                new_x += 1
            } 
            let east = (new_x,node.y);

            allowed_coordonne.push(south);
            allowed_coordonne.push(east);
            return allowed_coordonne;
        },
        Tiles::SouthToWest => {
            let mut new_y = node.y;
            if node.y < (length - 1) {
                new_y += 1;
            }
            let south = (node.x, new_y);

            let mut new_x = node.x;
            if node.x > 0 {
                new_x -= 1
            } 
            let west = (new_x,node.y);

            allowed_coordonne.push(south);
            allowed_coordonne.push(west);
            return allowed_coordonne;

        },
        Tiles::HorizontalPipe => {

            let mut new_x = node.x;
            if node.x < (length_x - 1) {
                new_x += 1
            } 
            let east = (new_x, node.y);


            let mut new_x = node.x;
            if node.x > 0 {
                new_x -= 1
            } 
            let west = (new_x,node.y);

            allowed_coordonne.push(east);
            allowed_coordonne.push(west);
            return allowed_coordonne;
        },
        _=> panic!("on ne devrais se retrouver en dehors des cas géré"),
    }

}

fn char_to_tiles(charactere: char) -> Tiles {
    match charactere {
        '|' => Tiles::VerticalPipe,
        '-' => Tiles::HorizontalPipe,
        'L' => Tiles::NorthToEast,
        'J' => Tiles::NorthToWest,
        '7' => Tiles::SouthToWest,
        'F' => Tiles::SouthToEast,
        '.' => Tiles::Ground,
        'S' => Tiles::Starting,
        _ => panic!("caractère non trouvé")
    }
}

fn check_neighbors_starting_point(coordonnee: (usize,usize), map: &Map) -> bool {

    let x = coordonnee.0;
    let y = coordonnee.1;
    let node = map.map.get(y).unwrap().get(x).unwrap();

    match node.tile {
        Tiles::Ground => false,
        Tiles::Starting => false,
        _ => true
    }
}