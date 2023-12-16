mod reader;
mod components;
mod transformer;

use std::path::Path;
use crate::reader::*;
use crate::transformer::*;
use crate::components::*;

fn main() {

    let mut almanach = Almanach::new();
    let mut min_location: Option<u64 >= None;
    
    /*let paths = [
        "./input/calibration/map_seed_soil.txt",
        "./input/calibration/map_soil_fertilizer.txt",
        "./input/calibration/map_fertilizer_water.txt",
        "./input/calibration/map_water_light.txt",
        "./input/calibration/map_light_temperature.txt",
        "./input/calibration/map_temperature_humidity.txt",
        "./input/calibration/map_humidity_location.txt",
    ];*/
 

    let paths = [
        "./input/data/map_seed_soil.txt",
        "./input/data/map_soil_fertilizer.txt",
        "./input/data/map_fertilizer_water.txt",
        "./input/data/map_water_light.txt",
        "./input/data/map_light_temperature.txt",
        "./input/data/map_temperature_humidity.txt",
        "./input/data/map_humidity_location.txt",
    ];

    let seeds = get_seeds(String::from("./input/data/seeds.txt"));
    

    let mut index = 0;

    for path in paths {

        let mut path_string = String::from(path);
        let categories = get_categories(&mut path_string);
        
        if let Ok(lines) = read_lines(path.to_owned()) {

            let source = categories.get(1).unwrap().to_string();
            let destination = categories.get(2).unwrap().to_string();

            let mut mapper = transform_lines_to_mapper(lines,index,source,destination);
            
             // mapper.execute_maps();
            
            almanach.add_mapper(mapper);
        }

        index += 2;
    }
//    println!("{:#?}",almanach);

    for seed in seeds {
        let trace = make_trace(&mut almanach.mappers, seed);

        let last = trace.last();

        min_location = match (last, min_location) {
            (None, _) => min_location,
            (Some(min), None) => Some(*min),
            (Some(min), Some(current_min)) => {
                if *min < current_min {
                    Some(*min)
                } else {
                    Some(current_min)
                }
            }
        };
        
        let result: String = trace.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
        println!("{:#?}",result);
   } 

    if let Some(min) = min_location {
        println!("La plus petite valeur est : {}", min);
    } else {
        println!("Aucune valeur à comparer.");
    }

}

pub fn get_categories<'a>(path: &'a mut String) -> Vec<&'a str> {
    let path = Path::new(path);
    // Récupérer le dernier élément du chemin (nom du fichier sans extension)
    if let Some(file_stem) = path.file_stem()  {
        // Convertir le file_stem en une chaîne de caractères
        if let Some(file_stem_str) = file_stem.to_str() {
            // Diviser la chaîne en utilisant le caractère `_`
            let parts: Vec<&'a str> = file_stem_str.split('_').collect();

            return parts;
        } else {
            
            return Vec::new();
        }
    }
    return Vec::new();
}

pub fn make_trace(mappers: &mut Vec<Mapper>, seed: u64) -> Vec<u64> {

    mappers
    .sort_by(|mapper,b| mapper.source.index.cmp(&b.source.index));

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
    let mut value_in = source.clone();
    let mut value_out = value_in;

    println!("{:?} to {:?} ", mapper.source, mapper.destination);
    for map in mapper.maps {
        
        if value_in >= map.index_source && value_in <= map.index_source + map.length {
            println!("valeur {:?} est >= {:?} et <= à {:?}",value_in,map.index_source, map.index_source + map.length);
            let offset = value_in - map.index_source;
            value_out = map.index_destination + offset;
            break;
        } else {
            println!("valeur {:?} est <= {:?} ou >= à {:?}",value_in,map.index_source, map.index_source + map.length);
        }
        println!("nouvelle valeur de value_out {:?}",value_out);
    }
    
    return value_out;
}

pub fn get_seeds(path: String) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(path.to_owned()) {
        for line in lines {
            if let Ok(line_content) = line {
                for number_str in line_content.split_whitespace() {
                    if let Ok(number) = number_str.parse::<u64>() {
                        result.push(number);
                    } else {
                        // Gérer l'erreur de conversion
                        println!("Erreur de conversion : {}", number_str);
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
                        // Gérer l'erreur de conversion
                        // ...
                    }
                }
            }
        }
    }

     // Ajouter le minimum et le maximum au résultat
    if let Some(min_val) = min {
        result.push(min_val);
    }

    if let Some(max_val) = max {
        result.push(max_val);
    }

    result
}