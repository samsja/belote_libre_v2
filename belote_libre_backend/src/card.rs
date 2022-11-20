use std::collections::HashMap;
use std::fmt;

use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    Seven,
    Height,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub symbol: Symbol,
}

impl Card {
    pub fn new(suit: Suit, symbol: Symbol) -> Card {
        Card { suit, symbol }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.symbol, self.suit)
    }
}

pub fn get_suit_shortcut() -> HashMap<String, Suit> {
    HashMap::from([
        ("H".to_string(), Suit::Heart),
        ("D".to_string(), Suit::Diamond),
        ("C".to_string(), Suit::Club),
        ("S".to_string(), Suit::Spade),
    ])
}

pub fn get_symbol_shortcut() -> HashMap<String, Symbol> {
    HashMap::from([
        ("7".to_string(), Symbol::Seven),
        ("8".to_string(), Symbol::Height),
        ("9".to_string(), Symbol::Nine),
        ("10".to_string(), Symbol::Ten),
        ("J".to_string(), Symbol::Jack),
        ("Q".to_string(), Symbol::Queen),
        ("K".to_string(), Symbol::King),
        ("A".to_string(), Symbol::Ace),
    ])
}

#[macro_export]
macro_rules! card {
    ($a:expr,$b:expr) => {{
        Card::new(
            get_suit_shortcut()[&$a.to_string()],
            get_symbol_shortcut()[&$b.to_string()],
        )
    }};
}

pub(crate) use card;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_card() {
        let _card = Card::new(Suit::Diamond, Symbol::Ten);
        println!("{}", _card);
    }

    #[test]
    fn init_card_macro() {
        let card = Card::new(Suit::Heart, Symbol::Seven);
        let card2 = card!("H", "7");

        assert!(card == card2);
    }
}
