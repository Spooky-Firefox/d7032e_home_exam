use crate::common::{Phase, UserStrategy};
use std::sync::{Arc, Mutex};

pub struct ReplenishHand;

impl ReplenishHand {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ReplenishHand {
    fn evaluate(_state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        todo!()
    }
}
