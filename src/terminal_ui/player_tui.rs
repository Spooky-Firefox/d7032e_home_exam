use crate::common::user_decision::{DecisionChoice, UserStrategy};

pub struct PlayerTUI {
    // Add fields as necessary
    state_ref: !, // Replace '!' with the actual type
}

impl UserStrategy for PlayerTUI {
    fn get_user_decision(
        &self,
        decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice> {
        todo!()
    }
}

impl PlayerTUI {
    pub fn new(state_ref: !) -> Self {
        Self { state_ref }
    }
}


impl drop::Drop for PlayerTUI {
    fn drop(&mut self) {
        // Clean up terminal UI resources here
    }
}
