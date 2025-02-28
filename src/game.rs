use std::cmp::min;

use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};
use strum::IntoEnumIterator;

const MAX_HEALTH: isize = 20;

#[derive(Debug)]
pub struct State {
    pub health: isize,
    pub deck: Vec<Card>,
    pub open: [Option<Card>; 4],
    pub weapon: Option<Card>,
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
        let open = [deck.pop(), deck.pop(), deck.pop(), deck.pop()];
        State {
            health: MAX_HEALTH,
            deck,
            open,
            weapon: None,
        }
    }

    fn fight(&self, card: Card) -> Option<State> {
        if ![Suit::Spades, Suit::Clubs].contains(&card.suit) {
            return None;
        }
        let health: isize;
        match self.weapon {
            None => health = self.health - card.rank.value() as isize,
            Some(weapon) => {
                health = self.health - (card.rank.value() + weapon.rank.value()) as isize
            }
        }
        Some(State {
            health,
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
        })
    }

    fn heal(&self, card: Card) -> Option<State> {
        if !(card.suit == Suit::Hearts) {
            return None;
        }

        Some(State {
            health: min(self.health + card.rank.value() as isize, MAX_HEALTH),
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
        })
    }

    pub fn play(&self, pos: usize) -> Option<State> {
        if pos > 3 {
            return None;
        }
        if self.open[pos].is_none() {
            return None;
        }

        let card = self.open[pos].unwrap();
        let turn = match card.suit {
            Suit::Spades => self.fight(card),
            Suit::Hearts => self.heal(card),
            Suit::Diamonds => None,
            Suit::Clubs => self.fight(card),
        };

        if turn.is_none() {
            return None;
        }

        let mut new_state = turn.unwrap();
        new_state.open[pos] = None;

        if new_state.open.iter().flatten().count() == 1 {
            new_state.open = [
                new_state.open.iter().flatten().last().copied(),
                new_state.deck.pop(),
                new_state.deck.pop(),
                new_state.deck.pop(),
            ]
        }

        Some(new_state)
    }
}
