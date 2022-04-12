use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    assets::{spawn_sprite, Graphic, Graphics},
    world_object::{WallConnection, WorldObject},
    GRID_SIZE,
};

#[derive(Component)]
pub struct WallGrid {
    walls: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

#[derive(Component)]
pub struct Wall;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls).add_system(wall_update);
    }
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
                    let (mut graphic, mut transform) =
                        wall_query.get_mut(*ent).expect("Wall is not in query");
                    match (left, right, up, down) {
                        (false, false, false, false) => {
                            *graphic =
                                Graphic::WorldObject(WorldObject::Wall(WallConnection::None));
                            transform.rotation = Quat::from_rotation_z(0.0);
                        }
                        //Down
                        (false, false, false, true) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                            transform.rotation = Quat::from_rotation_z(-90.0 * PI / 180.0);
                        }
                        //Up
                        (false, false, true, false) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
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
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
                        }
                        //Down right
                        (false, true, false, true) => {
                            *graphic =
                                Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                        }
                        //Up right
                        (false, true, true, false) => {
                            *graphic =
                                Graphic::WorldObject(WorldObject::Wall(WallConnection::Corner));
                            transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                        }
                        //Up down right
                        (false, true, true, true) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                            transform.rotation = Quat::from_rotation_z(90.0 * PI / 180.0);
                        }
                        //Left
                        (true, false, false, false) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::One));
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
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                            transform.rotation = Quat::from_rotation_z(-90.0 * PI / 180.0);
                        }
                        //left right
                        (true, true, false, false) => {
                            *graphic =
                                Graphic::WorldObject(WorldObject::Wall(WallConnection::Cross));
                        }
                        //left right down
                        (true, true, false, true) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                        }
                        //left right up
                        (true, true, true, false) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::T));
                            transform.rotation = Quat::from_rotation_z(180.0 * PI / 180.0);
                        }
                        (true, true, true, true) => {
                            *graphic = Graphic::WorldObject(WorldObject::Wall(WallConnection::All));
                        }
                    }
                }
            }
        }
    }
}

fn create_wall(
    commands: &mut Commands,
    grid: &mut WallGrid,
    x: usize,
    y: usize,
    graphics: &Graphics,
) {
    if grid.walls[x][y] == None {
        let wall = spawn_sprite(
            commands,
            graphics,
            Graphic::WorldObject(WorldObject::Wall(WallConnection::None)),
        );
        commands
            .entity(wall)
            //FIXME assumes wall size is 32
            .insert(Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 0.0))
            .insert(Wall)
            .insert(Name::new("Wall"));
        grid.walls[x][y] = Some(wall);
    }
}

fn create_room(
    commands: &mut Commands,
    grid: &mut WallGrid,
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
    graphics: &Graphics,
) {
    let y1 = 0;
    let y2 = height - 1;
    for x in 0..width {
        create_wall(commands, grid, x + x_offset, y1 + y_offset, graphics);
        create_wall(commands, grid, x + x_offset, y2 + y_offset, graphics);
    }

    let x1 = 0;
    let x2 = width - 1;
    for y in 1..(height - 1) {
        create_wall(commands, grid, x1 + x_offset, y + y_offset, graphics);
        create_wall(commands, grid, x2 + x_offset, y + y_offset, graphics);
    }
}

fn spawn_walls(mut commands: Commands, graphics: Res<Graphics>) {
    let mut grid = WallGrid {
        walls: [[None; GRID_SIZE]; GRID_SIZE],
    };
    create_room(&mut commands, &mut grid, 1, 3, 10, 5, &graphics);
    create_room(&mut commands, &mut grid, 3, 1, 3, 7, &graphics);
    create_room(&mut commands, &mut grid, 9, 0, 15, 7, &graphics);

    let mut to_add: Vec<Entity> = Vec::new();
    //Janky but I cant work out the iterator over Optional
    for ent in grid.walls.iter().flatten().flatten() {
        to_add.push(*ent);
    }

    commands
        .spawn()
        .insert(grid)
        .push_children(&to_add)
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(-100.0, -100.0, 100.0))
        .insert(Name::new("Wall Grid"));
}
