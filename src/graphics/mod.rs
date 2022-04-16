use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

use self::assets::SpriteSheet;

mod ascii;
mod assets;
mod pixel_perfect_selection;

pub use ascii::spawn_ascii_sprite;

#[derive(Inspectable, Deserialize, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(
    Inspectable, Component, Deserialize, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy,
)]
pub enum Graphic {
    Player(Orientation),
    WorldObject(WorldObject),
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Down
    }
}

#[derive(
    Inspectable, Deserialize, Component, Serialize, Hash, Debug, PartialEq, Eq, Clone, Copy,
)]
pub enum WorldObject {
    Canister,
    CanisterMachine,
    SmallLabel(usize),
    Terminal(Orientation),
    Wall(WallConnection),
}

impl Default for WorldObject {
    fn default() -> Self {
        WorldObject::Canister
    }
}

#[derive(Default)]
pub struct AsciiSheet {
    handle: Handle<TextureAtlas>,
    pub tile_size: f32,
}

#[derive(Component)]
pub struct AsciiText;

#[derive(Copy, Clone)]
pub struct NineSliceIndices {
    center: usize,
    upper_left_index: usize,
    upper_right_index: usize,
    lower_left_index: usize,
    lower_right_index: usize,
    horizontal_index: usize,
    vertical_index: usize,
}

//Entry on the ron sheet description
#[derive(Clone, Copy, Debug, Reflect, Deserialize)]
pub struct SpriteDesc {
    pub sheet: SpriteSheet,
    pub min: Vec2,
    pub max: Vec2,
    #[serde(default)]
    flip_x: bool,
    #[serde(default)]
    flip_y: bool,
}

#[derive(Component, Default)]
pub struct Clickable {
    just_clicked: bool,
}

#[derive(Component)]
pub struct PixelPerfectHitBox {
    width: usize,
    height: usize,
    mask: Vec<Vec<bool>>,
}

struct AsciiPlugin {
    pub tile_size: f32,
}
struct GameAssetsPlugin;
struct PixelPerfectPlugin;

pub struct GraphicsPluginGroup;

impl PluginGroup for GraphicsPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(AsciiPlugin { tile_size: 32.0 })
            .add(GameAssetsPlugin)
            .add(PixelPerfectPlugin);
    }
}

//Internal Resource holding all handles and indices
struct Graphics {
    handle_map: HashMap<SpriteSheet, Handle<TextureAtlas>>,
    graphics_map: HashMap<Graphic, (SpriteDesc, usize)>,
}
