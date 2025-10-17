mod game;
mod terminal_ui;
mod common;
mod util;

use std::{thread::sleep, time::Duration};

use terminal_ui::PlayerTUI;

use crate::game::Game;
fn main() {
    let mut game: Game<PlayerTUI> = game::Game::new();
    game.start();
}
