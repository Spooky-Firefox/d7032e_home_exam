use crate::common::{Phase, UserStrategy};
use std::sync::{Arc, Mutex};

pub struct ExchangeHand;

impl ExchangeHand {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ExchangeHand {
    fn evaluate(&mut self, _state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        todo!()
    }
}
