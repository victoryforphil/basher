use std::collections::BTreeSet;

use rerun::external::ndarray::Data;
use serde::{Deserialize, Serialize};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};


pub struct MockSystem {
    pub value: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MockSystemData{
    value: u16,
}

impl MockSystem {
    pub fn new() -> MockSystem {
        MockSystem { value: 0 }
    }
}
/*
 self.value += inputs.value;
        MockSystemData { value: self.value }
*/
impl System for MockSystem {
    fn init(&mut self) {
        self.value = 1;
    }

    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("mock/inputs")); 
        topics
    }

    fn execute<'a>(&mut self, inputs: &'a victory_data_store::database::DataView, dt: victory_time_rs::Timespan) -> victory_data_store::database::DataView {

        let inputs:MockSystemData = inputs.get_latest(&TopicKey::from_str("mock/inputs")).unwrap();
        self.value += inputs.value;
        let mut outputs = DataView::new();
        outputs.add_latest(&TopicKey::from_str("mock/outputs"), MockSystemData { value: self.value });
        outputs
    }

    fn cleanup(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use rapier3d::data;
    use victory_time_rs::Timespan;

    use super::*;

    use crate::system::mock::MockSystem;

    #[test]
    fn test_mock_system() {
        let mut system = MockSystem::new();
        system.init();
        assert_eq!(system.value, 1);
        let inputs = MockSystemData { value: 1 };
        let mut data_view = DataView::new();
        data_view.add_latest(&TopicKey::from_str("mock/inputs"), inputs);

        let out_dv:DataView = system.execute(&data_view, Timespan::zero());
        let outputs:MockSystemData = out_dv.get_latest(&TopicKey::from_str("mock/outputs")).unwrap();
        assert_eq!(outputs.value, 2);
    }
}
