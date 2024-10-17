
use std::any::{self, Any};

use nalgebra::Owned;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use victory_data_store::{database::Datastore, topics::TopicKey};
use victory_time_rs::{Timepoint, Timespan};

use crate::state;
pub mod mock;
pub mod quad;
pub mod simulation;
pub mod viz;
