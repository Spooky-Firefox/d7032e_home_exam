use crate::game_objects::dice::Dice;

/// Represents the possible outcomes of an event dice roll
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Brigand,        // Roll value: 1
    Trade,          // Roll value: 2
    Celebration,    // Roll value: 3
    PlentifulHarvest, // Roll value: 4
    EventCard,      // Roll values: 5 and 6
}

/// Event dice used to determine which event occurs during a turn
#[derive(Debug, Clone)]
pub struct EventDice {
    dice: Dice,
    pub current_event: EventType,
}

impl EventDice {
    /// Create a new event dice
    pub fn new() -> Self {
        Self {
            dice: Dice::new(6),
            current_event: EventType::EventCard, // Default value
        }
    }

    pub fn roll(&mut self) -> EventType {
        let roll_value = self.dice.roll();
        self.current_event = match roll_value {
            1 => EventType::Brigand,
            2 => EventType::Trade,
            3 => EventType::Celebration,
            4 => EventType::PlentifulHarvest,
            5 | 6 => EventType::EventCard,
            _ => unreachable!(),
        };
        self.current_event
    }
}

impl Default for EventDice {
    fn default() -> Self {
        Self::new()
    }
}
