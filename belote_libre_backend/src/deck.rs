use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::card::{Card, Suit, Symbol};

use itertools::iproduct;
use strum::IntoEnumIterator;

use std::mem;

const MAX_CARDS_DECK: usize = 42;

#[derive(Debug)]
pub struct Deck {
    #[allow(dead_code)]
    cards: Vec<Card>,
}

impl Deck {
    fn new_empty() -> Deck {
        Deck {
            cards: Deck::new_empty_vec_card(),
        }
    }

    fn new_empty_vec_card() -> Vec<Card> {
        Vec::<Card>::with_capacity(MAX_CARDS_DECK)
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

    pub fn shuffle_cut(&mut self) {
        // todo implement cut
        let mut rng = thread_rng();
        let max_indice = self.cards.len() - 1;
        let cut_indice = rng.gen_range(1..(max_indice - 1)); // it should not be the first or last

        let mut top_deck = self.cards.split_off(cut_indice);

        top_deck.append(&mut self.cards);
        self.cards = top_deck;
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

    #[test]
    fn deck_cut() {
        let mut deck = Deck::new_ordered();
        assert_eq!(deck.cards.len(), 32);
        deck.shuffle_cut();
        assert_eq!(deck.cards.len(), 32);
    }
}
