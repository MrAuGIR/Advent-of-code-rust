use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Category {
    pub index: usize,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Mapper {
    pub maps: Vec<Map>,
    pub mapping: BTreeMap<u64, u64>,
    pub source: Category,
    pub destination: Category,
}

impl Mapper {

    pub fn new(source: Category, destination: Category) -> Mapper {
        
        return Mapper {
            maps: Vec::new(),
            mapping: BTreeMap::new(),
            source: source,
            destination: destination
        };
    }

    pub fn push_maps(&mut self, map: Map ){
        self.maps.push(map);
    }

    pub fn execute_maps(&mut self) {

        for map in &self.maps {
            let start_position = map.index_source;
            let start_value = map.index_destination;
            let length = map.length;

            /* for index in start_position..start_position + length {
                
                self.mapping.insert(index, start_value + index - start_position);
                println!("{:?} + {:?} = {:?}",start_value,index - start_position,start_value + index - start_position);
            } */

             // Définir la taille du lot en fonction de la distribution des valeurs
            let batch_size = length / 10 as u64;
            println!("batch size {:?} ",batch_size);
            // Utiliser le traitement par lots
            for batch_start in (start_position..start_position + length).step_by(batch_size as usize) {
                let batch_end = (batch_start + batch_size).min(start_position + length);

                let values: Vec<_> = (batch_start..batch_end)
                    .map(|index| (index, start_value + index - start_position))
                    .collect();
                
                self.mapping.extend(values);
            }
        }
        
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub index_destination: u64,
    pub index_source: u64,
    pub length: u64
}

impl Map {
    pub fn new<'a>(destination: &str, source: &str, length: &str) -> Result<Self, &'a str> {

        let destination = destination.parse::<u64>().map_err(|_| "Échec de la conversion destination")?;
        let source = source.parse::<u64>().map_err(|_| "echec de la conversion source")?;
        let length = length.parse::<u64>().map_err(|_| "Echec de la conversion de length")?;

        Ok(Map {
            index_destination: destination,
            index_source: source,
            length: length,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Almanach {
    pub mappers: Vec<Mapper>
}

impl Almanach {

    pub fn new() -> Almanach {
        Almanach {
            mappers: Vec::new()
        }
    }

    pub fn add_mapper(&mut self,mapper: Mapper) {
        self.mappers.push(mapper);
    }
}

#[derive(Debug, Clone)]
pub struct SeedRang
{
    pub seed_start: u64,
    pub seed_length: u64,
    pub seed_end: u64,
}

impl SeedRang {

    pub fn new(start: u64, length: u64) -> SeedRang
    {
        SeedRang {
            seed_length: length,
            seed_start: start,
            seed_end: start + length,
        }
    }
}