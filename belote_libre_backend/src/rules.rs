use crate::card::{Card, Suit};
use crate::fold::Fold;

#[derive(Debug, Copy, Clone)]
pub enum GameContext {
    ToutAtout,
    SansAtout,
    Atout(Suit),
}

pub trait Rule {
    fn is_play_valid(&self, context: GameContext, card_to_be_played: Card, fold: &Fold) -> bool;
}

pub struct NoRule {}

impl Rule for NoRule {
    fn is_play_valid(&self, _context: GameContext, _card: Card, _fold: &Fold) -> bool {
        true
    }
}

pub struct DefaultRule {}

impl Rule for DefaultRule {
    //stil wip missing some rules
    fn is_play_valid(&self, context: GameContext, card: Card, fold: &Fold) -> bool {
        match context {
            GameContext::Atout(suit) => self.is_play_valid_atout(suit, card, fold),
            GameContext::SansAtout => true,
            GameContext::ToutAtout => true,
        }
    }
}

impl DefaultRule {
    fn is_play_valid_atout(&self, suit_atout: Suit, card: Card, fold: &Fold) -> bool {
        let fold_suit = fold.get_main_suit();

        match fold_suit {
            Ok(suit) => {
                if card.suit == suit {
                    true
                } else if card.suit == suit_atout {
                    true
                } else {
                    false
                }
            }
            Err(_) => true, // if the fold is empty then you can play your card
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
        assert!(rule.is_play_valid(context, Card::new(Suit::Heart, Symbol::Seven), &fold))
    }

    #[test]
    fn test_default_rule_main_color() {
        let mut fold = Fold::new();
        fold.push(Card::new(Suit::Heart, Symbol::Seven));

        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);

        assert!(rule.is_play_valid(context, Card::new(Suit::Heart, Symbol::Height), &fold));
        assert!(!rule.is_play_valid(context, Card::new(Suit::Spade, Symbol::Seven), &fold));
        assert!(rule.is_play_valid(context, Card::new(Suit::Diamond, Symbol::Seven), &fold));
    }
}
