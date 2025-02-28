use crate::game;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use std::io;

pub struct State {
    turns: Vec<game::State>,
    use_weapon: bool,
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl StatefulWidget for &mut App {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let current_state = state.turns.last().unwrap();
        let title = Line::from(" Scoundrel ".bold());
        let instructions = Line::from(vec![
            " Health ".into(),
            current_state.health.to_string().green().bold(),
            " Used heal ".into(),
            current_state.used_heal.to_string().bold(),
            " Deck ".into(),
            current_state.deck.len().to_string().bold(),
            " Using weapon ".into(),
            state.use_weapon.to_string().bold(),
            " Toggle Use Weapon ".into(),
            "<T>".blue().bold(),
            " Undo ".into(),
            "<U>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let inner_area = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(20),
            ])
            .split(inner_area);

        match current_state.deck.first() {
            None => (),
            Some(card) => card.face_down().render(chunks[1], buf),
        };

        for (i, card) in current_state.open.iter().enumerate() {
            match card {
                None => continue,
                Some(c) => c.face_up().render(chunks[i + 2], buf),
            }
        }
        block.render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut state = State {
            turns: vec![game::State::new()],
            use_weapon: true,
        };
        while !self.exit {
            terminal.draw(|frame| self.draw(frame, &mut state))?;
            self.handle_events(&mut state)?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, state: &mut State) {
        frame.render_stateful_widget(self, frame.area(), state);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('u') => {
                if state.turns.len() > 1 {
                    state.turns.pop();
                }
            }
            KeyCode::Char('w') => {
                state.use_weapon = !state.use_weapon;
            }
            KeyCode::Char('1') => {
                state
                    .turns
                    .last()
                    .unwrap()
                    .play(0, state.use_weapon)
                    .map(|s| state.turns.push(s));
            }
            KeyCode::Char('2') => {
                state
                    .turns
                    .last()
                    .unwrap()
                    .play(1, state.use_weapon)
                    .map(|s| state.turns.push(s));
            }
            KeyCode::Char('3') => {
                state
                    .turns
                    .last()
                    .unwrap()
                    .play(2, state.use_weapon)
                    .map(|s| state.turns.push(s));
            }
            KeyCode::Char('4') => {
                state
                    .turns
                    .last()
                    .unwrap()
                    .play(3, state.use_weapon)
                    .map(|s| state.turns.push(s));
            }
            _ => {}
        }
    }

    fn handle_events(&mut self, state: &mut State) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, state)
            }
            _ => {}
        };
        Ok(())
    }
}
