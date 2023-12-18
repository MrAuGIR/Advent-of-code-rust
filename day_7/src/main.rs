mod game;
mod components;
mod reader;
mod transformers;

use transformers::transform_line_to_hand;

use crate::game::*;
use crate::components::*;

fn main() {

    // on récupère les mains "hands"
    let path = String::from("./input/calibration/hands.txt");

    // initialise game
    let cards = initialize_game();

    // creation des mains
    let mut hands: Vec<Hand> = transform_line_to_hand(&path, cards);
    
    // determiner type de chaque main
    for hand in &mut hands {
        determine_type_of_hand(hand);
    }

    // trié les mains
    hands.sort_by(|a,b| a.score.cmp(&b.score));
    // re trie des mains
    hands.sort_by(|a, b| compare_hands(a, b));

    // calcul du score total
    let mut score_total = 0u32;

    for (index,hand) in hands.iter().enumerate() {
        println!("suite: {} -> score {:?}",hand.motif, hand.score);

        let index_str = index + 1;
        score_total += (index + 1) as u32 * hand.bid;

        println!(" {:?} x {:?}",index_str,hand.bid);
    }

    println!("{:#?}", score_total);
    
}
