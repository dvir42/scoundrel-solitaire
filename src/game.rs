use std::cmp::{max, min};

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
    pub used_heal: bool,
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
            used_heal: false,
        }
    }

    fn fight(&self, card: Card, use_weapon: bool) -> Option<State> {
        if ![Suit::Spades, Suit::Clubs].contains(&card.suit) {
            return None;
        }

        let health: isize;
        if !use_weapon || self.weapon.is_none() {
            health = self.health - card.rank.value();
        } else {
            health = self.health - max(card.rank.value() - self.weapon.unwrap().rank.value(), 0);
        }

        Some(State {
            health,
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
            used_heal: self.used_heal,
        })
    }

    fn heal(&self, card: Card) -> Option<State> {
        if !(card.suit == Suit::Hearts) {
            return None;
        }

        let new_health;
        if self.used_heal {
            new_health = self.health;
        } else {
            new_health = min(self.health + card.rank.value(), MAX_HEALTH);
        }

        Some(State {
            health: new_health,
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
            used_heal: true,
        })
    }

    fn equip_weapon(&self, card: Card) -> Option<State> {
        if !(card.suit == Suit::Diamonds) {
            return None;
        }

        Some(State {
            health: self.health,
            deck: self.deck.clone(),
            open: self.open,
            weapon: Some(card),
            used_heal: self.used_heal,
        })
    }

    pub fn play(&self, pos: usize, use_weapon: bool) -> Option<State> {
        if pos > 3 {
            return None;
        }
        if self.open[pos].is_none() {
            return None;
        }

        let card = self.open[pos].unwrap();
        let turn = match card.suit {
            Suit::Spades => self.fight(card, use_weapon),
            Suit::Hearts => self.heal(card),
            Suit::Diamonds => self.equip_weapon(card),
            Suit::Clubs => self.fight(card, use_weapon),
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
            ];
            new_state.used_heal = false;
        }

        Some(new_state)
    }
}
