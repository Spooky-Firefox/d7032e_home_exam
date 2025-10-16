pub trait DecisionChoice {
    fn name(&self) -> String;
    fn text(&self) -> String;
}

pub trait UserStrategy {
    fn get_user_decision(
        &self,
        decisions: Vec<Box<dyn DecisionChoice>>,
    ) -> Box<dyn DecisionChoice>;
}
