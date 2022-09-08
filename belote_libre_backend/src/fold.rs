use crate::card::Card;
use crate::rules::{GameContext, Rule};

const MAX_CARDS_FOLD: usize = 4;

pub struct Fold {
    cards: Vec<Card>,
    context: GameContext,
    rule: Box<dyn Rule>,
}

impl Fold {
    pub fn new(context: GameContext, rule: Box<dyn Rule>) -> Fold {
        let cards = Vec::with_capacity(MAX_CARDS_FOLD);
        Fold {
            cards,
            context,
            rule,
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_over(&self) -> bool {
        return self.cards.len() >= MAX_CARDS_FOLD;
    }

    pub fn is_play_valid(&mut self, card: Card) -> bool {
        if self.is_over() {
            return false;
        }

        let last_card = match self.cards.len() {
            0 => None,
            n => Some(self.cards[n - 1]),
        };

        if self
            .rule
            .allow_card_to_be_played(card, last_card, self.context)
        {
            self.cards.push(card);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Symbol};
    use crate::rules::NoRule;

    #[test]
    fn init_fold() {
        let _fold = Fold::new(GameContext::ToutAtout, Box::new(NoRule {}));
    }

    #[test]
    fn is_play_valid() {
        let mut fold = Fold::new(GameContext::ToutAtout, Box::new(NoRule {}));
        assert!(fold.is_play_valid(Card::new(Suit::Diamond, Symbol::Ten)));
        assert!(fold.is_play_valid(Card::new(Suit::Diamond, Symbol::Ace)));
    }

    #[test]
    fn play_too_much_card() {
        let mut fold = Fold::new(GameContext::ToutAtout, Box::new(NoRule {}));

        for _ in 0..MAX_CARDS_FOLD {
            assert!(fold.is_play_valid(Card::new(Suit::Diamond, Symbol::Ten)));
        }

        assert_eq!(
            fold.is_play_valid(Card::new(Suit::Diamond, Symbol::Ten)),
            false
        ); // 5th time should fail
    }
}
