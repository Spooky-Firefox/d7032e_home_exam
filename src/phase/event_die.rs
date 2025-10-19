use crate::common::{Phase, UserStrategy};
use std::sync::{Arc, Mutex};

pub struct EventDie;

impl EventDie {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for EventDie {
    fn evaluate(&mut self, _state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        todo!()
    }
}
