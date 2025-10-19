use std::sync::{Arc, Mutex};

use crate::{common::UserStrategy, util::ProductionDie};
use hecs::World;
//todo dont use initialize cards, make a better solution later
use crate::cards::initialize::initialize_cards;
pub struct Game<T: UserStrategy> {
    user_strategy: T,
    phases: Vec<Box<dyn crate::common::Phase>>,
    state: Arc<Mutex<World>>,
}

impl<T: UserStrategy> Game<T> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(World::new()));
        state.lock().unwrap().spawn((ProductionDie::new(),));

        initialize_cards(state.clone());
        Game {
            user_strategy: T::new(state.clone()),
            state,
            phases: vec![Box::new(crate::phase::production_die::ProductionDie::new())],
        }
        
    }

    pub fn start(&mut self) {
        // temporary user decision example
        self.user_strategy.send_message("game started".to_string());
        
        self.phases.iter_mut().for_each(|phase| {
            phase.evaluate(
                self.state.clone(), /* this is a arc, "shallow copy" */
                &mut self.user_strategy,
            );
        });
        // let _u = self.user_strategy.get_user_decision(
        //     (0..10)
        //         .map(|i| {
        //             Box::new(crate::util::SimpleDecisionChoice {
        //                 name: format!("Choice {}", i),
        //                 description: format!("This is choice number {}", i),
        //             }) as Box<dyn crate::common::DecisionChoice>
        //         })
        //         .collect(),

        // );
        // Game starting logic can be implemented here
    }
}
