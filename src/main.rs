#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use std::fs;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
//use bevy_loading::prelude::*;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod debug;
mod graphics;
mod grid;
mod mouse;
mod player;
mod prelude;

use debug::DebugPlugin;
use graphics::GraphicsPluginGroup;
use grid::GridPluginGroup;
use mouse::MousePlugin;
use player::{Player, PlayerPlugin};
use ron::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
enum AppState {
    Game,
}

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Bevy Space Game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugins(GridPluginGroup)
        .add_plugins(GraphicsPluginGroup)
        .add_state(AppState::Game)
        .add_plugin(MousePlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_system(save_game)
        .add_system(load_game)
        //.add_system(slow_down)
        .run();
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(0.100));
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let size = 450.0 / 2.0;
    //let size = 300.0 / 2.0;
    //let size = 150.0 / 2.0;

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
