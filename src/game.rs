use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};
use strum::IntoEnumIterator;

const MAX_HEALTH: isize = 20;

#[derive(Debug)]
pub struct State {
    played_in_room: usize,
    played: Option<Card>,
    pub health: isize,
    pub used_heal: bool,
    pub deck: VecDeque<Card>,
    pub open: [Option<Card>; 4],
    pub weapon: Option<Card>,
    pub killed_with_weapon: Vec<Card>,
    pub can_run: bool,
    pub game_over: bool,
}

fn random_deck() -> VecDeque<Card> {
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
    VecDeque::from(cards)
}

impl State {
    pub fn new() -> State {
        let mut deck = random_deck();
        let open = [
            deck.pop_front(),
            deck.pop_front(),
            deck.pop_front(),
            deck.pop_front(),
        ];
        State {
            played_in_room: 0,
            played: None,
            health: MAX_HEALTH,
            used_heal: false,
            deck,
            open,
            weapon: None,
            killed_with_weapon: Vec::new(),
            can_run: true,
            game_over: false,
        }
    }

    fn fight(&self, card: Card, use_weapon: bool) -> Option<State> {
        if ![Suit::Spades, Suit::Clubs].contains(&card.suit) {
            return None;
        }

        let can_use_weapon = match self.killed_with_weapon.last() {
            None => true,
            Some(last_killed) => card.rank.value() <= last_killed.rank.value(),
        };

        let health: isize;
        let mut killed_with_weapon = self.killed_with_weapon.clone();
        if !use_weapon || self.weapon.is_none() || !can_use_weapon {
            health = self.health - card.rank.value();
        } else {
            health = self.health - max(card.rank.value() - self.weapon.unwrap().rank.value(), 0);
            killed_with_weapon.push(card);
        }

        Some(State {
            played_in_room: self.played_in_room,
            played: Some(card),
            health,
            used_heal: self.used_heal,
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
            killed_with_weapon,
            can_run: self.can_run,
            game_over: self.game_over,
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
            played_in_room: self.played_in_room,
            played: Some(card),
            health: new_health,
            used_heal: true,
            deck: self.deck.clone(),
            open: self.open,
            weapon: self.weapon,
            killed_with_weapon: self.killed_with_weapon.clone(),
            can_run: self.can_run,
            game_over: self.game_over,
        })
    }

    fn equip_weapon(&self, card: Card) -> Option<State> {
        if !(card.suit == Suit::Diamonds) {
            return None;
        }

        Some(State {
            played_in_room: self.played_in_room,
            played: Some(card),
            health: self.health,
            used_heal: self.used_heal,
            deck: self.deck.clone(),
            open: self.open,
            weapon: Some(card),
            killed_with_weapon: Vec::new(),
            can_run: self.can_run,
            game_over: self.game_over,
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
        let action = match card.suit {
            Suit::Spades => self.fight(card, use_weapon),
            Suit::Hearts => self.heal(card),
            Suit::Diamonds => self.equip_weapon(card),
            Suit::Clubs => self.fight(card, use_weapon),
        };

        if action.is_none() {
            return None;
        }

        let mut new_state = action.unwrap();
        new_state.open[pos] = None;

        new_state.played_in_room += 1;

        if new_state.played_in_room == 3 {
            new_state.open = [
                new_state.open.iter().flatten().last().copied(),
                new_state.deck.pop_front(),
                new_state.deck.pop_front(),
                new_state.deck.pop_front(),
            ];
            new_state.played_in_room = 0;
            new_state.used_heal = false;
            new_state.can_run = true;
        } else {
            new_state.can_run = false;
        }

        if new_state.health <= 0
            || (new_state.deck.len() <= 0 && new_state.open.iter().all(|c| c.is_none()))
        {
            new_state.game_over = true;
        }

        Some(new_state)
    }

    pub fn run(&self) -> Option<State> {
        if !self.can_run {
            return None;
        }

        let mut deck = self.deck.clone();
        for card in self.open {
            match card {
                None => continue,
                Some(c) => deck.push_back(c),
            };
        }
        let open = [
            deck.pop_front(),
            deck.pop_front(),
            deck.pop_front(),
            deck.pop_front(),
        ];
        Some(State {
            played_in_room: 0,
            played: None,
            health: self.health,
            used_heal: self.used_heal,
            deck,
            open,
            weapon: self.weapon,
            killed_with_weapon: self.killed_with_weapon.clone(),
            can_run: false,
            game_over: self.game_over,
        })
    }

    pub fn score(&self) -> isize {
        if self.health <= 0 {
            [
                self.deck.iter().collect::<Vec<&Card>>(),
                self.open.iter().flatten().collect(),
            ]
            .concat()
            .iter()
            .fold(self.health, |acc, c| {
                if [Suit::Spades, Suit::Clubs].contains(&c.suit) {
                    acc - c.rank.value()
                } else {
                    acc
                }
            })
        } else {
            if self.health < 20 {
                self.health
            } else {
                20 + match self.played.unwrap().suit {
                    Suit::Hearts => self.played.unwrap().rank.value(),
                    _ => 0,
                }
            }
        }
    }
}
