mod card;
mod hand;
use hand::hand::*;
use std::str::FromStr;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut sorted_hands: Vec<(&'a str, Hand)> = Vec::new();

    for h in hands {
        sorted_hands.push((h, Hand::from_str(h).unwrap()));
    }

    sorted_hands.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let winning = sorted_hands.pop().unwrap();
    let mut output = vec![winning.0];

    let winning_hands = sorted_hands.iter().rev().take_while(|a| a.1 == winning.1);
    for h in winning_hands {
        output.push(h.0);
    }

    output
}
