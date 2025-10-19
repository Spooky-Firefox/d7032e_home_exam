use crate::{
    common::{Phase, UserStrategy},
    util,
};
use std::sync::{Arc, Mutex};

pub struct ProductionDie;

impl ProductionDie {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ProductionDie {
    fn evaluate(&mut self, state: Arc<Mutex<hecs::World>>, user_strategy: &mut dyn UserStrategy) {
        let mut world = state.lock().unwrap(); // get the lock on the world
        let q = world.query_mut::<(&util::ProductionDie,)>(); // query for the ProductionDie component (aka our only die)

        let (_, (die,)) = q
            .into_iter()
            .next()
            .unwrap(); // get the first (and only) result

        user_strategy.get_user_decision(vec![
            Box::new(util::SimpleDecisionChoice {
                name: "production Die".to_string(),
                description: format!("{}", die.roll()).to_string(),
            })
        ]);
    }
}
