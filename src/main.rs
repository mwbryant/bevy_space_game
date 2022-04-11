#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]
use std::fs;

use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod ascii;
mod assets;
mod canisters;
mod debug;
mod gas;
mod player;
mod utils;
mod world_object;

use ascii::AsciiPlugin;
use assets::GameAssetsPlugin;
use canisters::CanisterPlugin;
use debug::DebugPlugin;
use player::{Player, PlayerPlugin};
use ron::from_str;
use serde::{Deserialize, Serialize};

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Bevy Space Game".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(CanisterPlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(AsciiPlugin { tile_size: 32.0 })
        .add_plugin(PlayerPlugin)
        .add_plugin(GameAssetsPlugin)
        .add_system(save_game)
        .add_system(load_game)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    //let size = 450.0 / 2.0;
    let size = 300.0 / 2.0;
    //let size = 60.0 / 2.0;

    camera.orthographic_projection.right = size * RESOLUTION;
    camera.orthographic_projection.left = -size * RESOLUTION;

    camera.orthographic_projection.top = size;
    camera.orthographic_projection.bottom = -size;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    player_translation: Vec3,
}

fn save_game(player_query: Query<&Transform, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::O) {
        let transform = player_query.single();

        let save_file = SaveFile {
            player_translation: transform.translation,
        };

        fs::write(
            "saves/save1.ron",
            ron::ser::to_string_pretty(&save_file, ron::ser::PrettyConfig::default()).unwrap(),
        )
        .unwrap();
    }
}

fn load_game(mut player_query: Query<&mut Transform, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::L) {
        let save_file: SaveFile =
            from_str(&fs::read_to_string("saves/save1.ron").unwrap()).expect("Failed to load ron");

        let mut transform = player_query.single_mut();
        transform.translation = save_file.player_translation;
    }
}
