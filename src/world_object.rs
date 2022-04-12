use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::assets::Orientation;

#[derive(Inspectable, Deserialize, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum WallConnection {
    None,
    One,
    Corner,
    Cross,
    T,
    All,
}

impl Default for WallConnection {
    fn default() -> Self {
        WallConnection::None
    }
}

#[derive(
    Inspectable, Deserialize, Component, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy,
)]
pub enum WorldObject {
    Canister,
    CanisterMachine,
    SmallLabel(usize),
    Terminal(Orientation),
    Wall(WallConnection),
}

impl Default for WorldObject {
    fn default() -> Self {
        WorldObject::Canister
    }
}
