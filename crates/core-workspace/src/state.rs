use crate::{BindingState, M02Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingStateMachine {
    state: BindingState,
}

impl Default for BindingStateMachine {
    fn default() -> Self {
        Self {
            state: BindingState::Unbound,
        }
    }
}

impl BindingStateMachine {
    pub fn state(&self) -> BindingState {
        self.state
    }

    pub fn transition(&mut self, next: BindingState) -> Result<(), M02Error> {
        let allowed = matches!(
            (self.state, next),
            (BindingState::Unbound, BindingState::Discovering)
                | (BindingState::Discovering, BindingState::Validating)
                | (BindingState::Discovering, BindingState::Blocked)
                | (BindingState::Validating, BindingState::Bound)
                | (BindingState::Validating, BindingState::Blocked)
                | (BindingState::Bound, BindingState::Drifted)
                | (BindingState::Bound, BindingState::Detaching)
                | (BindingState::Drifted, BindingState::Revalidating)
                | (BindingState::Drifted, BindingState::Detaching)
                | (BindingState::Revalidating, BindingState::Bound)
                | (BindingState::Revalidating, BindingState::Blocked)
                | (BindingState::Blocked, BindingState::Discovering)
                | (BindingState::Blocked, BindingState::Validating)
                | (BindingState::Blocked, BindingState::Detaching)
                | (BindingState::Detaching, BindingState::Unbound)
        );
        if !allowed {
            return Err(M02Error::InvalidTransition {
                from: self.state,
                to: next,
            });
        }
        self.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_attach_lifecycle_is_explicit() {
        let mut machine = BindingStateMachine::default();
        for state in [
            BindingState::Discovering,
            BindingState::Validating,
            BindingState::Bound,
        ] {
            machine.transition(state).unwrap();
        }
        assert_eq!(machine.state(), BindingState::Bound);
    }

    #[test]
    fn illegal_transition_is_typed() {
        let mut machine = BindingStateMachine::default();
        assert!(matches!(
            machine.transition(BindingState::Bound),
            Err(M02Error::InvalidTransition { .. })
        ));
    }
}
