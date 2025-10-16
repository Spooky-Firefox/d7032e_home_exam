use std::{
    f32::consts::E, sync::mpsc::{self, RecvTimeoutError, Sender}, thread::{self, JoinHandle, Thread}, time::Duration
};

use crate::common::{DecisionChoice, UserStrategy};

pub struct PlayerTUI {
    // Add fields as necessary
    thread: JoinHandle<Result<(), PlayerTUIError>>,
    send_event: mpsc::Sender<SendToTUI>,
    receive_event: mpsc::Receiver<ReceiveFromTUI>,
}
struct PlayerTUIThread {
    receive_event: mpsc::Receiver<SendToTUI>,
    send_event: mpsc::Sender<ReceiveFromTUI>,
}

enum SendToTUI {
    Quit,
    RequestDecision(Vec<Box<dyn DecisionChoice>>),
    ShowMessage(String),
}

enum ReceiveFromTUI {
    DecisionMade(Box<dyn DecisionChoice>),
    Quit,
}

enum PlayerTUIError {
    Todo,
}

impl PlayerTUIThread {
    fn new() -> (
        mpsc::Sender<SendToTUI>,
        mpsc::Receiver<ReceiveFromTUI>,
        JoinHandle<Result<(), PlayerTUIError>>,
    ) {
        let (send_to_tui, receive_send_to_tui) = mpsc::channel::<SendToTUI>();
        let (send_to_game, receive_event) = mpsc::channel();
        let player_tui_thread = PlayerTUIThread {
            receive_event: receive_send_to_tui,
            send_event: send_to_game,
        };
        let thread = thread::spawn(move || player_tui_thread.run());
        (send_to_tui, receive_event, thread)
    }

    fn run(&self) -> Result<(), PlayerTUIError> {
        loop {
            match self.receive_event.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => match event {
                    SendToTUI::Quit => break,
                    SendToTUI::RequestDecision(_decisions) => {
                        // Handle decision request
                    }
                    SendToTUI::ShowMessage(_message) => {
                        // Handle showing message
                    }
                },
                Err(RecvTimeoutError::Timeout) => continue, // Timeout occurred redraw UI
                Err(RecvTimeoutError::Disconnected) => break, // Channel closed or timeout
            }
        }
        Ok(())
    }
}

impl UserStrategy for PlayerTUI {
    fn get_user_decision(
        &self,
        _decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice> {
        todo!()
    }
}

impl PlayerTUI {
    pub fn new(_state_ref: ()) -> Self {
        let (send_event, receive_event, player_tui) = PlayerTUIThread::new();
        Self {
            thread: player_tui,
            send_event,
            receive_event,
        }
    }
}

impl Drop for PlayerTUI {
    fn drop(&mut self) {
        // Clean up terminal UI resources here
    }
}
