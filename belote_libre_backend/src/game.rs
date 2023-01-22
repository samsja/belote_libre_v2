const MAX_PLAYER_ATTEMPT: usize = 10;

use crate::card::Card;
use crate::deck::Deck;
use crate::fold::Fold;
use crate::hand::Hand;
use crate::player::{BasicPlayer, Player};
use crate::rules::{GameContext, NoRule, Rule};

pub fn game() -> Vec<Fold> {
    let mut deck = Deck::new_shuffled();
    let rule = Box::new(NoRule {});
    game_wo_deck_init(&mut deck, &*rule)
}

pub fn game_wo_deck_init(deck: &mut Deck, rule: &dyn Rule) -> Vec<Fold> {
    let mut hands = deck.into_hands().collect::<Vec<Hand>>();
    let players = (0..hands.len())
        .into_iter()
        .map(|_| BasicPlayer::new())
        .collect::<Vec<BasicPlayer>>();

    let max_folds = hands[0].len();

    let mut folds = Vec::<Fold>::with_capacity(max_folds);
    let game_context = GameContext::ToutAtout;
    for _ in 0..max_folds {
        let mut current_fold = Fold::new_empty();

        for (player_, hand_) in players.iter().zip(hands.iter_mut()) {
            let mut card_played: Result<Card, &'static str> = Err("not initialized");

            for _ in 0..MAX_PLAYER_ATTEMPT {
                let need_break = false;
                card_played = match hand_.get_a_card(player_.play_card_id(&hand_)) {
                    Ok(card_played) => {
                        if rule.is_play_valid(game_context, &card_played, &current_fold, &hand_) {
                            Ok(card_played)
                        } else {
                            Err("card_played not valid")
                        }
                    }
                    Err(error) => {
                        panic!("{}", error); //should use logging
                    }
                };

                if need_break {
                    break;
                }
            }

            match card_played {
                Ok(card) => {
                    current_fold.push(card);
                }
                Err(_) => {
                    panic!(
                        "Oh no a player played a wrong card for {} times",
                        MAX_PLAYER_ATTEMPT
                    )
                }
            };
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
        let folds = game();
        assert_eq!(folds.len(), 8); // there should be 32/4 = 8 folds at the end

        for fold_ in folds {
            assert_eq!(fold_.len(), 4) // there should be only 4 card_playeds per fold
        }
    }

    #[test]
    fn chain_games() {
        let mut deck = Deck::new_shuffled();
        let mut folds = game_wo_deck_init(&mut deck, &*Box::new(NoRule {}));
        let card_playeds_iter = folds.iter_mut().map(|fold| fold.get_cards());
        let mut new_deck = Deck::new(concat(card_playeds_iter));
        new_deck.shuffle_cut();
        game_wo_deck_init(&mut new_deck, &*Box::new(NoRule {}));
    }
}
