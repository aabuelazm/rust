pub mod hand {
    use crate::card::card::*;
    use std::cmp::Ordering;
    use std::collections::HashSet;

    enum RankHandRanks {
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
    }

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
        fn highest_rank(&self) -> Rank {
            self.cards.last().unwrap().get_rank()
        }

        fn is_straight(&self) -> (bool, Rank) {
            let highest = self.highest_rank();

            // checking for Five-High Straight
            if highest == Rank::Ace {
                for i in 0..4 {
                    if self.cards[i].get_rank() as u16 != Rank::Two as u16 + i as u16 {
                        break;
                    } else if i == 4 {
                        return (true, Rank::Five);
                    }
                }
            }

            for i in 0..4 {
                if self.cards[3 - i].get_rank() as u16 != highest as u16 - i as u16 {
                    break;
                } else if i == 4 {
                    return (true, highest);
                }
            }

            (false, highest)
        }

        fn is_flush(&self) -> bool {
            let mut iterator = self.cards.iter();
            let sample = iterator.next().unwrap().get_suit();

            while let Some(test) = iterator.next() {
                if test.get_suit() != sample {
                    return false;
                }
            }

            true
        }

        fn high_card(&self, high_rank: Rank) -> u16 {
            let random_array = [493, 164, 120, 84, 56, 35, 20, 10, 4, 1, 0];
            let mut result = 0;
            let mut last_rank = 12 - high_rank as u16 as usize;

            for i in 0..last_rank {
                result += random_array[i];
            }

            for i in 1..=4 {
                last_rank = 12 - last_rank - self.cards[i].get_rank() as u16 as usize;
                for j in i..last_rank {
                    result += random_array[j];
                }
            }

            result
        }

        fn find_rank_patterns(&self) -> Option<RankHandRanks> {
            let mut hashed_ranks = HashSet::new();

            for c in self.cards.iter() {
                hashed_ranks.insert(c.get_rank());
            }

            match hashed_ranks.len() {
                2 => {
                    let test_rank = self.cards[0].get_rank();
                    let count_this = self.cards.iter().take_while(|x| x.get_rank() == test_rank);

                    match count_this.count() {
                        1 | 4 => Some(RankHandRanks::FourOfAKind),
                        2 | 3 => Some(RankHandRanks::FullHouse),
                        _ => panic!("How??"),
                    }
                }

                3 => {
                    let test_rank = self.cards[0].get_rank();
                    let count_this = self.cards.iter().take_while(|x| x.get_rank() == test_rank);

                    match count_this.count() {
                        3 => Some(RankHandRanks::ThreeOfAKind),
                        2 => Some(RankHandRanks::TwoPair),

                        1 => {
                            let test2 = self.cards[4].get_rank();
                            let count2 = self
                                .cards
                                .iter()
                                .rev()
                                .take_while(|x| x.get_rank() == test2);

                            match count2.count() {
                                1 | 3 => Some(RankHandRanks::ThreeOfAKind),
                                2 => Some(RankHandRanks::TwoPair),
                                _ => panic!("How??"),
                            }
                        }

                        _ => panic!("How??"),
                    }
                }

                4 => Some(RankHandRanks::OnePair),
                _ => None,
            }
        }

        pub fn evaluate(&self) -> u16 {
            let score = 7462;

            match (self.is_flush(), self.is_straight()) {
                (true, (true, r)) => return score + (r as u16) - 12,
                (true, (false, _)) => return score - 322 - self.high_card(self.highest_rank()),
                (false, (true, r)) => return score + (r as u16) - 1599 - 12,
                (false, (false, _)) => match self.find_rank_patterns() {
                    Some(RankHandRanks::FourOfAKind) => {}
                    Some(RankHandRanks::FullHouse) => {}
                    Some(RankHandRanks::ThreeOfAKind) => {}
                    Some(RankHandRanks::TwoPair) => {}
                    Some(RankHandRanks::OnePair) => {}
                    None => return score - 6185 - self.high_card(self.highest_rank()),
                },
            };

            score
        }
    }
}
