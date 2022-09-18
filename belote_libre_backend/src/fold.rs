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

    pub fn is_play_valid(&mut self, rule: &Box<dyn Rule>, card: Card) -> bool {
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

    // this function is needed other wise I cant create a static box and the ref passing dot not
    // work 
    fn _hack_static_box(rule: Box<dyn Rule>) -> Box<dyn Rule + 'static> {
        rule 
    }
    fn get_rule() -> Box<dyn Rule + 'static> { 
        _hack_static_box(Box::new(NoRule {})) 
    }

    #[test]
    fn init_fold() {
        let _fold = Fold::new(GameContext::ToutAtout);
    }

    #[test]
    fn is_play_valid() {
        let mut fold = Fold::new(GameContext::ToutAtout);
        assert!(fold.is_play_valid(&get_rule(), Card::new(Suit::Diamond, Symbol::Ten)));
        assert!(fold.is_play_valid(&get_rule(), Card::new(Suit::Diamond, Symbol::Ace)));
        assert_eq!(fold.len(), 2);
    }

    #[test]
    fn play_too_much_card() {
        let mut fold = Fold::new(GameContext::ToutAtout);

        for _ in 0..MAX_CARDS_FOLD {
            assert!(fold.is_play_valid(&get_rule(), Card::new(Suit::Diamond, Symbol::Ten)));
        }

        assert_eq!(
            fold.is_play_valid(&get_rule(), Card::new(Suit::Diamond, Symbol::Ten)),
            false
        ); // 5th time should fail
    }

    #[test]
    fn test_get_cards() {
        let mut fold = Fold::new(GameContext::ToutAtout);

        let card_ = Card::new(Suit::Diamond, Symbol::Ten);
        assert!(fold.is_play_valid(&get_rule(), card_));
        assert_eq!(fold.len(), 1);

        let cards = fold.get_cards();

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], card_);
    }
}
