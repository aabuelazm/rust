//use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Implementing the Card struct with method to parse through str to get card details
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err("Input is not in correct format");
        }

        let rank: Rank;

        match s.chars().nth(0).unwrap() {
            'A' => rank = Rank::Ace,
            'K' => rank = Rank::King,
            'Q' => rank = Rank::Queen,
            'J' => rank = Rank::Jack,
            '9' => rank = Rank::Nine,
            '8' => rank = Rank::Eight,
            '7' => rank = Rank::Seven,
            '6' => rank = Rank::Six,
            '5' => rank = Rank::Five,
            '4' => rank = Rank::Four,
            '3' => rank = Rank::Three,
            '2' => rank = Rank::Two,
            _ => return Err("Card rank is incorrect"),
        }

        let suit: Suit;

        match s.chars().nth(1).unwrap() {
            'H' => suit = Suit::Hearts,
            'D' => suit = Suit::Diamonds,
            'C' => suit = Suit::Clubs,
            'S' => suit = Suit::Spades,
            _ => return Err("Card Suit is incorrect"),
        }

        Ok(Card { rank, suit })
    }
}

// Implementing the Hand struct with methods to parse and evaluate a vector of cards and their
// total score
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split_ascii_whitespace();
        let mut cards: Vec<Card> = Vec::new();

        for card in iter {
            cards.push(Card::from_str(card).unwrap());
        }

        if cards.len() != 5 {
            return Err("Hands can only contain 5 cards");
        }

        Ok(Hand { cards })
    }
}

impl Hand {
    pub fn highest_rank(&self) -> Rank {
        let mut highest = Rank::Two;

        for &card in self.cards.as_slice() {
            if card.rank > highest {
                highest = card.rank;
            }
        }

        highest
    }

    pub fn is_straight(&self) -> (bool, Rank) {
        let highest = self.highest_rank();

        (true, highest)
    }

    pub fn is_flush(&self) -> (bool, Rank, Rank, Rank, Rank, Rank) {
        unimplemented!("Not yet")
    }

    pub fn is_straight_flush(&self) -> (bool, Rank) {
        if self.is_straight().0 && self.is_flush().0 {
            (true, self.highest_rank())
        } else {
            (false, Rank::Two)
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
