use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    Error,
};
use serde::{Deserialize, Serialize};

use crate::prefab;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub enum Scene1 {
    Human { data: prefab::Creature },
    Cyclops { data: prefab::Creature },
}
