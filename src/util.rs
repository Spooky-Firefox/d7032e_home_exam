use crate::common::DecisionChoice;

pub struct SimpleDecisionChoice {
    pub name: String,
    pub description: String,
}

impl DecisionChoice for SimpleDecisionChoice {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn text(&self) -> String {
        self.description.clone()
    }
}
