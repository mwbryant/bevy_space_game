use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    assets::{spawn_sprite, Graphic, Graphics, Orientation},
    comp_from_config,
    pixel_perfect_selection::Clickable,
    world_object::WorldObject,
};

#[derive(Component, Deserialize, Clone, Copy)]
pub struct Player {
    move_speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_terminal)
            .add_system(player_movement);
    }
}

fn spawn_terminal(mut commands: Commands, graphics: Res<Graphics>) {
    let ent = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::Terminal(Orientation::Left)),
    );
    commands
        .entity(ent)
        .insert(Name::new("Terminal"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(-32.0, 32.0, 100.0));

    let ent = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::Terminal(Orientation::Right)),
    );
    commands
        .entity(ent)
        .insert(Name::new("Terminal"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(32.0, -32.0, 100.0));

    let ent = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::Terminal(Orientation::Down)),
    );
    commands
        .entity(ent)
        .insert(Name::new("Terminal"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(-32.0, -32.0, 100.0));
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform, &mut Graphic)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform, mut graphic) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        *graphic = Graphic::Player(Orientation::Up);
        y_delta += player.move_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        *graphic = Graphic::Player(Orientation::Down);
        y_delta -= player.move_speed * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        *graphic = Graphic::Player(Orientation::Left);
        x_delta -= player.move_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        *graphic = Graphic::Player(Orientation::Right);
        x_delta += player.move_speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
    transform.translation = target;
}

fn spawn_player(mut commands: Commands, graphics: Res<Graphics>) {
    let player = spawn_sprite(&mut commands, &graphics, Graphic::Player(Orientation::Down));

    commands
        .entity(player)
        .insert(comp_from_config!(Player))
        .insert(Transform::from_xyz(0.0, 0.0, 500.0))
        .insert(Clickable::default())
        .insert(Name::new("Player"));
}
