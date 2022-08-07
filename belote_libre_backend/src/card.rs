use std::fmt;

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    symbol: Symbol,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_card() {
        let _card = Card::new(Suit::Diamond, Symbol::Ten);
        println!("{}", _card);
    }
}
