use crate::card::Card;
use crate::hand::Hand;

pub trait Player {
    fn play_card_id(&self) -> usize;
    fn get_hand<'a>(&'a self) -> &'a Hand;
    fn play_card(&self) -> Card {
        self.get_hand()[self.play_card_id()]
    }
}



pub struct BasicPlayer {
    hand: Hand,
}

impl BasicPlayer {
    pub fn new(hand: Hand) -> BasicPlayer {
        BasicPlayer { hand }
    }
}

impl Player for BasicPlayer {
    fn play_card_id(&self) -> usize {
        0
    }

    fn get_hand<'a>(&'a self) -> &'a Hand {
        &self.hand
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

        let player = BasicPlayer::new(hand);
        let played_card = player.play_card();

        assert_eq!(init_card, played_card);
    }
}
