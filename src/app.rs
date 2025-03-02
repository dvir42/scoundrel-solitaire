use crate::game;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
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

        let status = Line::from(vec![
            " Health ".into(),
            current_state.health.to_string().green().bold(),
            " | Used heal ".into(),
            current_state.used_heal.to_string().bold(),
            " | Deck ".into(),
            current_state.deck.len().to_string().bold(),
            " | Using weapon ".into(),
            state.use_weapon.to_string().bold(),
            " ".into(),
        ])
        .left_aligned();
        let instructions = Line::from(vec![
            " Toggle Use Weapon ".into(),
            "<W>".blue().bold(),
            " | Undo ".into(),
            "<U>".blue().bold(),
            " | Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .right_aligned();

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(status)
            .title_bottom(instructions)
            .border_set(border::THICK);

        let inner_area = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(2)
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 2); 2])
            .split(area);

        let room_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Percentage(20),
            ])
            .split(inner_area[0]);

        match current_state.deck.first() {
            None => (),
            Some(card) => card.face_down().render(room_area[1], buf),
        };

        for (i, card) in current_state.open.iter().enumerate() {
            match card {
                None => continue,
                Some(c) => c.face_up().render(room_area[i + 2], buf),
            }
        }

        match current_state.weapon {
            None => (),
            Some(c) => c.face_up().render(inner_area[1], buf),
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
