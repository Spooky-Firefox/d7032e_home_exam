use serde::{de, Deserialize, Serialize};

/// A basic card component containing a name and description
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct JsonCardComponent {
    pub name: String,
    #[serde(rename = "cardText")]
    pub description: String,

    #[serde(rename = "type")]
    pub type_of_card: CardType,

    #[serde(rename = "number")]
    pub number_of_cards: u32,

    pub theme: Theme,

    pub placement: Placement,

    pub produces: Option<Resource>,

    #[serde(rename = "ResourceStorage")]
    pub storage: Option<ResourceStorage>,

    // This is fore use in conjunction with number_of_cards to assign activation dice to each card created
    // this is not a good way to do it but it works for now
    // one better way would be to not have number_of_cards and instead have a list of cards to create
    // but due to time constraints we go with this approach (rewriting json is time consuming)
    pub for_each_card_activation_dice: Option<Vec<ActivationDice>>,

    pub card_activation_dice: Option<ActivationDice>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct ActivationDice(pub u8);
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Resource {
    Lumber,
    Gold,
    Grain,
    Brick,
    Wool,
    Ore,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]

pub enum ResourceStorage {
    Amount0,
    Amount1,
    Amount2,
    Amount3,
}

impl ResourceStorage {
    pub fn increase(&mut self) {
        *self = match self {
            ResourceStorage::Amount0 => ResourceStorage::Amount1,
            ResourceStorage::Amount1 => ResourceStorage::Amount2,
            ResourceStorage::Amount2 => ResourceStorage::Amount3,
            ResourceStorage::Amount3 => ResourceStorage::Amount3, // maxed out
        }
    }

    pub fn decrease(&mut self) {
        *self = match self {
            ResourceStorage::Amount0 => ResourceStorage::Amount0, // min value
            ResourceStorage::Amount1 => ResourceStorage::Amount0,
            ResourceStorage::Amount2 => ResourceStorage::Amount1,
            ResourceStorage::Amount3 => ResourceStorage::Amount2,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            ResourceStorage::Amount0 => 0,
            ResourceStorage::Amount1 => 1,
            ResourceStorage::Amount2 => 2,
            ResourceStorage::Amount3 => 3,
        }
    }
}

pub struct Card {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Owner {
    Player1,
    Player2,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    Board(i32, i32),
    EventStack(u32),
    DrawStack1(u32),
    DrawStack2(u32),
    DrawStack3(u32),
    DrawStack4(u32),
}

// Deleted duplicate Owner enum
