use crate::deck::Deck;
use crate::fold::Fold;
use crate::player::{BasicPlayer, Player};
use crate::rules::{GameContext, NoRule};

pub fn game() -> Vec<Fold> {
    let mut deck = Deck::new_shuffled();

    let players = deck
        .into_hands()
        .map(|hand| BasicPlayer::new(hand))
        .collect::<Vec<BasicPlayer>>();

    let max_folds = players[0].get_hand().len();

    let mut folds = Vec::<Fold>::with_capacity(max_folds);

    for _ in 0..max_folds {
        let mut current_fold = Fold::new(GameContext::ToutAtout, Box::new(NoRule {}));

        for player_ in &players {
            if current_fold.is_play_valid(player_.play_card()) {
            } else {
                panic!("Oh no a player played a wrong card")
            }
        }

        folds.push(current_fold);
    }

    folds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_game() {
        let folds = game();
        assert_eq!(folds.len(), 8); // there should be 32/4 = 8 folds at the end

        for fold_ in folds {
            assert_eq!(fold_.len(), 4) // there should be only 4 card per fold
        }
    }
}
