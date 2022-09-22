use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::card::{Card, Suit, Symbol};
use crate::hand::Hand;

use itertools::iproduct;
use strum::IntoEnumIterator;

const MAX_CARDS_DECK: usize = 32;
const CARD_PER_HAND: usize = 8;
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Deck {
        Deck { cards }
    }

    fn new_empty() -> Deck {
        Deck::new(Deck::new_empty_vec_card())
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

    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::new_ordered();
        deck.shuffle();
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

    pub fn into_hands<'a>(&'a mut self) -> impl Iterator<Item = Hand> + 'a {
        let iter = self.cards.chunks(CARD_PER_HAND);
        iter.map(|x| Hand::new(x.to_vec()))
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
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
    }

    #[test]
    fn init_deck_shuffled() {
        let deck = Deck::new_shuffled();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
    }

    #[test]
    fn deck_shuffle() {
        let mut deck = Deck::new_ordered();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
        deck.shuffle();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
    }

    #[test]
    fn deck_cut() {
        let mut deck = Deck::new_ordered();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
        deck.shuffle_cut();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);
    }

    #[test]
    fn deck_to_hands() {
        let mut deck = Deck::new_ordered();
        assert_eq!(deck.cards.len(), MAX_CARDS_DECK);

        let hands = deck.into_hands().collect::<Vec<Hand>>();
        assert_eq!(hands.len(), 4); // there should be only 4 hands

        for hand_ in hands {
            assert_eq!(hand_.len(), 8); // there should be only 4 hands
        }
    }
}
