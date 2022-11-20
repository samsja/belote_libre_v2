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
    use crate::card::{card, get_suit_shortcut, get_symbol_shortcut};
    
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

    fn default_rule_main_color_helper(card: &Card, valid: bool) {
        let mut fold = Fold::new();
        fold.push(Card::new(Suit::Heart, Symbol::Seven));

        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);

        let hand = Hand::new(vec![*card]);

        let value = rule.is_play_valid(context, card, &fold, &hand);
        assert!(value == valid);
    }

    macro_rules! main_color_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {

                let (card, valid) = $value;
                default_rule_main_color_helper(&card, valid);
            }
        )*
        }
    }

    main_color_tests! {
        h8: (card!("H", "8"), true),
        s7: (Card::new(Suit::Spade, Symbol::Seven), false),
        d7: (Card::new(Suit::Diamond, Symbol::Seven), true),
    }
}
