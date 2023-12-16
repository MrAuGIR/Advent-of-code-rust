mod reader;
mod components;
mod transformer;
mod env;

use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::reader::*;
use crate::transformer::*;
use crate::components::*;
use crate::env::*;

fn main() {

    let mode = String::from("prod");
    
    let paths = get_paths_by_mod(mode);

    let mut almanach = Almanach::new();
    let mut min_location: Option<u64 >= None;
    let minimal_location = Arc::new(Mutex::new(None));
    let mut handles: Vec<_> = Vec::new();
    
    let seeds = get_seeds(String::from("./input/data/seeds.txt"));
    //println!("{:#?}",seeds);

    let mut index = 0;

    for path in paths {

        let mut path_string = String::from(path);
        let categories = get_categories(&mut path_string);
        
        if let Ok(lines) = read_lines(path.to_owned()) {

            let source = categories.get(1).unwrap().to_string();
            let destination = categories.get(2).unwrap().to_string();

            let mapper = transform_lines_to_mapper(lines,index,source,destination);
            
            almanach.add_mapper(mapper);
        }

        index += 2;
    }
//    println!("{:#?}",almanach);
    let almanach_shared = Arc::new(Mutex::new(almanach.clone()));

    for seed_range in seeds {

        let minimal_location: Arc<Mutex<Option<u64>>> = Arc::clone(&minimal_location);
        let almanach_shared = Arc::clone(&almanach_shared);
       
        let handle = thread::spawn(move || {
            for seed in  seed_range.seed_start..seed_range.seed_end + 1u64 {

                let almanach = almanach_shared.lock().unwrap();
      
                  let trace = make_trace(&mut almanach.mappers.clone(), seed);
      
                  let last = trace.last();

                  let mut minimal_location = minimal_location.lock().unwrap();
      
                  *minimal_location = match (last, *minimal_location) {
                      (None, _) => minimal_location.clone(),
                      (Some(min), None) => Some(*min),
                      (Some(min), Some(current_min)) => {
                          if *min < current_min {
                              Some(*min)
                          } else {
                              Some(current_min)
                          }
                      }
                  };
      
                 // let result: String = trace.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
                 // println!("{:#?}",result);
              }
        });
        handles.push(handle);
    } 

    // Attendre que tous les threads se terminent
    for handle in handles {
        handle.join().unwrap();
    }

    // Récupérer la valeur finale de minimal_location
    let mut minimal_location = minimal_location.lock().unwrap();

    if let Some(min) = minimal_location.take() {
        println!("La plus petite valeur est : {}", min);
    } else {
        println!("Aucune valeur à comparer.");
    }

}

pub fn get_categories<'a>(path: &'a mut String) -> Vec<&'a str> {
    let path = Path::new(path);
    
    if let Some(file_stem) = path.file_stem()  {
        
        if let Some(file_stem_str) = file_stem.to_str() {
            
            let parts: Vec<&'a str> = file_stem_str.split('_').collect();

            return parts;
        } else {
            
            return Vec::new();
        }
    }
    return Vec::new();
}

pub fn make_trace(mappers: &mut Vec<Mapper>, seed: u64) -> Vec<u64> {

   /*  mappers
    .sort_by(|mapper,b| mapper.source.index.cmp(&b.source.index)); */

    let mut trace: Vec<u64> = Vec::new();
    let mut source = seed.clone();

    trace.push(source.clone());

    for mapper in mappers {
        
        let destination =  find_source_destination(&source, mapper.clone());
        source = destination;

       trace.push(destination);
    }

    return trace;
}


pub fn find_source_destination(source: &u64,mapper: Mapper) -> u64 {
    /*let destination = mapper.mapping.get(source).unwrap_or(source); // on n'utilise pas le mapping car trop gournmand en ressources
    return destination.clone(); */
    let value_in = source.clone();
    let mut value_out = value_in;

    for map in mapper.maps {
        
        if value_in >= map.index_source && value_in <= map.index_source + map.length {
            
            let offset = value_in - map.index_source;
            value_out = map.index_destination + offset;
            break;
        } else {
          //  println!("valeur {:?} est <= {:?} ou >= à {:?}",value_in,map.index_source, map.index_source + map.length);
        }
    }
    
    return value_out;
}

pub fn get_seeds(path: String) -> Vec<SeedRang> {
    let mut result: Vec<SeedRang> = Vec::new();

    if let Ok(lines) = read_lines(path.to_owned()) {

        for line in lines {

            if let Ok(line_content) = line {
                    let iter: Vec<&str> = line_content.split_whitespace().collect();

                    let mut iter = iter.iter();

                    while let Some(start_str) = iter.next() {
                        if let Some(length_str) = iter.next() {
                            if let (Ok(start), Ok(length)) = (start_str.parse::<u64>(), length_str.parse::<u64>()) {
                                
                                let seed_range = SeedRang::new(start, length);
                                result.push(seed_range);
                            } else {
                                println!("Erreur de conversion : {} ou {} ", start_str, length_str)
                            }
                        } else {
                            println!("nombre impaire de fichier");
                        }
                    }

            }
        }
    }

    result
}

pub fn min_max_range(path: String) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut min: Option<u64> = None;
    let mut max: Option<u64> = None;

    if let Ok(lines) = read_lines(path.to_owned()) {
        for line in lines {
            if let Ok(line_content) = line {
                for number_str in line_content.split_whitespace() {
                    if let Ok(number) = number_str.parse::<u64>() {
                        // Mettre à jour le minimum et le maximum
                        min = match min {
                            None => Some(number),
                            Some(current_min) => Some(current_min.min(number)),
                        };

                        max = match max {
                            None => Some(number),
                            Some(current_max) => Some(current_max.max(number)),
                        };
                    } else {
                        
                    }
                }
            }
        }
    }

    if let Some(min_val) = min {
        result.push(min_val);
    }

    if let Some(max_val) = max {
        result.push(max_val);
    }

    result
}