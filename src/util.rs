use hecs::{Query, QueryMut, World};

use crate::common::{self, DecisionChoice};

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


pub struct DebugUI;

impl common::UserStrategy for DebugUI {
    fn new(state: std::sync::Arc<std::sync::Mutex<hecs::World>>) -> Self
    where
        Self: Sized {
        Self
    }

    fn get_user_decision(
        &self,
        decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice> {
        for decision in &decisions {
            println!("Decision available: {} - {}", decision.id(), decision.name());
        }

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // For simplicity, just return the first decision

        decisions[usize::from_str_radix(input.trim(), 10).unwrap()].clone_box()
    }

    fn send_message(&self, message: String) {
        println!("Message to user: {}", message);
    }
}

/// Utility function to query a single entity from the world
pub fn query_one_from_world<T: Query>(world: &mut World) -> Option<<T as Query>::Item<'_>> {
    let mut query = world.query_mut::<T>();
    query
        .into_iter()
        .next()
        .map(|entity| entity.1)

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
