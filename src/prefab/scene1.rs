use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{
        Entity,
    },
    Error,
};
use serde::{Deserialize, Serialize};

use crate::prefab::CreaturePrefab;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub enum Scene1Prefab {
    Human {
        data: CreaturePrefab
    },
    Cyclops {
        data: CreaturePrefab
    }
}
