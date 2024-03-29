use std::mem;

use crate::card::{Card, Suit};

const MAX_CARDS_FOLD: usize = 4;

pub struct Fold {
    pub cards: Vec<Card>,
}

impl Fold {
    pub fn new_empty() -> Fold {
        let cards = Vec::with_capacity(MAX_CARDS_FOLD);
        Fold::new(cards)
    }

    pub fn new(cards: Vec<Card>) -> Fold {
        Fold { cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_over(&self) -> bool {
        return self.cards.len() >= MAX_CARDS_FOLD;
    }

    pub fn get_cards(&mut self) -> Vec<Card> {
        mem::replace(&mut self.cards, Vec::with_capacity(MAX_CARDS_FOLD))
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_main_suit(&self) -> Result<Suit, &'static str> {
        if self.len() == 0 {
            Err("fold is empty no main color")
        } else {
            Ok(self.cards[0].suit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::card::{card, get_suit_shortcut, get_symbol_shortcut};
    use crate::card::{Card, Suit, Symbol};

    #[test]
    fn init_fold() {
        let _fold = Fold::new_empty();
    }
}
