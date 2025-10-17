use std::sync::{Arc, Mutex};

pub trait DecisionChoice: Send {
    fn name(&self) -> String;
    fn text(&self) -> String;
}

pub trait UserStrategy {
    fn new(state: Arc<Mutex<hecs::World>>) -> Self;

    fn get_user_decision(
        &self,
        decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice>;
}
