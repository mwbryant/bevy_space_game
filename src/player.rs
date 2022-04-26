use bevy::{
    render::camera::Camera2d,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Component, Inspectable, Deserialize, Clone, Copy)]
pub struct Player {
    move_speed: f32,
    breath_rate: f32,
    hitbox: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_terminal)
            .add_system(player_breath)
            .add_system(camera_follow)
            .add_system(player_collision.after(player_movement))
            .register_inspectable::<Player>()
            .add_system(player_movement);
    }
}
fn player_collision(
    mut player_query: Query<(&mut Transform, &Player)>,
    grid_query: Query<&WallGrid>,
    wall_query: Query<&GlobalTransform, (With<Wall>, Without<Player>)>,
) {
    let grid = grid_query.single();
    let (mut player_transform, player) = player_query.single_mut();
    for wall_transform in wall_query.iter() {
        match collide(
            player_transform.translation,
            player.hitbox,
            wall_transform.translation,
            Vec2::splat(grid.tile_size),
        ) {
            Some(Collision::Left) => {
                player_transform.translation.x =
                    wall_transform.translation.x - grid.tile_size / 2.0 - player.hitbox.x / 2.0
            }
            Some(Collision::Right) => {
                player_transform.translation.x =
                    wall_transform.translation.x + grid.tile_size / 2.0 + player.hitbox.x / 2.0
            }
            Some(Collision::Top) => {
                player_transform.translation.y =
                    wall_transform.translation.y + grid.tile_size / 2.0 + player.hitbox.y / 2.0
            }
            Some(Collision::Bottom) => {
                player_transform.translation.y =
                    wall_transform.translation.y - grid.tile_size / 2.0 - player.hitbox.y / 2.0
            }
            _ => {}
        }
    }
}

fn player_breath(
    player_query: Query<(&GlobalTransform, &Player)>,
    gas_query: Query<(&GasGrid, &GlobalTransform), Without<Player>>,
    mut tile_query: Query<&mut GasMixture>,
    mut text_query: Query<&mut Text, With<GasText>>,
    time: Res<Time>,
) {
    let (transform, player) = player_query.single();
    let (gas_grid, gas_transform) = gas_query.single();

    let x_index = ((transform.translation.x - gas_transform.translation.x
        + 0.5 * gas_grid.tile_size)
        / gas_grid.tile_size) as usize;
    let y_index = ((transform.translation.y - gas_transform.translation.y
        + 0.5 * gas_grid.tile_size)
        / gas_grid.tile_size) as usize;

    let tile = gas_grid.grid[x_index][y_index];

    let mut tile = tile_query.get_mut(tile).unwrap();
    let to_breath = (player.breath_rate * time.delta_seconds()) as f64;
    text_query.single_mut().sections[0].value = format!(
        "At Player: {:.0}° \nO2: {:.0} moles, {:.2} atms\nCO2: {:.0} moles, {:.2} atms",
        tile.temperature,
        tile.amount[1],
        tile.get_pressure(Gas::Oxygen),
        tile.amount[Gas::CarbonDioxide as usize],
        tile.get_pressure(Gas::CarbonDioxide),
    );
    if to_breath < tile.amount[Gas::Oxygen as usize] {
        tile.amount[Gas::Oxygen as usize] -= to_breath;
        tile.amount[Gas::CarbonDioxide as usize] += to_breath;
    } else {
        tile.amount[Gas::CarbonDioxide as usize] += tile.amount[Gas::Oxygen as usize];
        tile.amount[Gas::Oxygen as usize] = 0.0;
        println!("Player suffocating!");
    }
}

fn spawn_terminal(mut commands: Commands, assets: Res<AssetServer>) {
    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::Terminal(
            Orientation::Left,
        )))
        .id();
    commands
        .entity(ent)
        .insert(Name::new("Terminal"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(-32.0, 32.0, 100.0));

    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::Furance(true)))
        .id();
    commands
        .entity(ent)
        .insert(Name::new("Furance"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(-220.0, 32.0, 100.0));
    spawn_particle_spawner(
        &mut commands,
        "config/fire_particle_spawner.ron",
        Vec3::new(-230.0, 41.5, 900.0),
        &assets,
    );

    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::Terminal(
            Orientation::Right,
        )))
        .id();
    commands
        .entity(ent)
        .insert(Name::new("Terminal"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(32.0, -32.0, 100.0));

    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::Terminal(
            Orientation::Down,
        )))
        .id();
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

fn camera_follow(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
) {
    let player = player_query.single();
    let mut camera = camera_query.single_mut();
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Graphic::Player(Orientation::Down))
        .insert(comp_from_config!(Player))
        .insert(Transform::from_xyz(0.0, 0.0, 500.0))
        .insert(Clickable::default())
        .insert(Name::new("Player"));
}
