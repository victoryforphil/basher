use super::System;

pub struct MockSystem {
    pub value: u16,
}

impl MockSystem {
    pub fn new() -> MockSystem {
        MockSystem { value: 0 }
    }
}

impl System for MockSystem {
    fn init(&mut self, state: &mut crate::state::BasherState) {
        self.value = 1;
    }

    fn execute(&mut self, state: &mut crate::state::BasherState) {
        self.value = 2;
    }

    fn cleanup(&mut self, state: &mut crate::state::BasherState) {
        self.value = 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BasherState;
    use crate::system::mock::MockSystem;

    #[test]
    fn test_mock_system() {
        let mut state = BasherState::new();
        let mut system = MockSystem::new();
        system.init(&mut state);
        assert_eq!(system.value, 1);
        system.execute(&mut state);
        assert_eq!(system.value, 2);
        system.cleanup(&mut state);
        assert_eq!(system.value, 3);
    }
}
