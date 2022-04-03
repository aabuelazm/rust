pub mod hand {
    use crate::card::card::*;
    use std::cmp::Ordering;

    // Implementing the Hand struct with methods to parse and evaluate a vector of cards and their
    // total score
    #[derive(Debug, Clone, Eq)]
    pub struct Hand {
        cards: Vec<Card>,
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.evaluate() == other.evaluate()
        }

        fn ne(&self, other: &Self) -> bool {
            !(self.eq(other))
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.evaluate().cmp(&other.evaluate())
        }
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

    impl fmt::Display for Hand {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.cards.len() != 5 {
                return Err(fmt::Error);
            }

            for i in 0..4 {
                write!(f, "{} ", self.cards[i]);
            }

            write!(f, "{}", self.cards[4])
        }
    }

    impl Hand {
        pub fn highest_rank(&self) -> Rank {
            let mut highest = Rank::Two;

            for &card in self.cards.iter() {
                if card.rank > highest {
                    highest = card.rank;
                }
            }

            highest
        }

        pub fn is_straight(&self) -> (bool, Rank) {
            // let highest = self.highest_rank();

            unimplemented!("Not yet")
        }

        pub fn is_flush(
            &self,
        ) -> (
            bool,
            Option<Rank>,
            Option<Rank>,
            Option<Rank>,
            Option<Rank>,
            Option<Rank>,
        ) {
            unimplemented!("Not yet")
        }

        pub fn is_straight_flush(&self) -> (bool, Rank) {
            if self.is_straight().0 && self.is_flush().0 {
                (true, self.highest_rank())
            } else {
                (false, Rank::Two)
            }
        }

        pub fn evaluate(&self) -> u16 {
            unimplemented!("We'll get there")
        }
    }
}
