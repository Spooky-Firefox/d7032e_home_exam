use log::{info, trace};

use crate::{cards::cards::{Card, Owner, Position}, common::{Phase, UserStrategy}, game_objects::{active_player_token::ActivePlayerToken, production_dice}, util::query_one_from_world};
use std::sync::{Arc, Mutex};

pub struct ProductionPhase;

impl ProductionPhase {
    pub fn new() -> Self {
        Self
    }
}

impl Phase for ProductionPhase {
    fn evaluate(&mut self, state: Arc<Mutex<hecs::World>>, _user_strategy: &mut dyn UserStrategy) {
        // lock state
        let mut world = state.lock().unwrap();

        let production_dice = query_one_from_world::<(&production_dice::ProductionDice,)>(&mut world).unwrap().0;


        let roll_result = production_dice.0.current_value;

        let (_, owner) = query_one_from_world::<(&ActivePlayerToken,&Owner)>(&mut world).unwrap();
        let owner = owner.clone();
        // query all cards with have production matching the roll and owner == active player token
        let matching_cards = world.query_mut::<(&Position,&Card, &Owner)>().into_iter().filter(|(_, (pos, _, card_owner))| {
            **card_owner == owner && matches!(pos, Position::Board(_, _))
        });

        info!("cards matching for owner {:?}: {:?}", owner, matching_cards.map(|(_,(_, card,_))| card.name.clone()).collect::<Vec<_>>());
        // - Distribute resources to players based on their regions matching the roll
        // - Consider special cases like gold fields
    }
}
