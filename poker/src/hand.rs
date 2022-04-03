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

            cards.sort_unstable_by(|x, y| x.get_rank().cmp(&y.get_rank()));

            Ok(Hand { cards })
        }
    }

    impl Hand {
        pub fn highest_rank(&self) -> Rank {
            self.cards.last().unwrap().get_rank()
        }

        pub fn is_straight(&self) -> (bool, Rank) {
            let highest = self.highest_rank();

            // checking for Five-High Straight
            if highest == Rank::Ace {
                for i in 0..4 {
                    if self.cards[i].get_rank() as u8 != Rank::Two as u8 + i as u8 {
                        break;
                    } else if i == 4 {
                        return (true, Rank::Five);
                    }
                }
            }

            for i in 0..4 {
                if self.cards[3 - i].get_rank() as u8 != highest as u8 - i as u8 {
                    break;
                } else if i == 4 {
                    return (true, highest);
                }
            }

            (false, Rank::Two)
        }

        pub fn is_flush(&self) -> bool {
            let mut iterator = self.cards.iter();
            let sample = iterator.next().unwrap().get_suit();

            while let Some(test) = iterator.next() {
                if test.get_suit() != sample {
                    return false;
                }
            }

            true
        }

        // To be done next
        fn cards_left_value(&self, i: usize) -> u16 {
            let mut sum = 0;

            if i == 5 {
                let cr = self.cards[4].get_rank() as u8 - 7;
                sum += ((((cr as f64).powi(4) + (cr as f64).powi(2)) / 24.0)
                    + ((cr as f64).powi(3) + (cr as f64) / 12.0)) as u16;
            }

            sum
        }

        pub fn evaluate(&self) -> u16 {
            match self.is_flush() {
                true => match self.is_straight() {
                    (true, r) => 7450 + r as u16,
                    (false, _) => self.cards_left_value(5),
                },
                false => {
                    5;
                    4
                }
            };
            unimplemented!("We'll get there")
        }
    }
}
