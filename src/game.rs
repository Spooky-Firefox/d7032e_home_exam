use std::sync::{Arc, Mutex};

use crate::common::UserStrategy;
use hecs::World;

pub struct Game<T: UserStrategy> {
    user_strategy: T,
    state: Arc<Mutex<World>>,
}


impl<T: UserStrategy> Game<T> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(World::new()));
        Game {
            user_strategy: T::new(state.clone()),
            state
        }
    }

    pub fn start(&mut self) {
        let u = self.user_strategy.get_user_decision(
            (0..10)
                .map(|i| {
                    Box::new(crate::util::SimpleDecisionChoice {
                        name: format!("Choice {}", i),
                        description: format!("This is choice number {}", i),
                    }) as Box<dyn crate::common::DecisionChoice>
                })
                .collect(),

        );
        // Game starting logic can be implemented here
    }
}
