use crate::common::DecisionChoice;
use crate::common::Phase;
use crate::common::UserStrategy;
use crate::util::SimpleDecisionChoice;

/// a phase that loops over a set of phases until an abort message is chosen
/// compile time (const generics) enforced that the number of phases is known at compile time
/// and the same amount of abort messages are provided
struct LoopPhase<const PHASES: usize> {
    phases: [Box<dyn Phase>; PHASES],
    abort_messages: [Option<[SimpleDecisionChoice; 2]>; PHASES],
}

impl<const PHASES: usize> LoopPhase<PHASES> {
    pub fn new(
        phases: [Box<dyn Phase>; PHASES],
        abort_messages: [Option<[SimpleDecisionChoice; 2]>; PHASES],
    ) -> Self {
        // Ensure that no two abort messages in the array have the same id
        assert!(
            !abort_messages
                .iter()
                .any(|d| d.as_ref().is_some_and(|a| a[0].id() == a[1].id()))
        );
        Self {
            phases,
            abort_messages,
        }
    }
}

impl<const PHASES: usize> Phase for LoopPhase<PHASES> {
    fn evaluate(
        &mut self,
        state: std::sync::Arc<std::sync::Mutex<hecs::World>>,
        user_strategy: &mut dyn UserStrategy,
    ) {
        loop {
            for (phase, abort_message) in
                &mut self.phases.iter_mut().zip(self.abort_messages.clone())
            {
                phase.evaluate(state.clone(), user_strategy);
                if let Some(abort_message) = abort_message {

                    // its not nice having to convert to vec boxes here
                    // but a vec won't allow compile time checks
                    // could be solved with an assert but i prefer compile time over runtime checks
                    // another option would be to change the decision api to only require the index trait
                    let decision = user_strategy.get_user_decision(vec![
                        Box::new(abort_message[0].clone()),
                        Box::new(abort_message[1].clone()),
                    ]);
                    if decision.id() == abort_message[1].id {
                        return;
                    }
                }
            }
        }
    }
}
