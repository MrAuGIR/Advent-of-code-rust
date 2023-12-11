use std::collections::HashMap;
use num::pow;


#[derive(Debug, Clone)]
pub struct Card {
    pub index: usize,
    pub winning_suite: Vec<u16>,
    pub played_suite: Vec<u16>,
    pub occurence: HashMap<u16, usize>,
    pub score: u32,
}

impl Card {
    pub fn new (winning: Vec<u16>, played: Vec<u16>) -> Card {
        Card {
            index: 0,
            winning_suite: winning,
            played_suite: played,
            occurence: HashMap::new(),
            score: 0u32,
        }
    }

    pub fn set_index(&mut self,i: usize) {
        self.index = i;
    }

    pub fn find_occurences(&mut self) {

        for &win_number in &self.winning_suite {

            if let Some(_) = self.played_suite.iter().find(|&n| n == &win_number) {
               let counter = self.occurence.entry(win_number).or_insert(0);
               *counter += 1;
            } else {
                continue;
            }
        }
    }

    pub fn calcul_score(&mut self) {
        if self.occurence.is_empty() {
            self.score = 0;
            
        } else {
            self.score = pow(2,self.occurence.len() - 1);
        }
        // println!("{}^{} = {}", 2, self.occurence.len(), self.score);
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    cards: Vec<Card>,
    total_score: u32,
}

impl Game {
    pub fn new () -> Game {
        Game {
            cards: Vec::new(),
            total_score: 0u32,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        
    }

    pub fn check_occurence_in_cards(&mut self) {
        for card in &mut self.cards {
            card.find_occurences();
        }
    }

    pub fn get_all_scores(&mut self) {
        self.total_score = 0u32;
        for card in &mut self.cards {
            card.calcul_score();
            self.total_score += card.score;
            println!("score {:?}", card.score);
        }
        println!("score total {:?}", self.total_score);
    }
}