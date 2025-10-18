use crate::common::{Phase, UserStrategy};
use std::sync::{Arc, Mutex};

pub struct ActionPhase;

impl ActionPhase {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ActionPhase {
    fn evaluate(_state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        todo!()
    }
}
