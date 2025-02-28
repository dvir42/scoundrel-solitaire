use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};
use strum::IntoEnumIterator;

pub struct State {
    pub health: u8,
    pub deck: Vec<Card>,
    pub open: [Card; 4],
}

fn random_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Rank::iter()
        .flat_map(|rank| {
            Suit::iter().map(move |suit| {
                if [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack].contains(&rank)
                    && [Suit::Hearts, Suit::Diamonds].contains(&suit)
                {
                    None
                } else {
                    Some(Card { rank, suit })
                }
            })
        })
        .flatten()
        .collect();
    let mut rng = rand::rng();
    cards.shuffle(&mut rng);
    cards
}

impl State {
    pub fn new() -> State {
        let mut deck = random_deck();
        let open = [
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
            deck.pop().unwrap(),
        ];
        State {
            health: 20,
            deck,
            open,
        }
    }
}
