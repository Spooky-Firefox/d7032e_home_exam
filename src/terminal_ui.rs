use crossbeam::channel::{self, Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::common::{DecisionChoice, UserStrategy};

use color_eyre::{Result, owo_colors::colors::css::Turquoise};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, List, ListState, Paragraph},
};

pub struct PlayerTUI {
    // Add fields as necessary
    thread: Option<JoinHandle<Result<(), PlayerTUIError>>>,
    send_event: Sender<SendToTUI>,
    receive_event: Receiver<ReceiveFromTUI>,
}
struct PlayerTUIThread {
    state: Arc<Mutex<hecs::World>>,
    player_list_state: ListState,
    player_actions: Vec<Box<dyn DecisionChoice>>,
    receive_event: Receiver<SendToTUI>,
    send_event: Sender<ReceiveFromTUI>,
}

enum SendToTUI {
    Quit,
    _RequestDecision(Vec<Box<dyn DecisionChoice>>),
    _ShowMessage(String),
}

enum ReceiveFromTUI {
    _DecisionMade(Box<dyn DecisionChoice>),
    _Quit,
}

enum PlayerTUIError {
    _Todo,
    ColorEyre(color_eyre::Report),
}

impl From<color_eyre::Report> for PlayerTUIError {
    fn from(err: color_eyre::Report) -> Self {
        PlayerTUIError::ColorEyre(err)
    }
}

impl PlayerTUIThread {
    fn new(
        state: Arc<Mutex<hecs::World>>,
    ) -> (
        Sender<SendToTUI>,
        Receiver<ReceiveFromTUI>,
        JoinHandle<Result<(), PlayerTUIError>>,
    ) {
        let (send_to_tui, receive_send_to_tui) = channel::unbounded::<SendToTUI>();
        let (send_to_game, receive_event) = channel::unbounded();
        let mut player_tui_thread = PlayerTUIThread {
            state,
            player_actions: Vec::new(),
            receive_event: receive_send_to_tui,
            send_event: send_to_game,
            player_list_state: ListState::default(),
        };
        let thread = thread::spawn(move || player_tui_thread.start());
        (send_to_tui, receive_event, thread)
    }

    fn start(&mut self) -> Result<(), PlayerTUIError> {
        // init ratatui terminal here
        color_eyre::install()?;
        let terminal = ratatui::init();
        let result = self.run(terminal);
        ratatui::restore();
        result
    }

    fn run(
        &mut self,
        mut term: ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), PlayerTUIError> {
        // Placeholder for the actual terminal UI loop
        loop {
            term.draw(|f| self.render(f)).unwrap();
            match self.receive_event.recv_timeout(Duration::from_millis(10)) {
                Ok(event) => match event {
                    SendToTUI::Quit => break,
                    SendToTUI::_RequestDecision(decisions) => {
                        self.player_actions = decisions;
                    }
                    SendToTUI::_ShowMessage(_message) => {
                        // Handle showing message
                    }
                },
                Err(channel::RecvTimeoutError::Timeout) => {
                    match crossterm::event::poll(Duration::from_secs(0)) {
                        Ok(false) => {}
                        Ok(true) => match crossterm::event::read().unwrap() {
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('q'),
                                ..
                            }) => break,
                            Event::Key(KeyEvent {
                                code: KeyCode::Down,
                                ..
                            }) => {
                                self.player_list_state.select_next();
                            }
                            Event::Key(KeyEvent {
                                code: KeyCode::Up, ..
                            }) => {
                                self.player_list_state.select_previous();
                            }
                            Event::Key(KeyEvent {
                                code: KeyCode::Enter,
                                ..
                            }) => {
                                if let Some(selected) = self.player_list_state.selected() {
                                    let choice = self.player_actions.swap_remove(selected);
                                    self.send_event
                                        .send(ReceiveFromTUI::_DecisionMade(choice))
                                        .unwrap();
                                }
                            }
                            _ => {}
                        },
                        Err(_) => break,
                    }
                } // Timeout occurred redraw UI
                Err(channel::RecvTimeoutError::Disconnected) => break, // Channel closed or timeout
            }
        }
        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .split(f.area());

        let choice_list = layout[1];
        let list = List::new(self.player_actions.iter().map(|action| action.name()))
            .block(Block::bordered().title("player actions"))
            .highlight_symbol("→")
            .highlight_style(Style::new().bold());
        f.render_stateful_widget(list, choice_list, &mut self.player_list_state);
    }
}

impl UserStrategy for PlayerTUI {
    fn get_user_decision(
        &self,
        _decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice> {
        self.send_event
            .send(SendToTUI::_RequestDecision(_decisions))
            .unwrap();
        match self.receive_event.recv().unwrap() {
            ReceiveFromTUI::_DecisionMade(choice) => choice,
            _ => panic!("Unexpected message received"),
        }
    }

    fn new(state: Arc<Mutex<hecs::World>>) -> Self {
        Self::new(state)
    }
}

impl PlayerTUI {
    pub fn new(state: Arc<Mutex<hecs::World>>) -> Self {
        let (send_event, receive_event, player_tui) = PlayerTUIThread::new(state);
        Self {
            thread: Some(player_tui),
            send_event,
            receive_event: receive_event,
        }
    }
}

/// when PlayerTUI goes out of scope, we want to make sure the thread is properly terminated
/// there exist a possibility that the thread have panic'ed, which would lead to an panic in drop
/// Hopefully the the given error message is enough to debug such an issue
impl Drop for PlayerTUI {
    fn drop(&mut self) {
        let _ = self.send_event.send(SendToTUI::Quit);
        let _ = self.thread.take().unwrap().join().unwrap();
        // Clean up terminal UI resources here
    }
}
