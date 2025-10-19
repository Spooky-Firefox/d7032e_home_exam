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



//TODO move to a file with common game entities 
pub struct ProductionDie;

impl ProductionDie {
    pub fn new() -> Self {
        Self
    }

    pub fn roll(&self) -> u8 {
        // Simulate a die roll (1-6)
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random_range(1..=6)
    }
}
