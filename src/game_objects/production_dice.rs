use crate::game_objects::dice::Dice;




/// Represents the result values of a production dice roll
/// Production dice used to determine which resource regions produce resources
/// Implemented as a tuple struct containing a generic Dice
#[derive(Debug, Clone)]
pub struct ProductionDice(pub Dice);
