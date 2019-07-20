use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Movable {
    pub speed: f64,
    pub direction_x: f64,
    pub direction_y: f64,
}
