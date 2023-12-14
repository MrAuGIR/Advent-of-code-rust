mod reader;
mod components;
mod transformer;

use std::path::Path;
use crate::reader::*;
use crate::transformer::*;
use crate::components::*;

fn main() {

    let mut almanach = Almanach::new();
    
    let paths = [
        "./input/calibration/map_seed_soil.txt",
        "./input/calibration/map_soil_fertilizer.txt",
        "./input/calibration/map_fertilizer_water.txt",
        "./input/calibration/map_water_light.txt",
        "./input/calibration/map_light_temperature.txt",
        "./input/calibration/map_temperature_humidity.txt",
        "./input/calibration/map_humidity_location.txt",
    ];

    let mut index = 0;

    for path in paths {

        let mut path_string = String::from(path);
        let categories = get_categories(&mut path_string);
        
        if let Ok(lines) = read_lines(path.to_owned()) {

            let source = categories.get(1).unwrap().to_string();
            let destination = categories.get(2).unwrap().to_string();

            let mut mapper = transform_lines_to_mapper(lines,index,source,destination);
            
            mapper.execute_maps();
            
            almanach.add_mapper(mapper);
        }

        index += 2;
    }

   println!("{:#?}",almanach.mappers);
    
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