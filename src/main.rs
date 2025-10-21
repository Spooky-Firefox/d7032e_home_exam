mod cards;
mod common;
mod game;
mod phase;
mod terminal_ui;
mod util;
mod game_objects;

use terminal_ui::PlayerTUI;

use crate::game::Game;
fn main() {
    // setup a logger for debugging
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(".log").expect("Failed to open log file"))
        .apply()
        .expect("Failed to initialize logger");

    log::info!("Starting game...");
    let mut game: Game<PlayerTUI> = game::Game::new();
    game.start();
}
