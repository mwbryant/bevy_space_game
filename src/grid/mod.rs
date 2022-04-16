use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

mod canister;
mod gas;
mod wall;

pub const GRID_SIZE: usize = 50;
pub const GAS_COUNT: usize = 7;

#[derive(Inspectable, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gas {
    None = 0,
    Oxygen,
    Nitrogen,
    CarbonDioxide,
    Helium3,
    Hydrogen,
    WaterVapor,
}

#[derive(Component, Clone, Copy, Default)]
pub struct GasTile {
    pub pressure: [f64; GAS_COUNT],
}

#[derive(Component)]
pub struct GasGrid {
    pub grid: [[Entity; GRID_SIZE]; GRID_SIZE],
    pub wall_mask: [[bool; GRID_SIZE]; GRID_SIZE],
    pub tile_size: f32,
}

#[derive(Component)]
pub struct WallGrid {
    tile_size: f32,
    pub walls: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

#[derive(Component)]
pub struct Wall;

#[derive(Component, Default, Inspectable, Deserialize)]
//TODO mols, temp, pressure
pub struct Canister {
    percent_full: f32,
    gas: Gas,
}

#[derive(Component, Deserialize, Default, Inspectable)]
pub struct CanisterMachine {
    canisters: [Canister; 4],
}

struct WallPlugin;
struct GasPlugin;
struct CanisterPlugin;

pub struct GridPluginGroup;

impl PluginGroup for GridPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(WallPlugin).add(GasPlugin).add(CanisterPlugin);
    }
}

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
