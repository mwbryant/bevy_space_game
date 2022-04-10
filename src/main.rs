#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]
use std::fs;

use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod ascii;
mod debug;
mod player;
mod utils;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::{Player, PlayerPlugin};
use ron::from_str;
use serde::{Deserialize, Serialize};

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Bevy Tutorial".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(AsciiPlugin { tile_size: 0.1 })
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_system(save_game)
        .add_system(load_game)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    //Set the camera to have normalized coordinates of y values -1 to 1
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    //Force the camera to use our settings
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    player_translation: Vec3,
}

fn save_game(player_query: Query<&Transform, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
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
        let mut transform = player_query.single_mut();

        let save_file: SaveFile =
            from_str(&fs::read_to_string("saves/save1.ron").unwrap()).expect("Failed to load ron");

        transform.translation = save_file.player_translation;
    }
}
