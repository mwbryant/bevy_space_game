use bevy_inspector_egui::Inspectable;
use serde::Deserialize;

#[derive(Inspectable, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gas {
    None,
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
