use std::sync::{Arc, Mutex};
use hecs::{EntityBuilder, World};
use serde_json;
use serde::{Serialize, Deserialize};
/// A basic card component containing a name and description
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct JsonCardComponent {
    pub name: String,
    #[serde(rename = "cardText")]
    pub description: String,

    #[serde(rename = "type")]
    pub type_of_card: CardType,

    pub theme: Theme,

    pub placement: Placement,
}

pub struct Card {
    pub name: String,
    pub description: String,
}

/// Card type enum representing different card categories
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CardType {
    #[serde(rename = "Action – Attack")]
    ActionAttack,
    #[serde(rename = "Action – Neutral")]
    ActionNeutral,
    #[serde(rename = "Building")]
    Building,
    #[serde(rename = "Center Card")]
    CenterCard,
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "Extraordinary Site")]
    ExtraordinarySite,
    #[serde(rename = "Marker Card")]
    MarkerCard,
    #[serde(rename = "Metropolis")]
    Metropolis,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Unit")]
    Unit,
    #[serde(rename = "Unit – Hero")]
    UnitHero,
    #[serde(rename = "Unit – Sage")]
    UnitSage,
    #[serde(rename = "Unit – Ship")]
    UnitShip,
    #[serde(rename = "Unit – Trade Ship")]
    UnitTradeShip,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Placement {
    #[serde(rename = "Action")]
    Action,
    #[serde(rename = "Action (Owls)")]
    ActionOwls,
    #[serde(rename = "Center Card")]
    CenterCard,
    #[serde(rename = "City")]
    City,
    #[serde(rename = "City (Foreign)")]
    CityForeign,
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "Marker Card")]
    MarkerCard,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Region (Foreign)")]
    RegionForeign,
    #[serde(rename = "Road")]
    Road,
    #[serde(rename = "Road (Foreign)")]
    RoadForeign,
    #[serde(rename = "Sea")]
    Sea,
    #[serde(rename = "Settlement/city")]
    SettlementCity,
}

#[derive(serde::Serialize, serde::Deserialize, Clone,PartialEq, Eq)]
pub enum Theme {
    #[serde(rename = "Barbarians")]
    Barbarians,
    #[serde(rename = "Basic")]
    Basic,
    #[serde(rename = "Basic + Gold")]
    BasicGold,
    #[serde(rename = "Basic + Progress")]
    BasicProgress,
    #[serde(rename = "Basic + Progress + Barbarians")]
    BasicProgressBarbarians,
    #[serde(rename = "Basic ÷ Turmoil")]
    BasicTurmoil,
    #[serde(rename = "Explorers")]
    Explorers,
    #[serde(rename = "Gold")]
    Gold,
    #[serde(rename = "Gold + Merchant Princes")]
    GoldMerchantPrinces,
    #[serde(rename = "Gold + Turmoil")]
    GoldTurmoil,
    #[serde(rename = "Intrigue")]
    Intrigue,
    #[serde(rename = "Merchant Princes")]
    MerchantPrinces,
    #[serde(rename = "Progress")]
    Progress,
    #[serde(rename = "Prosperity")]
    Prosperity,
    #[serde(rename = "Sages")]
    Sages,
    #[serde(rename = "Turmoil")]
    Turmoil,

}

/// Position component for cards
pub enum Position {
    Board(i32, i32),
    EventStack(u32),
    DrawStack1(u32),
    DrawStack2(u32),
    DrawStack3(u32),
    DrawStack4(u32),
}

pub enum Ownership {
    Player1,
    Player2,
}


/// Initialize the card deck for the game
pub fn initialize_cards(state: Arc<Mutex<World>>) {
    let mut world = state.lock().unwrap();

    let cards: Vec<JsonCardComponent> = serde_json::from_str(
        include_str!("../../VajbCruncher/cards.json")
    ).expect("Failed to parse card data");


    cards.into_iter().for_each(|card| {
        
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
        } = card;

        // if the theme is not basic we skip the card
        if theme != Theme::Basic {
            return;
        }

        // Create a new entity builder which we will use to build the entity
        let mut e  = EntityBuilder::new();

        e.add(Card {
            name,
            description,
        });

        e.add(type_of_card);
        e.add(placement);

        world.spawn(e.build());
    });

    // move cards into the board
    // select two forrest and place them
    // ...


    // move cards into stacks
    
    // 
    let mut stack_1_index = 0;


}
