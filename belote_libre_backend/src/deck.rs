use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Suit, Symbol};

use itertools::iproduct;
use strum::IntoEnumIterator;

const MAX_CARDS_DECK: usize = 42;

#[derive(Debug)]
pub struct Deck {
    #[allow(dead_code)]
    cards: Vec<Card>,
}

impl Deck {
    fn new_empty() -> Deck {
        Deck {
            cards: Vec::with_capacity(MAX_CARDS_DECK),
        }
    }

    pub fn new_ordered() -> Deck {
        let mut deck = Deck::new_empty();

        deck.cards.extend(
            iproduct!(Suit::iter(), Symbol::iter()).map(|(suit, sym)| Card::new(suit, sym)),
        );
        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn cut(&mut self) {
       // todo implement cut 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_deck_empty() {
        let _deck = Deck::new_empty();
        println!("{:?}", _deck);
    }

    #[test]
    fn init_deck_ordered() {
        let deck = Deck::new_ordered();
        println!("{:?}", deck);
        assert_eq!(deck.cards.len(), 32);
    }

    #[test]
    fn deck_shuffle() {
        let mut deck = Deck::new_ordered();
        assert_eq!(deck.cards.len(), 32);
        deck.shuffle();
        assert_eq!(deck.cards.len(), 32);
    }
}
