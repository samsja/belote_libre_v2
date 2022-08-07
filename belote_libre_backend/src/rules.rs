use crate::card::{Card, Suit};

#[derive(Debug, Copy, Clone)]
pub enum GameContext {
    ToutAtout,
    SantAtout,
    Atout(Suit),
}

pub trait Rule {
    fn allow_card_to_be_played(
        &self,
        card_to_be_played: Card,
        last_card_played: Option<Card>,
        context: GameContext,
    ) -> bool;
}

pub struct NoRule {}

impl Rule for NoRule {
    fn allow_card_to_be_played(&self, _c: Card, _c2: Option<Card>, _context: GameContext) -> bool {
        true
    }
}
