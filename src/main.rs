mod game;
mod terminal_ui;
mod common;
mod util;
mod phase;
mod cards;

use terminal_ui::PlayerTUI;

use crate::game::Game;
fn main() {
    let mut game: Game<PlayerTUI> = game::Game::new();
    game.start();
}
