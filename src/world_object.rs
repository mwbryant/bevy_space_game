use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::Orientation;

#[derive(Deserialize, Component, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum WorldObject {
    Canister,
    CanisterMachine,
    SmallLabel(usize),
    Terminal(Orientation),
}
