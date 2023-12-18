
use std::collections::HashMap;
use crate::components::*;
use std::cmp::Ordering;

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub enum Suite {
    HightCard = 0,
    OnePaire = 100,
    TwoPair = 200,
    ThreeOfKing = 300,
    FullHouse = 400,
    FourOfKing = 500,
    FiveOfKing = 600,
}

pub fn initialize_game() -> HashMap<String,Card>{

    let cards_definition = HashMap::from([
        (2u16,"2"),
        (3u16,"3"),
        (4u16,"4"),
        (5u16,"5"),
        (6u16,"6"),
        (7u16,"7"),
        (8u16,"8"),
        (9u16,"9"),
        (10u16,"T"),
        (11u16,"J"),
        (12u16,"Q"),
        (13u16,"K"),
        (14u16,"A")
    ]);

    let mut cards: HashMap<String,Card> = HashMap::new();

    for (strength,label) in &cards_definition {
        cards.insert(label.to_string(),Card::new(label.to_string(), strength.clone()));
    }
    return cards;
}


pub fn determine_type_of_hand(hand: &mut Hand)
{
    if hand.occurences.len() >= 5 {
        hand.score *= 1 ;
    } else if hand.occurences.len() >= 4 {
        hand.score += 100;
        hand.type_of_hand = Suite::OnePaire;
    } else {
        let pattern = hand.make_pattern_occurence();

        match &*pattern {
            "221" | "122" | "212" => {
                hand.score += 200;
                hand.type_of_hand = Suite::TwoPair;
            },
            "311" | "131" | "113" => {
                hand.score += 300;
                hand.type_of_hand = Suite::ThreeOfKing;
            }
            "32" | "23" => {
                hand.score += 400;
                hand.type_of_hand = Suite::FullHouse;
            },
            "41" | "14" => {
                hand.score += 500;
                hand.type_of_hand = Suite::FourOfKing;
            },
            "5" => {
                hand.score += 600;
                hand.type_of_hand = Suite::FiveOfKing;
            }
            _ => {
                panic!("oula ! quelle type de main nous avons ?")
            }
        }
    }
}

pub fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    // Compare les types de main en premier
    let type_ordering = a.score.cmp(&b.score);
    //let type_ordering = a.type_of_hand.cmp(&b.type_of_hand);

    if type_ordering == Ordering::Equal {
        // Si les types sont égaux, compare les cartes en fonction de leur force
        for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
            let card_ordering = Card::compare_cards(card_a, card_b);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        // Si toutes les cartes sont égales, compare le score
        a.score.cmp(&b.score)
    } else {
        type_ordering
    }
}