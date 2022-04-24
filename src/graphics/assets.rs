use crate::prelude::*;

use bevy_inspector_egui::RegisterInspectable;
//use bevy_loading::prelude::AssetsLoading;
use std::fs;

use ron::de::from_str;

use super::{GameAssetsPlugin, Graphics};

//All sheets in the assets
#[derive(Clone, Copy, Debug, Reflect, Deserialize, PartialEq, Eq, Hash)]
pub enum SpriteSheet {
    Character,
    StarterGraphics,
}

//Format to be loaded from ron
#[derive(Deserialize)]
struct GraphicsDesc {
    sheet_filename_map: HashMap<SpriteSheet, String>,
    graphics_map: HashMap<Graphic, SpriteDesc>,
}

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            Self::load_graphics.label("graphics"),
        )
        .register_inspectable::<Graphic>()
        .add_system_to_stage(CoreStage::PreUpdate, spawn_sprite)
        .add_system(update_sprite);
        //.add_system(Self::set_img_sampler_filter);
    }
}

//XXX Does not work if changed to graphic on another sheet
fn update_sprite(
    mut update_query: Query<(&mut TextureAtlasSprite, &Graphic), Changed<Graphic>>,
    graphics: Res<Graphics>,
) {
    for (mut sprite, graphic) in update_query.iter_mut() {
        if let Some((_, index)) = graphics.graphics_map.get(graphic) {
            sprite.index = *index;
        } else {
            error!(
                "Failed to load sprite for {:?}, missing in graphics_desc.ron?",
                graphic
            );
        }
    }
}

fn spawn_sprite(
    mut commands: Commands,
    res: Res<Graphics>,
    graphics_to_spawn: Query<(Entity, &Graphic, Option<&Transform>), Without<TextureAtlasSprite>>,
) {
    for (ent, to_spawn, transform) in graphics_to_spawn.iter() {
        if let Some((desc, index)) = res.graphics_map.get(to_spawn) {
            let mut sprite = TextureAtlasSprite::new(*index);
            sprite.flip_x = desc.flip_x;
            sprite.flip_y = desc.flip_y;
            let atlas = &res.handle_map[&desc.sheet];
            commands.entity(ent).insert_bundle(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: atlas.clone(),
                transform: *transform.unwrap_or(&Transform::default()),
                ..Default::default()
            });
        } else {
            error!(
                "Failed to load sprite for {:?}, missing in graphics_desc.ron?",
                to_spawn
            );
            commands
                .entity(ent)
                .insert(*transform.unwrap_or(&Transform::default()))
                .insert(GlobalTransform::default());
        }
    }
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
        //mut loading: ResMut<AssetsLoading>,
    ) {
        let sprite_desc = fs::read_to_string("assets/graphics_desc.ron").unwrap();

        let sprite_desc: GraphicsDesc = match from_str(&sprite_desc) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };

        //Load all images and create atlases
        let mut atlas_map = HashMap::default();
        for (sheet, file_name) in sprite_desc.sheet_filename_map.iter() {
            let image_handle = assets.load(file_name);
            //loading.add(&image_handle);
            //FIXME image size should either come from loaded image or from desc ron
            let atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));
            atlas_map.insert(*sheet, atlas);
        }

        //Add all sprites to their atlases and save the index
        let mut graphics_map = HashMap::default();
        for (item, desc) in sprite_desc.graphics_map.iter() {
            let atlas = atlas_map.get_mut(&desc.sheet).unwrap();
            let index = atlas.add_texture(bevy::sprite::Rect {
                min: desc.min,
                max: desc.max,
            });
            graphics_map.insert(*item, (*desc, index));
        }

        //Save the handles
        let mut handle_map = HashMap::default();
        for (sheet, atlas) in atlas_map {
            let atlas_handle = texture_assets.add(atlas);
            handle_map.insert(sheet, atlas_handle);
        }

        commands.insert_resource(Graphics {
            handle_map: handle_map,
            graphics_map: graphics_map,
        });
    }
}
