use std::mem;

use crate::card::Card;
use crate::rules::{GameContext, Rule};

const MAX_CARDS_FOLD: usize = 4;

pub struct Fold {
    cards: Vec<Card>,
    context: GameContext,
}

impl Fold {
    pub fn new(context: GameContext) -> Fold {
        let cards = Vec::with_capacity(MAX_CARDS_FOLD);
        Fold { cards, context }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_over(&self) -> bool {
        return self.cards.len() >= MAX_CARDS_FOLD;
    }
    // &dyn Rule is used as suggest here :
    // https://users.rust-lang.org/t/creating-reference-of-box/81413/26
    pub fn is_play_valid(&mut self, rule: &dyn Rule, card: Card) -> bool {
        if self.is_over() {
            return false;
        }

        let last_card = match self.cards.len() {
            0 => None,
            n => Some(self.cards[n - 1]),
        };

        if rule.allow_card_to_be_played(card, last_card, self.context) {
            self.cards.push(card);
            true
        } else {
            false
        }
    }

    pub fn get_cards(&mut self) -> Vec<Card> {
        mem::replace(&mut self.cards, Vec::with_capacity(MAX_CARDS_FOLD))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Symbol};
    use crate::rules::NoRule;

    #[test]
    fn init_fold() {
        let _fold = Fold::new(GameContext::ToutAtout);
    }

    #[test]
    fn is_play_valid() {
        let mut fold = Fold::new(GameContext::ToutAtout);
        assert!(fold.is_play_valid(&*Box::new(NoRule {}), Card::new(Suit::Diamond, Symbol::Ten)));
        assert!(fold.is_play_valid(&*Box::new(NoRule {}), Card::new(Suit::Diamond, Symbol::Ace)));
        assert_eq!(fold.len(), 2);
    }

    #[test]
    fn play_too_much_card() {
        let mut fold = Fold::new(GameContext::ToutAtout);

        for _ in 0..MAX_CARDS_FOLD {
            assert!(
                fold.is_play_valid(&*Box::new(NoRule {}), Card::new(Suit::Diamond, Symbol::Ten))
            );
        }

        assert_eq!(
            fold.is_play_valid(&*Box::new(NoRule {}), Card::new(Suit::Diamond, Symbol::Ten)),
            false
        ); // 5th time should fail
    }

    #[test]
    fn test_get_cards() {
        let mut fold = Fold::new(GameContext::ToutAtout);

        let card_ = Card::new(Suit::Diamond, Symbol::Ten);
        assert!(fold.is_play_valid(&*Box::new(NoRule {}), card_));
        assert_eq!(fold.len(), 1);

        let cards = fold.get_cards();

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], card_);
    }
}
