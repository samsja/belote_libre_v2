use crate::card::{Card, Suit, Symbol};
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
    fn is_play_valid_suit_atout(
        suit_atout: &Suit,
        card_symbol: &Symbol,
        fold: &Fold,
        _hand: &Hand,
    ) -> bool {
        //User need to always go up when playing atout suit

        let played_atout: Vec<Symbol> = fold
            .cards
            .clone()
            .into_iter()
            .filter(|card| card.suit == *suit_atout)
            .map(|card| card.symbol)
            .collect();

        match played_atout.iter().min() {
            Some(min) => card_symbol >= min,
            None => true,
        }
    }

    fn is_partner_master_of_the_fold(_fold: &Fold) -> bool {
        true
        // need to implement
    }

    fn is_play_valid_atout(&self, suit_atout: Suit, card: &Card, fold: &Fold, hand: &Hand) -> bool {
        if hand.contains(card) {
            let fold_suit = fold.get_main_suit().unwrap();

            if hand.contains_suit(&fold_suit) {
                if card.suit == fold_suit {
                    if fold_suit == suit_atout {
                        DefaultRule::is_play_valid_suit_atout(&suit_atout, &card.symbol, fold, hand)
                    } else {
                        true
                    }
                } else {
                    false
                }
            } else {
                if hand.contains_suit(&suit_atout) {
                    if card.suit != suit_atout {
                        DefaultRule::is_partner_master_of_the_fold(fold)
                    } else {
                        DefaultRule::is_play_valid_suit_atout(&suit_atout, &card.symbol, fold, hand)
                    }
                } else {
                    true
                }
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{card, get_suit_shortcut, get_symbol_shortcut};
    use crate::card::{Card, Suit, Symbol};

    #[test]
    fn default_rule_empty_fold() {
        let fold = Fold::new_empty();
        let rule = DefaultRule {};
        let context = GameContext::SansAtout;
        let card_to_play = Card::new(Suit::Heart, Symbol::Seven);
        let hand = Hand::new(vec![card_to_play]);
        assert!(rule.is_play_valid(context, &card_to_play, &fold, &hand))
    }

    #[test]
    fn default_rule_empty_hand() {
        let fold = Fold::new_empty();
        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);
        let card_to_play = Card::new(Suit::Heart, Symbol::Seven);
        let hand = Hand::new_empty();
        assert!(!rule.is_play_valid(context, &card_to_play, &fold, &hand))
    }

    #[test]
    fn up_atout() {
        let mut fold = Fold::new_empty();
        fold.push(card!("D", "8"));
        let rule = DefaultRule {};
        let context = GameContext::Atout(Suit::Diamond);
        let hand = Hand::new(vec![card!("S", "7"), card!("D", "J"), card!("D", "7")]);

        let card_to_play = card!("D", "7");
        assert!(!rule.is_play_valid(context, &card_to_play, &fold, &hand));

        let card_to_play = card!("D", "J");
        assert!(rule.is_play_valid(context, &card_to_play, &fold, &hand));
    }
}
