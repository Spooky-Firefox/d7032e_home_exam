mod terminal_ui;
mod common;
use terminal_ui::PlayerTUI;
fn main() {
    let player_tui = PlayerTUI::new(()/* pass the actual state_ref here */);
    println!("Hello, world!");
}
