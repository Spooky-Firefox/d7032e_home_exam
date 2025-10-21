/// Represents a generic dice that can have any number of sides
#[derive(Debug, Clone)]
pub struct Dice {
    pub sides: u8,
    pub current_value: u8,
}

impl Dice {
    /// Create a new dice with the specified number of sides
    pub fn new(sides: u8) -> Self {
        Self {
            sides,
            current_value: 1,
        }
    }

    /// Create a standard 6-sided dice
    pub fn d6() -> Self {
        Self::new(6)
    }

    /// Roll the dice to get a random number between 1 and the number of sides
    pub fn roll(&mut self) -> u8 {
        use rand::Rng;
        
        let mut rng = rand::rng();
        self.current_value = rng.random_range(1..=self.sides);
        self.current_value
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self::d6()
    }
}
