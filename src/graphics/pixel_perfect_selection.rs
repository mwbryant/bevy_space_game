use bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

use super::Graphics;
use super::{Graphic, PixelPerfectPlugin};
use crate::mouse::MousePosition;
use crate::{prelude::*, AppState};

#[derive(Default)]
struct HitboxCache {
    map: HashMap<Graphic, PixelPerfectHitBox>,
}

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_hitbox_cache)
            .insert_resource(HitboxCache::default())
            .add_system(test_hitbox);
    }
}

//TODO handle rotations and x/y flip
fn test_hitbox(
    mut query: Query<(&GlobalTransform, &Graphic, &Name, &mut Clickable)>,
    hitboxes: Res<HitboxCache>,
    mouse: Res<MousePosition>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let mut over_anything = false;
        for (transform, graphic, name, mut click) in query.iter_mut() {
            if let Some(hit_box) = hitboxes.map.get(graphic) {
                //x and y are centered
                let x_offset = transform.translation.x - hit_box.width as f32 / 2.0;
                let y_offset = transform.translation.y - hit_box.height as f32 / 2.0;

                let rel_x = (mouse.0.x - x_offset) as isize;
                let rel_y = (mouse.0.y - y_offset) as isize;

                if rel_x >= 0
                    && rel_x < hit_box.width as isize
                    && rel_y >= 0
                    && rel_y < hit_box.height as isize
                {
                    //invert y
                    let rel_y = hit_box.height as isize - rel_y - 1;
                    if hit_box.mask[rel_x as usize][rel_y as usize] {
                        over_anything = true;
                        click.just_clicked = true;
                        println!("Over {}!", name.as_str());
                    }
                }
            }
        }
        if !over_anything {
            println!("Over Nothing");
        }
    }
}

fn create_hitbox_cache(
    mut commands: Commands,
    graphics: Res<Graphics>,
    server: Res<AssetServer>,
    atlas_assets: Res<Assets<TextureAtlas>>,
    image_assets: Res<Assets<Image>>,
    mut cache: ResMut<HitboxCache>,
    // https://bevy-cheatbook.github.io/assets/assetevent.html
    mut ev_asset: EventReader<AssetEvent<Image>>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Modified { handle: texture } = ev {
            for (graphic, (desc, _)) in graphics.graphics_map.iter() {
                let atlas_handle = graphics.handle_map[&desc.sheet].clone();
                let desc_texture = atlas_assets.get(atlas_handle).unwrap().texture.clone();

                if desc_texture == *texture {
                    if server.get_load_state(texture.clone()) == LoadState::Loaded {
                        let image = image_assets.get(texture).unwrap();
                        add_graphic_to_hitboxes(&mut cache, graphic, desc.min, desc.max, image);
                    } else {
                        println!("Not Loaded");
                    }
                }
            }
        }
    }
}

//TODO support fliped images
fn add_graphic_to_hitboxes(
    cache: &mut HitboxCache,
    graphic: &Graphic,
    min: Vec2,
    max: Vec2,
    image: &Image,
) {
    assert!(Rgba8UnormSrgb == image.texture_descriptor.format);

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
