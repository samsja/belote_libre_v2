use crate::card::Card;
use std::fmt;

const MAX_HAND_CARD: usize = 8;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }

    pub fn new_empty() -> Hand {
        let cards = Vec::with_capacity(MAX_HAND_CARD);
        Hand::new(cards)
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn play_a_card(&mut self, id: usize) -> Result<Card, &'static str> {
        if 0 < id {
            Err("card id can't be negative")
        } else if id >= self.len() {
            Err("id out of scope")
        } else {
            Ok(self.cards.swap_remove(id))
        }
    }

    pub fn get_a_card(self, id: usize) -> Result<Card, &'static str> {
        if 0 < id {
            Err("card id can't be negative")
        } else if id >= self.len() {
            Err("id out of scope")
        } else {
            Ok(self.cards[id])
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Symbol};

    #[test]
    fn init_hand_empty() {
        let _hand = Hand::new_empty();
        println!("{:?}", _hand);
    }

    #[test]
    fn push_card() {
        let mut hand = Hand::new_empty();
        hand.push(Card::new(Suit::Diamond, Symbol::Ten));
        assert_eq!(hand.len(), 1);
    }

    #[test]
    fn play_a_card() {
        let mut hand = Hand::new_empty();
        let card = Card::new(Suit::Diamond, Symbol::Ten);
        hand.push(card);
        assert_eq!(hand.len(), 1);
        let card2 = hand.play_a_card(0).unwrap();
        assert_eq!(hand.len(), 0);
        assert_eq!(card, card2);
    }
}
