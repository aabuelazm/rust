pub mod card {
    pub use std::str::FromStr;

    #[repr(u16)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub enum Rank {
        Two = 0,
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

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Suit {
        Hearts,
        Diamonds,
        Clubs,
        Spades,
    }

    // Implementing the Card struct with method to parse through str to get card details
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Card {
        rank: Rank,
        suit: Suit,
    }

    impl FromStr for Card {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !(s.len() == 2 || s.len() == 3) {
                return Err("Input is not in correct format");
            }

            let rank: Rank;
            let mut ten_flag = false;

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
                '1' if s.chars().nth(1).unwrap() == '0' => {
                    rank = Rank::Ten;
                    ten_flag = true;
                }
                _ => return Err("Card rank is incorrect"),
            }

            let suit: Suit;

            match s.chars().nth(1 + ten_flag as usize).unwrap() {
                'H' => suit = Suit::Hearts,
                'D' => suit = Suit::Diamonds,
                'C' => suit = Suit::Clubs,
                'S' => suit = Suit::Spades,
                _ => return Err("Card Suit is incorrect"),
            }

            Ok(Card { rank, suit })
        } //from_str
    } // FromStr

    impl Card {
        pub fn get_rank(&self) -> Rank {
            return self.rank;
        }

        pub fn get_suit(&self) -> Suit {
            return self.suit;
        }
    }
}
