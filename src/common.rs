use std::sync::{Arc, Mutex};

pub trait DecisionChoice: Send {
    fn id(&self) -> u32;
    fn name(&self) -> String;
    fn text(&self) -> String;
    fn clone_box(&self) -> Box<dyn DecisionChoice>;
}

pub trait UserStrategy {
    fn new(state: Arc<Mutex<hecs::World>>) -> Self
    where
        Self: Sized;

    fn get_user_decision(
        &self,
        decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice>;
    
    fn send_message(&self, message: String);
}

pub trait Phase{
    fn evaluate(&mut self, state: Arc<Mutex<hecs::World>>, user_strategy: &mut dyn UserStrategy);
}
