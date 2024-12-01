use crate::components::{Hand, Card, JockerRule, StackRule};
use crate::game::{update_motif_with_jockers_rules, create_vec_of_cards_by_motif};
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

            let mut count_jocker = 0u64;

            let mut motif = data.get(0).unwrap().to_string();

            let mut there_are_jocker = false;
            
            for charac in motif.chars() {

                if charac == 'J' {
                    there_are_jocker = true;
                    count_jocker += 1u64;
                }

                let valeur = occurances.entry(charac.to_string()).or_insert(0u16);

                *valeur += 1u16;

                let card_to_add = cards.get(&charac.to_string()).expect("card not found in the game");

                vec_cards.push(card_to_add.clone());
            }

            // conversion du hasmap en vec
            let mut vec_card_count: Vec<(String, u16)> = occurances.clone().into_iter().collect();
            vec_card_count.sort_by(|a,b| b.1.cmp(&a.1));
            let old_cards: Vec<Card> = vec_cards.clone();

            // stack de rules a effectué
            let mut stacks_rules = StackRule::new(data.get(0).unwrap().to_string());

            // si des cartes jockers sont présentes
            while count_jocker > 0u64 {
                
                'occur: for (card_name, occurence) in &vec_card_count {

                    if *occurence == 5 {
                        count_jocker = 0u64;
                        break 'occur;
                    }

                    if card_name.to_owned() == 'J'.to_string() {
                        continue;
                    }

                    for _i in 0..occurence.to_owned() {
                        let jocker_rule = JockerRule{transform_to:card_name.clone()};
                        stacks_rules.rules.push(jocker_rule);
                        count_jocker -= 1u64;

                        if count_jocker == 0u64 {
                            break 'occur;
                        }
                    }
                }
            }

            if there_are_jocker {
                motif = update_motif_with_jockers_rules(&mut stacks_rules);
                println!("nouveau motif {:?}",motif);
                (vec_cards, occurances) = create_vec_of_cards_by_motif(motif.clone(),cards.to_owned());
                 //  println!("new vec_cards {:#?}", vec_cards);
                //  println!("new occurances : {:#?} ",occurances);
            } else {
                println!("motif {:?}",motif);
            }

            // a ajouter ancien et nouveau motif trier avec l'ancien motif quand on est sur le cas d'une égalité
            hands.push(Hand::new(
                vec_cards,
                old_cards,
                 data.get(1).map(|b| b.parse::<u32>().expect("error parse")).unwrap(),
                 motif.clone(),
                 data.get(0).unwrap().to_string(),
                 occurances,
                 provisoire_score,
                ));
        }
    }
    hands
}