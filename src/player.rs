use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    comp_from_config,
};

#[derive(Component, Deserialize, Clone, Copy)]
pub struct Player {
    move_speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += player.move_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.move_speed * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.move_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.move_speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
    transform.translation = target;
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::BLUE,
        Vec3::new(0.0, 0.0, 500.0),
        Vec3::splat(1.0),
    );

    commands.entity(player).insert(comp_from_config!(Player));
}
