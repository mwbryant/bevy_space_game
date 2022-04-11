use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gas {
    Oxygen,
    Nitrogen,
    CarbonDioxide,
    Helium,
    Hydrogen,
    WaterVapor,
}

impl Default for Gas {
    fn default() -> Gas {
        Gas::Nitrogen
    }
}
