const MAX_PLAYER_ATTEMPT: usize = 10;

use crate::deck::Deck;
use crate::fold::Fold;
use crate::player::{BasicPlayer, Player};
use crate::rules::{GameContext, NoRule};

pub fn full_game() -> Vec<Fold> {
    let mut deck = Deck::new_shuffled();
    game(&mut deck)
}

pub fn game(deck: &mut Deck) -> Vec<Fold> {
    let players = deck
        .into_hands()
        .map(|hand| BasicPlayer::new(hand))
        .collect::<Vec<BasicPlayer>>();

    let max_folds = players[0].get_hand().len();

    let mut folds = Vec::<Fold>::with_capacity(max_folds);

    for _ in 0..max_folds {
        let mut current_fold = Fold::new(GameContext::ToutAtout, Box::new(NoRule {}));

        for player_ in &players {
            let mut valid_play = false;

            for _ in 0..MAX_PLAYER_ATTEMPT {
                if current_fold.is_play_valid(player_.play_card()) {
                    valid_play = true;
                    break;
                }
            }

            if !valid_play {
                panic!(
                    "Oh no a player played a wrong card for {} times",
                    MAX_PLAYER_ATTEMPT
                )
            }
        }

        folds.push(current_fold);
    }

    folds
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::concat;

    #[test]
    fn try_game() {
        let folds = full_game();
        assert_eq!(folds.len(), 8); // there should be 32/4 = 8 folds at the end

        for fold_ in folds {
            assert_eq!(fold_.len(), 4) // there should be only 4 card per fold
        }
    }

    #[test]
    fn chain_game() {
        let mut deck = Deck::new_shuffled();
        let mut folds = game(&mut deck);
        let cards_iter = folds.iter_mut().map(|fold| fold.get_cards());
        let mut new_deck = Deck::new(concat(cards_iter));
        new_deck.shuffle_cut();
        game(&mut new_deck);
    }
}
