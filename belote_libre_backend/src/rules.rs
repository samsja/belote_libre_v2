use crate::card::{Card, Suit};
use crate::fold::Fold;

#[derive(Debug, Copy, Clone)]
pub enum GameContext {
    ToutAtout,
    SantAtout,
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
