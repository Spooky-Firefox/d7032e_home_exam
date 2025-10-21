use std::sync::{Arc, Mutex};

use crate::{
    cards::initialize::init_card_position,
    common::UserStrategy,
    phase::{
        brigand_attack, loop_phase, production, roll_dice::{self, DiceRoll}
    },
    util::{self, SimpleDecisionChoice},
};
use hecs::World;
use log::info;
//todo dont use initialize cards, make a better solution later
use crate::cards::initialize::initialize_cards;
use crate::game_objects;
pub struct Game<T: UserStrategy> {
    user_strategy: T,
    phase: Box<dyn crate::common::Phase>,
    state: Arc<Mutex<World>>,
}

impl<T: UserStrategy> Game<T> {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(World::new()));

        // TODO make another function to initialize other game components
        info!("Initializing game components...");

        // TODO move make this less clunky have array of initializers or something

        info!("Initializing cards...");
        initialize_cards(state.clone());

        info!("Initializing card positions...");
        init_card_position(state.clone());

        info!("initializing game objects...");
        game_objects::init::initialize_game_objects(state.clone());
        Game {
            user_strategy: T::new(state.clone()),
            state,
            phase: Box::new(loop_phase::LoopPhase::new(
                [
                    Box::new(roll_dice::DiceRoll::new()),
                    Box::new(brigand_attack::EventBrigandAttack::new()),
                    Box::new(production::ProductionPhase::new()),
                ],
                [
                    None,
                    None,
                    Some([
                        SimpleDecisionChoice::new(0, "Continue", "Continue to next Turn"),
                        SimpleDecisionChoice::new(1, "Abort", "Abort the game"),
                    ]),
                ],
            )),
        }
    }

    pub fn start(&mut self) {
        self.user_strategy.send_message("game started".to_string());
        info!("evaluating base phase...");
        self.phase
            .evaluate(self.state.clone(), &mut self.user_strategy);
    }
}
