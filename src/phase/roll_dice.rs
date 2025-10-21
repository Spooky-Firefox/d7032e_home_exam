use crate::{
    common::{Phase, UserStrategy},
    game_objects, util,
};
use std::sync::{Arc, Mutex};

pub struct DiceRoll;
impl DiceRoll {
    pub fn new() -> Self {
        DiceRoll
    }
}

impl Phase for DiceRoll {
    fn evaluate(&mut self, state: Arc<Mutex<hecs::World>>, user_strategy: &mut dyn UserStrategy) {
        let mut world = state.lock().unwrap(); // get the lock on the world
        
        let q = world.query_mut::<(&mut game_objects::production_dice::ProductionDice,)>(); // query for the ProductionDie component (aka our only die)
        let (_, (dice,)) = q
            .into_iter()
            .next()
            .expect("Could not find the production die"); // get the first (and only) result

        let p_roll = dice.0.roll(); // roll the die

        let q = world.query_mut::<(&mut game_objects::event_dice::EventDice,)>(); // query for the EventDie  component (aka our only die)
        let (_, (dice,)) = q
            .into_iter()
            .next()
            .expect("Could not find the event die"); // get the first (and only) result

        let e_roll = dice.roll(); // roll the die

        user_strategy.send_message(format!(
            "Rolled production die: {}, event die: {:?}",
            p_roll, e_roll
        ));

    }
}
