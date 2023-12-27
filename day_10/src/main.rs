mod component;
mod transformer;
mod reader;

use component::{Map, Node, Tiles};
use transformer::*;

use crate::reader::*;

fn main() {
    
  let path_to_map = String::from("./input/data.txt");

  // init map
  let mut map = Map::new();

  if let Ok(lines) = read_lines(path_to_map) {
      
    hydrate_map(lines, &mut map);
    connect_neighbors(&mut map);

    if let Some(paths) = map.parcour_en_profondeur() {
      println!("path trouver {:?}",paths.len());

      if let Some(path) = paths.iter().max_by_key(|v| v.len()) {
        
        let mut path_as_string = " ".to_owned();
        for node in path {
          path_as_string.push_str(format!("({},{})",node.x,node.y).as_str())
        }

        let max_step = path.len() / 2;

        println!("{:?}",path_as_string);
        println!("{}", max_step);

      } else {
          println!("Le vecteur principal est vide");
      }
    }
      
  }

  

  //println!("{:#?}", map);
}