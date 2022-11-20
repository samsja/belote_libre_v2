use crate::card::{Card, Suit};
use crate::fold::Fold;
use crate::hand::Hand;

#[derive(Debug, Copy, Clone)]
pub enum GameContext {
    ToutAtout,
    SansAtout,
    Atout(Suit),
}

pub trait Rule {
    fn is_play_valid(
        &self,
        context: GameContext,
        card_to_be_played: &Card,
        fold: &Fold,
        hand: &Hand,
    ) -> bool;
}

pub struct NoRule {}

impl Rule for NoRule {
    fn is_play_valid(
        &self,
        _context: GameContext,
        _card: &Card,
        _fold: &Fold,
        _hand: &Hand,
    ) -> bool {
        true
    }
}

pub struct DefaultRule {}

impl Rule for DefaultRule {
    //stil wip missing some rules
    fn is_play_valid(&self, context: GameContext, card: &Card, fold: &Fold, hand: &Hand) -> bool {
        match context {
            GameContext::Atout(suit) => self.is_play_valid_atout(suit, card, fold, hand),
            GameContext::SansAtout => true,
            GameContext::ToutAtout => true,
        }
    }
}

impl DefaultRule {
    fn is_play_valid_atout(&self, suit_atout: Suit, card: &Card, fold: &Fold, hand: &Hand) -> bool {
        if hand.contains(card) {
            let fold_suit = fold.get_main_suit().unwrap();
            if card.suit == fold_suit {
                true
            } else if card.suit == suit_atout {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Symbol};

    #[test]
    fn default_rule_empty_fold() {
        let fold = Fold::new();
        let rule = DefaultRule {};
        let context = GameContext::SansAtout;
        let card_to_play = Card::new(Suit::Heart, Symbol::Seven);
        let hand = Hand::new(vec![card_to_play]);
        assert!(rule.is_play_valid(context, &card_to_play, &fold, &hand))
    }

    #[test]
    fn default_rule_empty_hand() {
        let fold = Fold::new();
        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);
        let card_to_play = Card::new(Suit::Heart, Symbol::Seven);
        let hand = Hand::new_empty();
        assert!(!rule.is_play_valid(context, &card_to_play, &fold, &hand))
    }

    #[test]
    fn test_default_rule_main_color() {
        let mut fold = Fold::new();
        fold.push(Card::new(Suit::Heart, Symbol::Seven));

        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);

        for (card_to_play, valid) in vec![
            (Card::new(Suit::Heart, Symbol::Height), true),
            (Card::new(Suit::Spade, Symbol::Seven), false),
            (Card::new(Suit::Diamond, Symbol::Seven), true),
        ]
        .iter()
        {
            let hand = Hand::new(vec![*card_to_play]);

            let value = rule.is_play_valid(context, card_to_play, &fold, &hand);
            assert!(value == *valid);
        }
    }
}
