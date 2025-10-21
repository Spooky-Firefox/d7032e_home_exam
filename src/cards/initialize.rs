use crate::cards::cards::{ActivationDice, Card, JsonCardComponent, Owner, Position, ResourceStorage, Theme};
use hecs::{EntityBuilder, World};
use serde_json;
use std::sync::{Arc, Mutex};

/// Initialize the card deck for the game
pub fn initialize_cards(state: Arc<Mutex<World>>) {
    let mut world = state.lock().unwrap();

    let cards: Vec<JsonCardComponent> =
        serde_json::from_str(include_str!("cards.json")).expect("Failed to parse card data");

    // assert that each cards has a unique name
    // since we dont have a card type id or similar we use the name as the unique identifier
    let mut card_names = std::collections::HashSet::new();
    for card in &cards {
        if !card_names.insert(&card.name) {
            panic!("Duplicate card name found: {}", card.name);
        }
    }

    let mut e = EntityBuilder::new();
    cards.into_iter().for_each(|card| {
        // create the number of cards specified an example would be roads, which we have 9 of
        for i in 0..card.number_of_cards {
            // destructure the card to get its fields
            // this allows us to insert each field as its own component
            // as well as handle "null components" by skipping them
            // so if a component have null on vp we simply dont insert a vp component

            // there exist another way to do this with serde and that is to serialize it to a map
            // and then iterate over the map inserting components based on the keys
            // but this way is more type safe and easier to read
            // but may lead to more Option<> unwraps/if let in the future if we add more optional components
            let JsonCardComponent {
                name,
                description,
                type_of_card,
                placement,
                theme,
                produces,
                for_each_card_activation_dice,
                storage,
                ..
            } = card.clone(); // the clone is necessary to as we might construct multiple cards from the same template

            // if the theme is not basic we skip the card
            if theme != Theme::Basic {
                return;
            }

            // Create a new entity builder which we will use to build the entity
            if let Some(r) = produces {
                e.add(r);
            }

            if let Some(storage) = storage {
                e.add(storage);
            }

            // this is not the nicest way of doing it but due to the number_of_cards we have to do it this way
            // see the comment in cards.rs:21 for more details and possible future improvements
            if let Some(activation) =
                for_each_card_activation_dice.map(|a| a.get(i as usize).unwrap().clone())
            {
                e.add(activation);
            }

            e.add(Card { name, description });

            e.add(type_of_card);
            e.add(placement);

            world.spawn(e.build());
        }
    });
}

pub fn init_card_position(state: Arc<Mutex<World>>) {
    
    
    fn place_cards(world: &mut World, card_name: &str, position: Position) {
        // move cards into  the board
        // select two forrest and place them
        let mut q = world.query::<&Card>();
        let region_cards = q
            .iter()
            .filter_map(|(e, card)| {
                if card.name == card_name {
                    Some(e)
                } else {
                    None
                }
            })
            .take(2)
            .collect::<Vec<_>>();
        drop(q); // need to drop the query to avoid borrow issues (since both insert and query borrow world mutably)

        // this should not trigger since we know there are at least two forest cards
        assert!(
            region_cards.len() == 2,
            "Not enough cards found to initialize the board {}",
            card_name
        );

        // the center road is at 0,0
        // +x is right, +y is down

        // unwrap is safe here as we know there exist at least two forest cards (if loading the cards have not failed)
        world.insert_one(region_cards[0], position.clone()).unwrap();
        world.insert_one(region_cards[0], Owner::Player1).unwrap();

        world.insert_one(region_cards[1], position).unwrap();
        world.insert_one(region_cards[1], Owner::Player2).unwrap();
    }

    fn place_card_with_activation(world: &mut World, card_name: &str, position: Position, owner: Owner, activation: ActivationDice) -> hecs::Entity {
        // move cards into  the board
        let mut q = world.query::<(&Card,&ActivationDice)>();
        let card = q
            .iter()
            .filter_map(|(e, (card, card_activation))| {
                if card.name == card_name && *card_activation == activation {
                    Some(e)
                } else {
                    None
                }
            }).next().unwrap();
        drop(q); // need to drop the query to avoid borrow issues (since both insert and query borrow world mutably)
        world.insert_one(card, position.clone()).unwrap();
        world.insert_one(card, owner).unwrap();
        card
    }

    let mut world = state.lock().unwrap();

    // place cards for player 1
    // ugly, the better idea would be to have individual cards instead of relying on number of cards in the card template
    place_card_with_activation(&mut world, "Gold Field", Position::Board(0, -1), Owner::Player1, ActivationDice(1)); // Gold Field
    let cards_with_1_resource =  [
        place_card_with_activation(&mut world, "Forest", Position::Board(-2, -1), Owner::Player1, ActivationDice(2)), // Forest
        place_card_with_activation(&mut world, "Field", Position::Board(2, -1), Owner::Player1, ActivationDice(6)), // Field
        place_card_with_activation(&mut world, "Hill", Position::Board(-2, 1), Owner::Player1, ActivationDice(3)), // Hills
        place_card_with_activation(&mut world, "Pasture", Position::Board(0, 1), Owner::Player1, ActivationDice(4)), // Pasture
        place_card_with_activation(&mut world, "Mountain", Position::Board(2, 1), Owner::Player1, ActivationDice(5)), // Mountains
        
        // place cards for player 2 
        place_card_with_activation(&mut world, "Forest", Position::Board(-2, -1), Owner::Player2, ActivationDice(3)), // Forest
        place_card_with_activation(&mut world, "Field", Position::Board(2, -1), Owner::Player2, ActivationDice(5)), // Field
        place_card_with_activation(&mut world, "Hill", Position::Board(-2, 1), Owner::Player2, ActivationDice(2)), // Hills
        place_card_with_activation(&mut world, "Pasture", Position::Board(0, 1), Owner::Player2, ActivationDice(1)), // Pasture
        place_card_with_activation(&mut world, "Mountain", Position::Board(2, 1), Owner::Player2, ActivationDice(6)), // Mountains
    ];
    place_card_with_activation(&mut world, "Gold Field", Position::Board(0, -1), Owner::Player2, ActivationDice(4)); // Gold Field

    // yet again some code with not so nice looks
    // this could be improved by not having Number of cards in the card template
    // and each card on the board could have position owner and other components already defined
    // but for now this will do :(
    world.query_many_mut::<&mut ResourceStorage, _>(cards_with_1_resource)
    .into_iter()
    .for_each(|s| {
        *s.unwrap() = ResourceStorage::Amount1;
    });
    
    // place road cards for both players
    place_cards(&mut world, "Road", Position::Board(0, 0));

    // place settlement cards for both players
    place_cards(&mut world, "Settlement", Position::Board(-1, 0));
    place_cards(&mut world, "Settlement", Position::Board(1, 0));
    

    // TODO place cards into stacks

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_cards() -> Result<(), Vec<String>> {
        let mut err: Vec<String> = vec![];
        let state = Arc::new(Mutex::new(World::new()));
        initialize_cards(state.clone());

        // query the world to check if cards have been added
        let world = state.lock().unwrap();
        let card_count = world.query::<&Card>().iter().count();
        assert!(card_count > 0, "No cards were added to the world");

        // test some random cards to see if they exist
        // its to much work to test all cards here

        let mut q = world.query::<&Card>();
        let has_forest = q.iter().filter(|(_, card)| card.name == "Forest").count();
        let has_gold_field = q
            .iter()
            .filter(|(_, card)| card.name == "Gold Field")
            .count();
        let has_road = q.iter().filter(|(_, card)| card.name == "Road").count();

        if has_forest != 4 {
            err.push("Forest card not found".to_string());
        }
        if has_gold_field != 4 {
            err.push("Gold Field card not found".to_string());
        }
        if has_road != 9 {
            err.push("Road card not found".to_string());
        }

        if err.len() > 0 { Err(err) } else { Ok(()) }
    }

    #[test]
    fn test_init_card_position() -> Result<(), Vec<String>> {
        let mut err: Vec<String> = vec![];

        let state = Arc::new(Mutex::new(World::new()));
        initialize_cards(state.clone());
        init_card_position(state.clone());

        let world = state.lock().unwrap();
        let mut q = world.query::<(&Card, &Position, &Owner)>();

        // Check that the Forest cards are in the correct positions for both players
        let mut forest_player1 = false;
        let mut forest_player2 = false;
        for (_, (card, position, ownership)) in q.iter() {
            if card.name == "Forest" && *position == Position::Board(-2, -1) {
                match ownership {
                    Owner::Player1 => forest_player1 = true,
                    Owner::Player2 => forest_player2 = true,
                }
            }
        }
        if !forest_player1 {
            err.push("Forest card not in correct position for Player 1".to_string());
        }
        if !forest_player2 {
            err.push("Forest card not in correct position for Player 2".to_string());
        }

        // Check that the Gold Field cards are in the correct positions for both players
        let mut gold_field_player1 = false;
        let mut gold_field_player2 = false;
        for (_, (card, position, ownership)) in q.iter() {
            if card.name == "Gold Field" && *position == Position::Board(0, -1) {
                match ownership {
                    Owner::Player1 => gold_field_player1 = true,
                    Owner::Player2 => gold_field_player2 = true,
                }
            }
        }
        if !gold_field_player1 {
            err.push("Gold Field card not in correct position for Player 1".to_string());
        }
        if !gold_field_player2 {
            err.push("Gold Field card not in correct position for Player 2".to_string());
        }

        // Check that the Field cards are in the correct positions for both players
        let mut field_player1 = false;
        let mut field_player2 = false;
        for (_, (card, position, ownership)) in q.iter() {
            if card.name == "Field" && *position == Position::Board(2, -1) {
                match ownership {
                    Owner::Player1 => field_player1 = true,
                    Owner::Player2 => field_player2 = true,
                }
            }
        }
        if !field_player1 {
            err.push("Field card not in correct position for Player 1".to_string());
        }
        if !field_player2 {
            err.push("Field card not in correct position for Player 2".to_string());
        }

        // other tests can be added similarly for Hills, Pasture, Mountains

        if err.len() > 0 { Err(err) } else { Ok(()) }
    }
}
