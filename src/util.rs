use crate::common::DecisionChoice;

#[derive(Clone)]
pub struct SimpleDecisionChoice {
    pub id: u32,
    pub name: String,
    pub description: String,
}

impl SimpleDecisionChoice {
    pub fn new(id: u32, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

impl DecisionChoice for SimpleDecisionChoice {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn text(&self) -> String {
        self.description.clone()
    }

    fn id(&self) -> u32 {
        self.id
    }
    
    fn clone_box(&self) -> Box<dyn DecisionChoice> {
        Box::new(self.clone())
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
