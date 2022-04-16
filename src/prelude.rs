pub use bevy::prelude::*;
pub use bevy::utils::HashMap;
pub use bevy_inspector_egui::Inspectable;
pub use serde::{Deserialize, Serialize};

pub use crate::graphics::*;
pub use crate::grid::*;

pub use crate::comp_from_config;

#[macro_export]
macro_rules! comp_from_config {
    ($comp_type:ty) => {
        ron::from_str::<$comp_type>(
            &std::fs::read_to_string(
                "config/".to_owned() + &stringify!($comp_type).to_lowercase() + ".ron",
            )
            .unwrap(),
        )
        .expect(&("Failed to load ".to_owned() + &stringify!($comp_type).to_lowercase() + ".ron"))
    };
    ($comp_type:ty,$file_name:expr) => {
        ron::from_str::<$comp_type>(&std::fs::read_to_string(($file_name)).unwrap())
            .expect(&("Failed to load ".to_owned() + $file_name))
    };
}
