use crate::{common::{Phase, UserStrategy}, game_objects::event_dice};
use std::sync::{Arc, Mutex};

pub struct EventBrigandAttack;

impl EventBrigandAttack {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for EventBrigandAttack {
    fn evaluate(&mut self, state: Arc<Mutex<hecs::World>>, user_strategy: &mut dyn UserStrategy) {
        // lock state
        let mut world = state.lock().unwrap();
        let event_dice_query = world.query_mut::<(&event_dice::EventDice,)>();
        let (_, (event_dice,)) = event_dice_query
            .into_iter()
            .next()
            .expect("Could not find the event die");

        if event_dice.current_event == event_dice::EventType::Brigand {
            user_strategy.send_message("Brigand attack event triggered!".to_string());
            // Implement brigand attack logic here
        } else {
            user_strategy.send_message("No brigand attack this turn.".to_string());
            // Implement logic for no brigand attack
        }
    }
}
