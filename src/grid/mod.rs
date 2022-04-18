use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

mod canister;
mod gas;
mod wall;

pub const GRID_SIZE: usize = 50;
pub const IDEAL_GAS_CONST: f64 = 8.314462618153 /* m^3*Pa/K*mol */ * (1.0/101325.0); //atm/Pa
pub const TILE_VOLUME: f64 = 2.0; // m^3

pub const GAS_COUNT: usize = 7;
//Possible gas types, GasTiles contain all of these
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

impl Default for Gas {
    fn default() -> Gas {
        Gas::None
    }
}

/// Component: Tile containing moles of gas and the temperature
#[derive(Component, Clone, Copy, Default, Inspectable, Deserialize)]
pub struct GasMixture {
    pub amount: [f64; GAS_COUNT],
    pub temperature: f64,
}

impl GasMixture {
    pub fn single_gas(gas: Gas, amount: f32, temperature: f32) -> GasMixture {
        let mut mixture = GasMixture::default();
        mixture.amount[gas as usize] = amount as f64;
        mixture.temperature = temperature as f64;
        mixture
    }

    #[allow(dead_code)]
    pub fn get_pressure(&self, gas: Gas) -> f64 {
        self.amount[gas as usize] * self.temperature * IDEAL_GAS_CONST / TILE_VOLUME
    }
    pub fn get_total_pressure(&self) -> f32 {
        let mut total = 0.0;
        for i in 0..GAS_COUNT {
            total += self.amount[i] * self.temperature * IDEAL_GAS_CONST / TILE_VOLUME
        }
        total as f32
    }
}

/// Component: Grid holding GasTile entities, walls must be registered here to affect gases
#[derive(Component)]
pub struct GasGrid {
    pub grid: [[Entity; GRID_SIZE]; GRID_SIZE],
    pub wall_mask: [[bool; GRID_SIZE]; GRID_SIZE],
    pub tile_size: f32,
}

/// Component: Grid holding Wall entities
#[derive(Component)]
pub struct WallGrid {
    pub tile_size: f32,
    pub walls: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

/// Tag
#[derive(Component)]
pub struct Wall;

#[derive(Component, Default, Inspectable, Deserialize)]
//TODO mols, temp, pressure
///
pub struct Canister {
    gases: GasMixture,
    pub volume: f32,
    pub max_pressure: f32,
}

#[derive(Component, Deserialize, Default, Inspectable)]
pub struct CanisterMachine {
    canisters: [Canister; 4],
}

#[derive(Component, Inspectable)]
pub enum GasVisualizationSettings {
    Moles,
    Pressure,
    Temperature,
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
