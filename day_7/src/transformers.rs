use crate::components::{Hand, Card};
use crate::reader::read_lines;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Lines};


pub fn transform_line_to_hand(path: &String,cards: HashMap<String, Card>) -> Vec<Hand> {

    if let Ok(lines) = read_lines(path) {

        return create_hands(lines, cards);
    } else {
        panic!("can read file");
    }
}

fn create_hands(lines: Lines<io::BufReader<File>>, cards: HashMap<String,Card>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        
        if let Ok(line) = line {

            let mut vec_cards:Vec<Card> = Vec::new();

            let mut occurances : HashMap<String,u16> = HashMap::new();

            let data: Vec<&str> = line.split_whitespace().collect();

            let provisoire_score = 0u64;
            
            for charac in data.get(0).unwrap().chars() {

                let valeur = occurances.entry(charac.to_string()).or_insert(0u16);

                *valeur += 1u16;

                let card_to_add = cards.get(&charac.to_string()).expect("card not found in the game");

                // provisoire_score += card_to_add.strength as u64; // partie 1 seul la première carte compte dans le score en cas d'egalité
               /*  if counter < 1 {
                    provisoire_score += card_to_add.strength as u64;
                    counter += 1u16;
                }
                */
                vec_cards.push(card_to_add.clone());
            }
            hands.push(Hand::new(
                vec_cards,
                 data.get(1).map(|b| b.parse::<u32>().expect("error parse")).unwrap(),
                 data.get(0).unwrap().to_string(),
                 occurances,
                 provisoire_score,
                ));
        }
    }
    hands
}