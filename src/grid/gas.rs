use bevy_inspector_egui::RegisterInspectable;

use crate::prelude::*;

use super::{GasPlugin, WallGrid, GRID_SIZE};

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        //TODO use bevy 0.7 label systems
        app.add_startup_system(spawn_gas_grid)
            .add_system(diffuse_gas_grid)
            .add_system(gas_wall_connection)
            //.add_system(heat_gas)
            //.add_system(print_total)
            .add_system(gas_clamp)
            .register_inspectable::<GasVisualizationSettings>()
            .register_inspectable::<GasMixture>();
    }
}

fn gas_clamp(mut gas_query: Query<&mut GasMixture, Changed<GasMixture>>) {
    for mut tile in gas_query.iter_mut() {
        for amount in tile.amount.iter_mut() {
            *amount = amount.clamp(0.0, 200.0);
        }
    }
}

#[allow(dead_code)]
fn print_total(tile_query: Query<&GasMixture>, grid_query: Query<&GasGrid>) {
    let grid = grid_query.iter().next().unwrap();
    let mut total = 0.0;
    for tile in grid.grid.iter().flatten() {
        let tile = tile_query.get(*tile).unwrap();
        total += tile.amount[1];
    }
    println!("Total {:.1}", total);
}

#[allow(dead_code)]
fn heat_gas(mut tile_query: Query<&mut GasMixture>, grid_query: Query<&GasGrid>, time: Res<Time>) {
    if time.time_since_startup().as_secs() < 5 {
        let grid = grid_query.iter().next();
        let mut tile = tile_query.get_mut(grid.unwrap().grid[23][23]).unwrap();
        tile.temperature += 500.0 * time.delta_seconds() as f64;
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

//XXX does temp diffuse based on the material...
fn diffuse_temperature(
    i: usize,
    j: usize,
    x0: &[[GasMixture; GRID_SIZE]; GRID_SIZE],
    x: &mut [[GasMixture; GRID_SIZE]; GRID_SIZE],
    wall_mask: &[[bool; GRID_SIZE]; GRID_SIZE],
    a: f64,
) {
    let up_wall = j == 0 || wall_mask[i][j - 1];
    let down_wall = j == GRID_SIZE - 1 || wall_mask[i][j + 1];
    let left_wall = i == 0 || wall_mask[i - 1][j];
    let right_wall = i == GRID_SIZE - 1 || wall_mask[i + 1][j];
    let wall_count =
        !up_wall as isize + !down_wall as isize + !left_wall as isize + !right_wall as isize;
    let mut new_x = x0[i][j].temperature;
    if !left_wall {
        new_x += a * x[i - 1][j].temperature;
    }
    if !right_wall {
        new_x += a * x[i + 1][j].temperature;
    }
    if !up_wall {
        new_x += a * x[i][j - 1].temperature;
    }
    if !down_wall {
        new_x += a * x[i][j + 1].temperature;
    }
    new_x /= 1.0 + wall_count as f64 * a;
    x[i][j].temperature = new_x;
}

//TODO should diffuse pressure but thats a bit complex, just multipling rate by temperature breaks the invariant that a tile loses as much as another gains
fn diffuse_moles(
    i: usize,
    j: usize,
    x0: &[[GasMixture; GRID_SIZE]; GRID_SIZE],
    x: &mut [[GasMixture; GRID_SIZE]; GRID_SIZE],
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
    let mut new_x = x0[i][j].amount[gas];
    if !left_wall {
        new_x += a * x[i - 1][j].amount[gas] * x[i - 1][j].temperature;
    }
    if !right_wall {
        new_x += a * x[i + 1][j].amount[gas] * x[i + 1][j].temperature;
    }
    if !up_wall {
        new_x += a * x[i][j - 1].amount[gas] * x[i][j - 1].temperature;
    }
    if !down_wall {
        new_x += a * x[i][j + 1].amount[gas] * x[i][j + 1].temperature;
    }
    new_x /= 1.0 + wall_count as f64 * (a * x[i][j].temperature);
    *x[i][j].amount.get_mut(gas).unwrap() = new_x;
}

//TODO optimize, grids probably dont need be all the same massive size, maybe map is many smaller grids with interfaces
//Thanks Jos Stam! http://graphics.cs.cmu.edu/nsp/course/15-464/Fall09/papers/StamFluidforGames.pdf
fn diffuse_gas_grid(
    mut tile_query: Query<(&mut GasMixture, &mut TextureAtlasSprite)>,
    grid_query: Query<(&GasGrid, &GasVisualizationSettings)>,
    time: Res<Time>,
) {
    for (grid, visualization) in grid_query.iter() {
        //Copy tiles XXX FIXME bad
        let mut x0 = [[GasMixture::default(); GRID_SIZE]; GRID_SIZE];
        let mut x = [[GasMixture::default(); GRID_SIZE]; GRID_SIZE];
        #[allow(clippy::needless_range_loop)]
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let (gas, _) = tile_query.get(grid.grid[i][j]).unwrap();
                x0[i][j] = *gas;
            }
        }

        let a = time.delta_seconds() as f64 * 0.0006 * (GRID_SIZE * GRID_SIZE) as f64;
        for _k in 0..30 {
            for i in 0..(GRID_SIZE) {
                for j in 0..(GRID_SIZE) {
                    diffuse_temperature(i, j, &x0, &mut x, &grid.wall_mask, a);
                }
            }
        }

        let a = time.delta_seconds() as f64 * 0.005;
        for _k in 0..50 {
            for i in 0..(GRID_SIZE) {
                for j in 0..(GRID_SIZE) {
                    for gas in 0..GAS_COUNT {
                        diffuse_moles(i, j, &x0, &mut x, &grid.wall_mask, a, gas);
                    }
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let (mut gas, mut sprite) = tile_query.get_mut(grid.grid[i][j]).unwrap();
                gas.amount = x[i][j].amount;
                gas.temperature = x[i][j].temperature;
                if !grid.wall_mask[i][j] {
                    match *visualization {
                        GasVisualizationSettings::None => {
                            sprite.color = Color::NONE;
                        }
                        GasVisualizationSettings::Pressure => {
                            sprite.color = Color::rgba(
                                (gas.get_pressure(Gas::Oxygen) as f32 / 1.5).clamp(0.0, 1.0),
                                (gas.get_pressure(Gas::Nitrogen) as f32 / 1.5).clamp(0.0, 1.0),
                                (gas.get_pressure(Gas::CarbonDioxide) as f32 / 1.0).clamp(0.0, 1.0),
                                0.25,
                            );
                        }
                        GasVisualizationSettings::Moles => {
                            sprite.color = Color::rgba(
                                ((gas.amount[Gas::Oxygen as usize] - 75.0) as f32 / 15.0)
                                    .clamp(0.0, 1.0),
                                (gas.amount[Gas::Nitrogen as usize] as f32 / 100.0).clamp(0.0, 1.0),
                                (gas.amount[Gas::CarbonDioxide as usize] as f32 / 100.0)
                                    .clamp(0.0, 1.0),
                                0.25,
                            );
                        }
                        GasVisualizationSettings::Temperature => {
                            sprite.color = Color::rgba(
                                ((gas.temperature - 250.0) as f32 / 250.0).clamp(0.0, 1.0),
                                0.0,
                                0.0,
                                0.25,
                            );
                        }
                    }
                } else {
                    sprite.color = Color::rgba(0.1, 0.1, 0.1, 0.0);
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
            let gas = GasMixture {
                //https://www.discovermagazine.com/the-sciences/how-cold-is-it-in-outer-space
                temperature: 2.7,
                ..Default::default()
            };
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
        .insert(GasVisualizationSettings::None)
        .insert(Name::new("Gas Grid"));
}
