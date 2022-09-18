use crate::hand::Hand;

pub trait Player {
    fn play_card_id(&self, hand: &Hand) -> usize;
}

pub struct BasicPlayer {}

impl BasicPlayer {
    pub fn new() -> BasicPlayer {
        BasicPlayer {}
    }
}

impl Player for BasicPlayer {
    fn play_card_id(&self, _hand: &Hand) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Symbol};

    #[test]
    fn init_player() {
        let mut hand = Hand::new_empty();
        let init_card = Card::new(Suit::Diamond, Symbol::Ten);

        hand.push(init_card);

        let player = BasicPlayer::new();
        player.play_card_id(&hand);
    }
}
