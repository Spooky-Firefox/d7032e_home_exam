use crossbeam::channel::{self, Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::common::{DecisionChoice, UserStrategy};

use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout}, style::{Style, Stylize}, widgets::{Block, List, ListState, Paragraph}, Frame
};

pub struct PlayerTUI {
    // Add fields as necessary
    thread: Option<JoinHandle<Result<(), PlayerTUIError>>>,
    send_event: Sender<SendToTUI>,
    receive_event: Receiver<ReceiveFromTUI>,
}

impl UserStrategy for PlayerTUI {
    fn get_user_decision(
        &self,
        _decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice> {
        self.send_event
            .send(SendToTUI::RequestDecision(_decisions))
            .unwrap();
        match self.receive_event.recv().unwrap() { // this unwrap will cause panic if the thread has panicked or closed
            ReceiveFromTUI::DecisionMade(choice) => choice,
            _ => panic!("Unexpected message received"),
        }
    }

    fn new(state: Arc<Mutex<hecs::World>>) -> Self {
        Self::new(state)
    }
    
    fn send_message(&self, message: String) {
        self.send_event
            .send(SendToTUI::ShowMessage(message))
            .unwrap();
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

struct PlayerTUIThread {
    _state: Arc<Mutex<hecs::World>>,
    player_list_state: ListState,
    player_actions: Vec<Box<dyn DecisionChoice>>,
    receive_event: Receiver<SendToTUI>,
    send_event: Sender<ReceiveFromTUI>,
    messages: Vec<String>,
}

enum SendToTUI {
    Quit,
    RequestDecision(Vec<Box<dyn DecisionChoice>>),
    ShowMessage(String),
}

enum ReceiveFromTUI {
    DecisionMade(Box<dyn DecisionChoice>),
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
            _state: state,
            player_actions: Vec::new(),
            receive_event: receive_send_to_tui,
            send_event: send_to_game,
            player_list_state: ListState::default(),
            messages: Vec::new(),
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
                    SendToTUI::RequestDecision(decisions) => {
                        self.player_actions = decisions;
                    }
                    SendToTUI::ShowMessage(message) => {
                        self.messages.push(message);
                    }
                },
                Err(channel::RecvTimeoutError::Timeout) => {
                    match crossterm::event::poll(Duration::from_secs(0)) {
                        Ok(false) => {}
                        Ok(true) => {
                            // this returns true if we should quit
                            // should probably change to enum with more options
                            if self.handle_key()? {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                } // Timeout occurred redraw UI
                Err(channel::RecvTimeoutError::Disconnected) => break, // Channel closed or timeout
            }
        }
        Ok(())
    }

    /// Handle key events
    /// q: quit
    /// up/down: navigate choices
    /// enter: select choice
    fn handle_key(&mut self) -> Result<bool, PlayerTUIError> {
        match crossterm::event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => Ok(true),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                self.player_list_state.select_next();
                Ok(false)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                self.player_list_state.select_previous();
                Ok(false)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if let Some(selected) = self.player_list_state.selected() {
                    let choice = self.player_actions.swap_remove(selected);
                    self.send_event
                        .send(ReceiveFromTUI::DecisionMade(choice))
                        .unwrap();
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn render(&mut self, f: &mut Frame) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .split(f.area());

        let choice_list = layout[1];
        let list = List::new(self.player_actions.iter().enumerate().map(|(i, action)| {
            if self.player_list_state.selected() == Some(i) {
                format!("{}\n{}", action.name(), action.text())
            } else {
                format!("{}", action.name())
            }
        })).block(
            Block::bordered()
                .title("player actions")
                .title_bottom("Use ↑↓ arrows to choose, enter to select"),
        )
        .highlight_symbol("→")
        .highlight_style(Style::new().bold());
        f.render_stateful_widget(list, choice_list, &mut self.player_list_state);

        let vert = Layout::new(ratatui::layout::Direction::Vertical,
            [Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(20),Constraint::Percentage(20)])
            .split(layout[0]);

        let player_board_block = Block::bordered().title("Player Board");
        f.render_widget(player_board_block, vert[0]);

        let opponent_board_block = Block::bordered().title("Opponent Board");
        f.render_widget(opponent_board_block, vert[1]);
        let hand = Paragraph::new("lorem ipsum dolor sit amet")
            .block(Block::bordered().title("Cards"));
        f.render_widget(hand, vert[2]);

        let info = Paragraph::new(self.messages.join("\n"))
            .block(Block::bordered().title("Info"));
        f.render_widget(info, vert[3]);
    }
}
