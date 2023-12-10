mod components;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

use crate::components::*;



fn main() {
    let path = String::from("./input/data.txt");

    let regex_indice_game = Regex::new(r"(\d+):").expect("error in regex");

    let entries = Entries{
        red: 12,
        green: 13,
        blue: 14
    };

    let mut available_games: Vec<Game> = Vec::new();
    let mut games: Vec<Game> = Vec::new();
    
    if let Ok(lines) = read_lines(path.to_owned()) {
        for line in lines {
            if let Ok(ip) = line   {

                // indice de la game
                let indice: Option<&str> =  regex_indice_game.find(&ip).map(|m| m.as_str());
                let indice_cleaned = indice.map(|s| s.trim_end_matches(":")).unwrap();
                
                let mut data =  ip.to_string();

                // suppression du 'Game %d :' et split de la chaine
                let indice = data.find(":").unwrap();
                data.drain(..indice + 1);
                let trimmed_series: Vec<&str> = data.trim().split(";").map(|m| m.trim()).collect();
                
                let mut series: Vec<Serie> = Vec::new();
                for serie in &trimmed_series {
                    let trimmed_cube: Vec<&str> = serie.split(",").map(|m| m.trim()).collect();

                    let mut cubes:  Vec<Cube> = Vec::new();
                    for data in &trimmed_cube {
                        let data_cube: Vec<&str> = data.split(" ").collect();

                        let cube = match data_cube.get(1).unwrap().to_lowercase().as_str() {
                            "red" => {
                                Cube::new(Color::Red(String::from("red")), data_cube.get(0).unwrap().to_string())
                            },
                            "blue" => {
                                Cube::new(Color::Blue(String::from("blue")), data_cube.get(0).unwrap().to_string())
                            },
                            "green" => {
                                Cube::new(Color::Green(String::from("green")), data_cube.get(0).unwrap().to_string())
                            },
                            _ => {
                                Cube::new(Color::Green(String::from("green")),String::from("0"))
                            }
                        };
                        cubes.push(cube);
                    }
                    series.push(Serie::new(cubes));
                }
                games.push(Game::new(indice_cleaned.to_string(), series))
            }
        }
    }
   // println!("{:#?}",games);

   // get_available_games(&entries, &games, &mut available_games);

   get_games_powed( &games);

    // somme des games valables
    let mut count = 0u16;
    for game in &available_games {
        count += game.indice.parse::<u16>().unwrap();
    }
    println!("Somme des games ok! {:?}",count);

  //  println!("{:#?}", available_games);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_available_games(entries: &Entries, games: &Vec<Game>, available_games: &mut Vec<Game> ) {
    'game_loop:for game in games {
        for serie in &game.series {
            if serie.is_availlable(entries) != true {
                println!("game {:#?} n'est pas valable",game.indice);
                continue 'game_loop;
            }
        }
        println!("game {:#?} est valable",game.indice);
        available_games.push(game.clone())
    }
}

fn get_games_powed(games: &Vec<Game>) {
    let mut sum = 0;
    for game in games {
        println!("game {:?} : {:#?}",game.indice,game.get_powed());
        sum += game.get_powed();
    }
    println!("sum : {:?}", sum);
}