use bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

use crate::mouse::MousePosition;
use crate::{
    assets::{Graphic, Graphics},
    AppState,
};

#[derive(Component)]
pub struct PixelPerfectHitBox {
    width: usize,
    height: usize,
    mask: Vec<Vec<bool>>,
}

#[derive(Default)]
pub struct HitboxCache {
    map: HashMap<Graphic, PixelPerfectHitBox>,
}

pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(create_hitbox_cache))
            .insert_resource(HitboxCache::default())
            .add_system(test_hitbox);
    }
}

fn test_hitbox(
    query: Query<(&Transform, &Graphic, &Name)>,
    hitboxes: Res<HitboxCache>,
    mouse: Res<MousePosition>,
) {
    for (transform, graphic, name) in query.iter() {
        if let Some(graphic) = hitboxes.map.get(graphic) {
            //x and y are centered
            let x_offset = transform.translation.x - graphic.width as f32 / 2.0;
            let y_offset = transform.translation.y - graphic.height as f32 / 2.0;

            let rel_x = (mouse.0.x - x_offset) as isize;
            let rel_y = (mouse.0.y - y_offset) as isize;

            if rel_x >= 0
                && rel_x < graphic.width as isize
                && rel_y >= 0
                && rel_y < graphic.height as isize
            {
                //invert y
                let rel_y = graphic.height as isize - rel_y - 1;
                if check_collision(
                    rel_x as usize,
                    rel_y as usize,
                    graphic.height,
                    &graphic.mask,
                ) {
                    println!("Over {}!", name.as_str());
                }
            }
        }
    }
}

fn check_collision(rel_x: usize, rel_y: usize, width: usize, mask: &[Vec<bool>]) -> bool {
    for (index, pixel) in mask.iter().flatten().enumerate() {
        let x = index / width;
        let y = index % width;
        if *pixel && x == rel_x && y == rel_y {
            return true;
        }
    }
    false
}

fn create_hitbox_cache(
    mut commands: Commands,
    graphics: Res<Graphics>,
    server: Res<AssetServer>,
    atlas_assets: Res<Assets<TextureAtlas>>,
    image_assets: Res<Assets<Image>>,
) {
    let mut cache = HitboxCache::default();
    for (graphic, (desc, _)) in graphics.graphics_map.iter() {
        let atlas_handle = graphics.handle_map[&desc.sheet].clone();
        let texture = atlas_assets.get(atlas_handle).unwrap().texture.clone();

        if server.get_load_state(texture.clone()) == LoadState::Loaded {
            let image = image_assets.get(texture).unwrap();
            assert!(Rgba8UnormSrgb == image.texture_descriptor.format);
            add_graphic_to_hitboxes(&mut cache, graphic, desc.min, desc.max, image);
        } else {
            println!("Not Loaded");
        }
    }
    commands.insert_resource(cache);
}

//TODO support fliped images
pub fn add_graphic_to_hitboxes(
    cache: &mut HitboxCache,
    graphic: &Graphic,
    min: Vec2,
    max: Vec2,
    image: &Image,
) {
    let width = max.x as usize - min.x as usize;
    let height = max.y as usize - min.y as usize;
    let image_width = image.texture_descriptor.size.width as usize;
    let mut mask = vec![vec![false; height]; width];

    for (index, data) in image.data.iter().enumerate() {
        //Alpha every 4?
        if index % 4 == 3 {
            let pixel_index = index / 4;
            //is in our graphic bounds?
            let x = (pixel_index % image_width) as f32;
            let y = (pixel_index / image_width) as f32;
            //FIXME expects min and max to be correct
            if x >= min.x && x < max.x && y >= min.y && y < max.y {
                //Check if alpha is low
                if data > &10 {
                    mask[(x - min.x) as usize][(y - min.y) as usize] = true;
                }
            }
        }
    }

    cache.map.insert(
        *graphic,
        PixelPerfectHitBox {
            width,
            height,
            mask,
        },
    );
}
