#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}
use crate::Card::*;

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let card_map: HashMap<Card, _> = HashMap::from([
            (Ace, 11),
            (Two, 2),
            (Three, 3),
            (Four, 4),
            (Five, 5),
            (Six, 6),
            (Seven, 7),
            (Eight, 8),
            (Nine, 9),
            (Jack, 10),
            (King, 10),
            (Queen, 10),
        ]);

        let mut val = 0;
        for card in &self.cards {
            if card == &Ace {
                val += if val > 11 { 1 } else { 11 }
            } else {
                val += card_map.get(card).unwrap();
            }
        }

        val
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
    println!("{}", hand.value())
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
