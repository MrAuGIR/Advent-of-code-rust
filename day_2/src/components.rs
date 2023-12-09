
#[derive(Debug, Clone)]
pub struct Game {
    pub indice: String,
    pub series: Vec<Serie>
}

impl Game {
    pub fn new(indice: String, series: Vec<Serie>) -> Game {
        Game {
            indice: indice,
            series:series,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Serie {
    pub cubes: Vec<Cube>
}

impl Serie {
    pub fn new(cubes: Vec<Cube>) -> Serie {
        Serie {
            cubes: cubes,
        }
    }

    pub fn is_availlable(&self,entries: &Entries) -> bool {

        let colors = vec![
            Color::Blue(String::from("blue")),
            Color::Red(String::from("red")),
            Color::Green(String::from("green")),
        ];

        for color in colors {
           // println!("test couleur {:#?}",color);
            if let Some(cube) = self.get_cube_color(&color) {
              //  println!("quantité cube {} < quantité entrie {}",cube.quantity.parse::<u16>().unwrap(),entries.get_quantity(&color));
                if cube.quantity.parse::<u16>().unwrap() > entries.get_quantity(&color) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_cube_color(&self, target_color: &Color) -> Option<&Cube> {
        self.cubes.iter().find(|&cube| &cube.color == target_color)
    }
    
}

#[derive(Debug,Clone)]
pub struct Cube {
    pub color: Color,
    pub quantity: String
}

impl Cube {
    pub fn new(color: Color, quantity: String) -> Cube {
        Cube {
            color: color,
            quantity: quantity,
        }
    }
}

pub struct Entries {
    pub red: u16,
    pub blue: u16,
    pub green: u16
}

impl Entries {
    pub fn get_quantity(&self, color: &Color) -> u16 {
        match color {
            Color::Blue(_) => self.blue,
            Color::Red(_) => self.red,
            Color::Green(_) => self.green,
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Color {
    Blue(String),
    Red(String),
    Green(String)
}
