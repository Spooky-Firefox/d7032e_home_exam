use crate::common::{Phase, UserStrategy};
use std::sync::{Arc, Mutex};

pub struct ProductionDie;

impl ProductionDie {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ProductionDie {
    fn evaluate(_state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        todo!()
    }
}
