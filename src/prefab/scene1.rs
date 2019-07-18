use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{
        Entity,
    },
    Error,
};
use serde::{Deserialize, Serialize};

use crate::prefab::Human;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub enum Scene1 {
    Player {
        data: Human
    }
}

