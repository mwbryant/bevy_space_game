use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{mouse::MousePosition, prelude::*};

use super::{Wall, WallGrid, WallPlugin, GRID_SIZE};

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls)
            .add_system_to_stage(CoreStage::PostUpdate, wall_update)
            .add_system(mouse_create_wall);
    }
}

fn mouse_create_wall(
    mut commands: Commands,
    mut wall_query: Query<(Entity, &GlobalTransform, &mut WallGrid)>,
    mouse: Res<MousePosition>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let (ent, transform, mut grid) = wall_query.iter_mut().next().unwrap();
    let target = get_mouse_tile(mouse.0, transform.translation.truncate(), grid.tile_size);

    if mouse_input.pressed(MouseButton::Left) {
        let new_wall = create_wall(&mut commands, &mut grid, target.0, target.1);
        if let Some(new_wall) = new_wall {
            commands.entity(ent).add_child(new_wall);
        }
    }

    if mouse_input.pressed(MouseButton::Right) {
        if let Some(wall) = grid.walls[target.0][target.1] {
            commands.entity(wall).despawn_recursive();
            grid.walls[target.0][target.1] = None;
        }
    }
}

fn get_mouse_tile(position: Vec2, map_pos: Vec2, tile_size: f32) -> (usize, usize) {
    let x = (position.x - map_pos.x + tile_size * 0.5) / tile_size;
    let y = (position.y - map_pos.y + tile_size * 0.5) / tile_size;
    (x as usize, y as usize)
}

//XXX creates updates grid before entity is actually spawned...
fn create_wall(commands: &mut Commands, grid: &mut WallGrid, x: usize, y: usize) -> Option<Entity> {
    if grid.walls[x][y] == None {
        let wall = commands
            .spawn()
            .insert(Graphic::WorldObject(WorldObject::Wall(
                WallConnection::None,
            )))
            //FIXME assumes wall size is 32
            .insert(Transform::from_xyz(
                x as f32 * grid.tile_size,
                y as f32 * grid.tile_size,
                0.0,
            ))
            .insert(Wall)
            .insert(Name::new("Wall"))
            .id();
        grid.walls[x][y] = Some(wall);
        return Some(wall);
    }
    None
}

fn create_room(
    commands: &mut Commands,
    grid: &mut WallGrid,
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
) {
    let y1 = 0;
    let y2 = height - 1;
    for x in 0..width {
        create_wall(commands, grid, x + x_offset, y1 + y_offset);
        create_wall(commands, grid, x + x_offset, y2 + y_offset);
    }

    let x1 = 0;
    let x2 = width - 1;
    for y in 1..(height - 1) {
        create_wall(commands, grid, x1 + x_offset, y + y_offset);
        create_wall(commands, grid, x2 + x_offset, y + y_offset);
    }
}

fn spawn_walls(mut commands: Commands) {
    let mut grid = WallGrid {
        tile_size: 32.0,
        walls: [[None; GRID_SIZE]; GRID_SIZE],
    };
    create_room(&mut commands, &mut grid, 20, 20, 9, 9);

    let mut to_add: Vec<Entity> = Vec::new();
    //Janky but I cant work out the iterator over Optional
    for ent in grid.walls.iter().flatten().flatten() {
        to_add.push(*ent);
    }

    commands
        .spawn()
        .push_children(&to_add)
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(
            -(GRID_SIZE as f32 * grid.tile_size) / 2.0,
            -(GRID_SIZE as f32 * grid.tile_size) / 2.0,
            100.0,
        ))
        .insert(grid)
        .insert(Name::new("Wall Grid"));
}

//TODO if a wall entity is not in grid warn!
//XXX Becareful making things children of walls as this rotates them...
fn wall_update(
    mut wall_query: Query<(&mut Graphic, &mut Transform), With<Wall>>,
    grid_query: Query<&WallGrid, Changed<WallGrid>>,
) {
    for grid in grid_query.iter() {
        for (i, row) in grid.walls.iter().enumerate() {
            for (j, ent) in row.iter().enumerate() {
                if let Some(ent) = ent {
                    let left = i > 0 && grid.walls[i - 1][j].is_some();
                    let right = i < GRID_SIZE - 1 && grid.walls[i + 1][j].is_some();
                    let down = j > 0 && grid.walls[i][j - 1].is_some();
                    let up = j < GRID_SIZE - 1 && grid.walls[i][j + 1].is_some();

                    if let Ok((mut graphic, mut transform)) = wall_query.get_mut(*ent) {
                        match (left, right, up, down) {
                            (false, false, false, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::None));
                                transform.rotation = Quat::from_rotation_z(0.0);
                            }
                            //Down
                            (false, false, false, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                                transform.rotation = Quat::from_rotation_z(-90.0 * PI / 180.0);
                            }
                            //Up
                            (false, false, true, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                                transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                            }
                            //Up Down
                            (false, false, true, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Cross));
                                transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                            }
                            //Right
                            (false, true, false, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                                transform.rotation = Quat::from_rotation_z(0.0 * PI / 180.0);
                            }
                            //Down right
                            (false, true, false, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                                transform.rotation = Quat::from_rotation_z(0.0 * PI / 180.0);
                            }
                            //Up right
                            (false, true, true, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                                transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                            }
                            //Up down right
                            (false, true, true, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                                transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                            }
                            //Left
                            (true, false, false, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                                transform.rotation = Quat::from_rotation_z(180.0 * PI / 180.0);
                            }
                            //Left Down
                            (true, false, false, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                                transform.rotation = Quat::from_rotation_z(-90.0 * PI / 180.0);
                            }
                            //left up
                            (true, false, true, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                                transform.rotation = Quat::from_rotation_z(180.0 * PI / 180.0);
                            }
                            //left up down
                            (true, false, true, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                                transform.rotation = Quat::from_rotation_z(-90.0 * PI / 180.0);
                            }
                            //left right
                            (true, true, false, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::Cross));
                                transform.rotation = Quat::from_rotation_z(0.0 * PI / 180.0);
                            }
                            //left right down
                            (true, true, false, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                                transform.rotation = Quat::from_rotation_z(0.0 * PI / 180.0);
                            }
                            //left right up
                            (true, true, true, false) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                                transform.rotation = Quat::from_rotation_z(180.0 * PI / 180.0);
                            }
                            (true, true, true, true) => {
                                *graphic =
                                    Graphic::WorldObject(WorldObject::Wall(WallConnection::All));
                                transform.rotation = Quat::from_rotation_z(0.0 * PI / 180.0);
                            }
                        }
                    }
                }
            }
        }
    }
}
