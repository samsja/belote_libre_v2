const _NUM_CARDS_FOLD: usize = 4;
const _NUM_PLAYER: usize = 4;
const MAX_FOLDS: usize = 8;

use crate::deck::{Deck};
use crate::hand::{Hand};
use crate::fold::{Fold};

pub fn game(){
    
    let mut deck = Deck::new_shuffled();

    let _hands = deck.into_hands().collect::<Vec<Hand>>();
    
    let _folds = Vec::<Fold>::with_capacity(MAX_FOLDS);
    

}
