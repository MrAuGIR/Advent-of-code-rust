
use std::fs::File;
use std::io::{self, Lines};

use crate::components::*;

pub fn transform_to_list(lines: Lines<io::BufReader<File>>) -> Game  {
    let mut game = Game::new();
    for (line_index,line) in lines.enumerate() {

        if let Ok(line_content) = line {

            
            if let Some((_, rest)) = line_content.split_once(": ") {
                println!("{}", rest);
                game.add_card(transform_card(line_index + 1,rest.to_string()));
            } else {
                println!("{}", line_content);
            }
        }
    }
    return game;
}

pub fn transform_card(index: usize, line: String) -> Card {
    let cards: Vec<Vec<u16>> = line.split("|").map(|serie| transform_suite(serie.trim().to_owned())).collect();
    let mut card = Card::new(cards[0].clone(), cards[1].clone());
    card.set_index(index);
    return card;
}

pub fn transform_suite(suite_str: String) -> Vec<u16> {
    
    let vec = suite_str
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(|m| {
            m.trim().parse::<u16>().unwrap_or(0)
        }).collect();

    return vec;
}