use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use strum_macros::EnumIter;

pub const CARD_WIDTH: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Rank {
    Ace,
    Two,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Rank {
    pub fn symbol(self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    pub fn value(self) -> isize {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl Suit {
    pub fn symbol(self) -> &'static str {
        match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Suit::Spades => Color::White,
            Suit::Hearts => Color::Red,
            Suit::Diamonds => Color::Red,
            Suit::Clubs => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

fn double<'a>(suit: Suit, rank: Option<Rank>, additional_suit: bool, bottom: bool) -> Line<'a> {
    let mut string = "│".to_owned();
    string.push_str(&match rank {
        None => format!(
            "{}{}",
            if additional_suit { suit.symbol() } else { " " },
            " ".repeat(CARD_WIDTH / 2 - 2 - 1)
        ),
        Some(rank) => format!(
            "{}{}",
            rank.symbol(),
            " ".repeat(CARD_WIDTH / 2 - 2 - rank.symbol().len())
        ),
    });
    string.push_str(suit.symbol());
    string.push_str(&" ".repeat(3));
    string.push_str(suit.symbol());
    string.push_str(&" ".repeat(CARD_WIDTH / 2 - 2));
    string.push_str(&"│");
    if bottom {
        Line::from(Span::raw(string.chars().rev().collect::<String>()))
    } else {
        Line::from(Span::raw(string))
    }
}

fn single<'a>(suit: Suit, rank: Option<Rank>, additional_suit: bool, bottom: bool) -> Line<'a> {
    let mut string = "│".to_owned();
    string.push_str(&match rank {
        None => format!(
            "{}{}",
            if additional_suit { suit.symbol() } else { " " },
            " ".repeat(CARD_WIDTH / 2 - 1)
        ),
        Some(rank) => format!(
            "{}{}",
            rank.symbol(),
            " ".repeat(CARD_WIDTH / 2 - rank.symbol().len())
        ),
    });
    string.push_str(suit.symbol());
    string.push_str(&" ".repeat(CARD_WIDTH / 2));
    string.push_str(&"│");
    if bottom {
        Line::from(Span::raw(string.chars().rev().collect::<String>()))
    } else {
        Line::from(Span::raw(string))
    }
}

fn blank<'a>(suit: Suit, rank: Option<Rank>, additional_suit: bool, bottom: bool) -> Line<'a> {
    let mut string = "│".to_owned();
    string.push_str(&match rank {
        None => format!(
            "{}{}",
            if additional_suit { suit.symbol() } else { " " },
            " ".repeat(CARD_WIDTH - 1)
        ),
        Some(rank) => format!(
            "{}{}",
            rank.symbol(),
            " ".repeat(CARD_WIDTH - rank.symbol().len())
        ),
    });
    string.push_str(&"│");
    if bottom {
        Line::from(Span::raw(string.chars().rev().collect::<String>()))
    } else {
        Line::from(Span::raw(string))
    }
}

impl Card {
    pub fn face_down<'a>(self) -> Paragraph<'a> {
        let mut card = Vec::new();

        card.push(Line::from(Span::raw(format!(
            "╭{}╮",
            "─".repeat(CARD_WIDTH)
        ))));

        card.extend(vec![blank(self.suit, None, false, false); 7]);

        card.push(Line::from(Span::raw(format!(
            "╰{}╯",
            "─".repeat(CARD_WIDTH)
        ))));

        Paragraph::new(card)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
    }

    pub fn face_up<'a>(self) -> Paragraph<'a> {
        let mut card = Vec::new();

        card.push(Line::from(Span::raw(format!(
            "╭{}╮",
            "─".repeat(CARD_WIDTH)
        ))));

        card.extend(match self.rank {
            Rank::Two => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![single(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false); 3],
                vec![single(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Three => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![single(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Four => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![double(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false); 3],
                vec![double(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Five => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![double(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![double(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Six => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![double(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Seven => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![double(self.suit, None, true, false)],
                vec![single(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![double(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Eight => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![double(self.suit, None, true, false)],
                vec![single(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![double(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Nine => vec![
                vec![double(self.suit, Some(self.rank), false, false)],
                vec![blank(self.suit, None, true, false)],
                vec![double(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![blank(self.suit, None, true, true)],
                vec![double(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Ten => vec![
                vec![double(self.suit, Some(self.rank), false, false)],
                vec![single(self.suit, None, true, false)],
                vec![double(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![double(self.suit, None, false, false)],
                vec![single(self.suit, None, true, true)],
                vec![double(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Jack => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![blank(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![blank(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Queen => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![blank(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![blank(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::King => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![blank(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![blank(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
            Rank::Ace => vec![
                vec![blank(self.suit, Some(self.rank), false, false)],
                vec![blank(self.suit, None, true, false)],
                vec![blank(self.suit, None, false, false)],
                vec![single(self.suit, None, false, false)],
                vec![blank(self.suit, None, false, false)],
                vec![blank(self.suit, None, true, true)],
                vec![blank(self.suit, Some(self.rank), false, true)],
            ]
            .concat(),
        });

        card.push(Line::from(Span::raw(format!(
            "╰{}╯",
            "─".repeat(CARD_WIDTH)
        ))));

        Paragraph::new(card)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.suit.color()))
    }
}
