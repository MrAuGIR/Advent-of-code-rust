use std::collections::HashMap;
use std::cmp::Ordering;
use crate::game::Suite;



#[derive(Debug, Clone)]
pub struct Card {
    pub label: String,
    pub strength: u16
}

impl Card {

    pub fn new(label: String, strength: u16) -> Card {
        Card {
            label: label,
            strength: strength
        }
    }

    pub fn compare_cards(a: &Card, b: &Card) -> Ordering {
        a.strength.cmp(&b.strength)
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u32,
    pub motif: String,
    pub rank: u16,
    pub occurences: HashMap<String,u16>,
    pub type_of_hand: Suite,
    pub score: u64
}

impl Hand {
    
    pub fn new(cards: Vec<Card>, bid: u32, motif:String, occurences: HashMap<String, u16>, score: u64) -> Hand {
        Hand {
            cards: cards,
            bid: bid,
            motif: motif,
            rank: 999u16,
            occurences: occurences,
            type_of_hand: Suite::HightCard,
            score: score
        }
    }

    pub fn make_pattern_occurence(&self) -> String {
        let mut motif = String::new();
    
        for (_cle, &valeur) in self.occurences.iter() {
            motif.push_str(&format!("{}", valeur));
        }
    
        motif
    }
    
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub hands: Vec<Hand>
}
