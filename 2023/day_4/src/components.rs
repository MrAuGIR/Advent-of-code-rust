use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub index: usize,
    pub winning_suite: Vec<u16>,
    pub played_suite: Vec<u16>,
    pub occurence: HashMap<u16, usize>,
    pub score: u32,
    pub copies_to_win: Vec<u16>, 
}

impl Card {
    pub fn new (winning: Vec<u16>, played: Vec<u16>) -> Card {
        Card {
            index: 0,
            winning_suite: winning,
            played_suite: played,
            occurence: HashMap::new(),
            score: 0u32,
            copies_to_win: Vec::new(), 
        }
    }

    pub fn set_index(&mut self,i: usize) {
        self.index = i;
    }

    pub fn find_occurences(&mut self) {

        // calcul des occurences trouver dans les suites de nombres
        for &win_number in &self.winning_suite {
            if let Some(_) = self.played_suite.iter().find(|&n| n == &win_number) {
                let counter = self.occurence.entry(win_number).or_insert(0);
                *counter += 1;
            }
        }
        self.score = self.occurence.len() as u32;
    }

    pub fn calculate_copies_to_win(&mut self, cards: &Vec<Card>) {
        // Effacer les anciennes copies à gagner
        self.copies_to_win.clear();

        if let Some(index) = cards.iter().position(|c| c.index == self.index) {
            // Copy the cards below based on the number of matching numbers
            let start_index = index + 1;
            let end_index = index + 1 + self.occurence.len();  // Adjust here to use the number of matching numbers
            for i in start_index..end_index {
                if i < cards.len() {
                    self.copies_to_win.push(i as u16);
                } else {
                    break;
                }
            }
        }
    
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub cards: Vec<Card>,
    pub copy_cards: Vec<Card>,
    pub total_score: u32,
}

impl Game {
    pub fn new () -> Game {
        Game {
            cards: Vec::new(),
            copy_cards: Vec::new(),
            total_score: 0u32,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        
    }

    pub fn get_clone_by_index(&self, index: usize) -> Card {
        return self.cards.get(index).unwrap().clone();
    }

    pub fn check_occurence_in_cards(&mut self) {
        let cards_len = self.cards.len();
        
        for i in 0..cards_len{
            self.cards[i].find_occurences();
        }
    }

    pub fn prepare_copies_card(&mut self) {
        let cards_len = self.cards.len();
        let mut cloned_cards = self.cards.clone();
        for i in 0..cards_len{
            self.cards[i].calculate_copies_to_win(&mut cloned_cards);
        }
    }

    pub fn get_all_scores(&mut self) -> Vec<Card>{
        self.total_score = 0u32;
        let mut card_stack = self.cards.to_vec();
        
        let mut card_zero_score: Vec<Card> = Vec::new();
    
        while let Some(current_card) = Some(card_stack.remove(0)) {
            
            println!("longueur de la pile {:?} ",card_stack.len());

            if current_card.score > 0 {

                for index in &current_card.copies_to_win {
                    card_stack.push(self.get_clone_by_index(index.clone() as usize));
                }
                
            }
            card_zero_score.push(current_card);
            
            if card_stack.is_empty() {
                break;
            }
        }
        return card_zero_score;
    }
}