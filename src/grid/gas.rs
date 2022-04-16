use crate::prelude::*;

use super::{GasPlugin, WallGrid, GRID_SIZE};

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gas_grid)
            .add_system(diffuse_gas_grid)
            .add_system(gas_wall_connection)
            .add_system(gas_clamp);
    }
}

impl Default for Gas {
    fn default() -> Gas {
        Gas::None
    }
}

fn gas_clamp(mut gas_query: Query<&mut GasTile, Changed<GasTile>>) {
    for mut tile in gas_query.iter_mut() {
        for pressure in tile.pressure.iter_mut() {
            *pressure = pressure.clamp(0.0, 200.0);
        }
    }
}

fn gas_wall_connection(mut gas_query: Query<&mut GasGrid>, wall_query: Query<&WallGrid>) {
    //TODO handle multi grids/walls
    //maybe a struct linking the 2
    //gas grids should be made by or from wall grid
    let walls = wall_query.single();
    let mut grid = gas_query.single_mut();
    for (i, row) in walls.walls.iter().enumerate() {
        for (j, ent) in row.iter().enumerate() {
            grid.wall_mask[i][j] = ent.is_some();
        }
    }
}

fn diffuse(
    i: usize,
    j: usize,
    x0: &[[GasTile; GRID_SIZE]; GRID_SIZE],
    x: &mut [[GasTile; GRID_SIZE]; GRID_SIZE],
    wall_mask: &[[bool; GRID_SIZE]; GRID_SIZE],
    a: f64,
    gas: usize,
) {
    let up_wall = j == 0 || wall_mask[i][j - 1];
    let down_wall = j == GRID_SIZE - 1 || wall_mask[i][j + 1];
    let left_wall = i == 0 || wall_mask[i - 1][j];
    let right_wall = i == GRID_SIZE - 1 || wall_mask[i + 1][j];
    let wall_count =
        !up_wall as isize + !down_wall as isize + !left_wall as isize + !right_wall as isize;
    let mut new_x = x0[i][j].pressure[gas];
    if !left_wall {
        new_x += a * x[i - 1][j].pressure[gas];
    }
    if !right_wall {
        new_x += a * x[i + 1][j].pressure[gas];
    }
    if !up_wall {
        new_x += a * x[i][j - 1].pressure[gas];
    }
    if !down_wall {
        new_x += a * x[i][j + 1].pressure[gas];
    }
    new_x /= 1.0 + wall_count as f64 * a;
    *x[i][j].pressure.get_mut(gas).unwrap() = new_x;
}

//TODO optimize, grids probably dont need be all the same massive size, maybe map is many smaller grids with interfaces
//Thanks Jos Stam! http://graphics.cs.cmu.edu/nsp/course/15-464/Fall09/papers/StamFluidforGames.pdf
fn diffuse_gas_grid(
    mut tile_query: Query<(&mut GasTile, &mut TextureAtlasSprite)>,
    grid_query: Query<&GasGrid>,
    time: Res<Time>,
) {
    for grid in grid_query.iter() {
        //Copy tiles XXX FIXME bad
        let mut x0 = [[GasTile::default(); GRID_SIZE]; GRID_SIZE];
        let mut x = [[GasTile::default(); GRID_SIZE]; GRID_SIZE];
        #[allow(clippy::needless_range_loop)]
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let (gas, _) = tile_query.get(grid.grid[i][j]).unwrap();
                x0[i][j] = *gas;
            }
        }

        let a = time.delta_seconds() as f64 * 0.003 * (GRID_SIZE * GRID_SIZE) as f64;
        for _k in 0..40 {
            for i in 0..(GRID_SIZE) {
                for j in 0..(GRID_SIZE) {
                    for gas in 0..GAS_COUNT {
                        diffuse(i, j, &x0, &mut x, &grid.wall_mask, a, gas);
                    }
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let (mut gas, mut sprite) = tile_query.get_mut(grid.grid[i][j]).unwrap();
                gas.pressure = x[i][j].pressure;
                if !grid.wall_mask[i][j] {
                    sprite.color = Color::rgba(
                        (x[i][j].pressure[Gas::Oxygen as usize] as f32 / 100.0).clamp(0.0, 1.0),
                        (x[i][j].pressure[Gas::Nitrogen as usize] as f32 / 100.0).clamp(0.0, 1.0),
                        (x[i][j].pressure[Gas::CarbonDioxide as usize] as f32 / 100.0)
                            .clamp(0.0, 1.0),
                        0.25,
                    );
                } else {
                    sprite.color = Color::rgba(0.1, 0.1, 0.1, 0.8);
                }
            }
        }
    }
}

fn spawn_gas_grid(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut gas_grid = GasGrid {
        //XXX find a better default entity
        grid: [[Entity::from_raw(0); GRID_SIZE]; GRID_SIZE],
        wall_mask: [[false; GRID_SIZE]; GRID_SIZE],
        tile_size: 32.0,
    };
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let sprite = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                0,
                Color::rgba(0.9, 0.1, 0.1, 0.10),
                Vec3::new(
                    x as f32 * gas_grid.tile_size,
                    y as f32 * gas_grid.tile_size,
                    900.0,
                ),
                Vec3::splat(1.0),
            );
            let mut gas = GasTile::default();
            gas.pressure[Gas::Oxygen as usize] = 100.0;
            gas.pressure[Gas::Nitrogen as usize] = 20.0;
            gas_grid.grid[x][y] = commands.entity(sprite).insert(gas).id()
        }
    }

    let children: Vec<Entity> = gas_grid.grid.iter().flatten().copied().collect();
    commands
        .spawn()
        .push_children(&children)
        .insert(Transform::from_xyz(
            -(GRID_SIZE as f32 * gas_grid.tile_size) / 2.0,
            -(GRID_SIZE as f32 * gas_grid.tile_size) / 2.0,
            0.0,
        ))
        .insert(gas_grid)
        .insert(GlobalTransform::default())
        .insert(Name::new("Gas Grid"));
}
