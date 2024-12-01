pub fn get_all_scores(&mut self) -> Vec<Card>{
    self.total_score = 0u32;
    let mut card_stack = self.cards.to_vec();
    println!("longueur de la pile de depart {:?} ",card_stack.len());
    let mut card_zero_score: Vec<Card> = Vec::new();

    while let Some(mut current_card) = Some(card_stack.remove(0)) {
        
       // println!("current card {:?} ",current_card.index);
       
        // Appliquer les règles spécifiques
        current_card.calcul_score(); // Exemple, ajustez cela en fonction de vos règles

        // Si la carte est gagnante, copiez les cartes en dessous et ajoutez-les à la pile
        if current_card.score > 0 {
           // println!("score de la card {:?} : {:?} ",current_card.index,current_card.score);

            let copies = self.get_copies_below(&current_card);

            //println!("longueur des copies {:?}",copies.len());

            card_stack.extend(copies.iter().cloned());

        }
        card_zero_score.push(current_card);
        println!("scrate {:?}",card_zero_score.len());
        if card_stack.is_empty() {
            break;
        }
    }
    return card_zero_score;
}

pub fn get_copies_below(&mut self, card: &Card) -> Vec<Card> {
    let mut copies = Vec::new();

    // Get the index of the current card
    if let Some(index) = self.cards.iter().position(|c| c.index == card.index) {
        // Copy the cards below based on the number of matching numbers
        let start_index = index + 1;
        let end_index = index + 1 + card.occurence.len();  // Adjust here to use the number of matching numbers
        for i in start_index..end_index {
            if i < self.cards.len() {
                let copy = self.cards[i].clone();
                copies.push(copy);
            } else {
                break;
            }
        }
    }

    // Add the copies to the copy stack
    self.copy_cards.extend(copies.iter().cloned());

    copies
}