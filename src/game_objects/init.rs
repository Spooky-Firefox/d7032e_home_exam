use hecs::{EntityBuilder, World};
use std::sync::{Arc, Mutex};

use crate::game_objects::{
    active_player_token::ActivePlayerToken,
    dice::Dice,
    event_dice::EventDice,
    production_dice::ProductionDice,
    strength_token::StrengthToken,
    trade_token::TradeToken,
};

/// Initialize all the game objects in the world
pub fn initialize_game_objects(state: Arc<Mutex<World>>) {
    let mut world = state.lock().unwrap();
    
    // Create active player token (one per game)
    world.spawn((ActivePlayerToken,));
    
    // Create dice for production (one per game)
    let dice = Dice::new(6);
    let production_dice = ProductionDice(dice);
    world.spawn((production_dice,));

    // Create event dice (one per game)
    let event_dice = EventDice::new();
    world.spawn((event_dice,));
    
    // Create strength token
    world.spawn((StrengthToken,));

    // create trade token    
    world.spawn((TradeToken,));
}
