use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Category {
    pub index: usize,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Mapper {
    pub maps: Vec<Map>,
    pub mapping: BTreeMap<u32, u32>,
    pub source: Category,
    pub destination: Category,
}

impl Mapper {

    pub fn new(source: Category, destination: Category) -> Mapper {
        
        return Mapper {
            maps: Vec::new(),
            mapping: Mapper::init_mapping(),
            source: source,
            destination: destination
        };
    }

    pub fn push_maps(&mut self, map: Map ){
        self.maps.push(map);
    }

    fn init_mapping() -> BTreeMap<u32, u32> {
        let mut hash_map: BTreeMap<u32, u32> = BTreeMap::new();

        for i in 0..100 {
            hash_map.insert(i,i);
        }
        return hash_map;
    }

    pub fn execute_maps(&mut self) {

        for map in &self.maps {
            let start_position = map.index_source;
            let start_value = map.index_destination;
            let length = map.length;

            for (index, value) in self.mapping.range_mut(start_position..start_position + length) {
                *value = start_value + index - start_position;
                //println!("{:?} + {:?} = {:?}",start_value,index - start_position,*value);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub index_destination: u32,
    pub index_source: u32,
    pub length: u32
}

impl Map {
    pub fn new<'a>(destination: &str, source: &str, length: &str) -> Result<Self, &'a str> {

        let destination = destination.parse::<u32>().map_err(|_| "Échec de la conversion destination")?;
        let source = source.parse::<u32>().map_err(|_| "echec de la conversion source")?;
        let length = length.parse::<u32>().map_err(|_| "Echec de la conversion de length")?;

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