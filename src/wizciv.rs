use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector2,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Read, System, SystemData, World},
    input::{ControllerButton, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::hex_grid::HexCoord;
use crate::systems::tile_select::Selectable;

pub struct WizCiv;

impl SimpleState for WizCiv {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        world.register::<Unit>();
        world.insert(MouseState::default());

        initialise_tiles(world, sprite_sheet_handle.clone());
        initialise_units(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(Default)]
pub struct MouseState {
    pub hex: HexCoord,
    pub left_state: ButtonState,
    pub right_state: ButtonState,
}

#[derive(Default)]
pub struct ButtonState {
    pub is_down: bool,
    pub was_down: bool,
}

impl ButtonState {
    pub fn is_clicked(&self) -> bool {
        !self.is_down && self.was_down
    }
}

pub const X_TILE_NUM: i16 = 3;
pub const Y_TILE_NUM: i16 = 3;

pub const HEX_SIZE: f32 = 90.0;
pub const HEX_HEIGHT: f32 = 103.923;

pub const WORLD_WIDTH: f32 = X_TILE_NUM as f32 * HEX_SIZE;
pub const WORLD_HEIGHT: f32 = Y_TILE_NUM as f32 * HEX_SIZE;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(WORLD_WIDTH * 0.5, WORLD_HEIGHT * 0.5, 100.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WORLD_WIDTH, WORLD_HEIGHT))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum TileType {
    Grass,
}

pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    fn grass() -> Self {
        Self::new(TileType::Grass)
    }

    fn new(tile_type: TileType) -> Self {
        Tile { tile_type }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_tiles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let tile_sprite = SpriteRender::new(sprite_sheet_handle, 1);
    for i in -X_TILE_NUM..X_TILE_NUM {
        for j in -Y_TILE_NUM..Y_TILE_NUM {
            let hex = HexCoord::new(i, j);
            let mut transform = Transform::default();
            let pixel: Vector2<f32> = hex.into();
            //let (i, j) = (i as i16, j as i16);
            //let (x, y) = indexes_to_coordinates(i, j);
            //transform.set_translation_xyz(x, y, 0.0);
            transform
                .set_translation_x(pixel.x)
                .set_translation_y(pixel.y);
            world
                .create_entity()
                .with(Tile::grass())
                .with(HexCoord::new(i, j))
                .with(transform)
                .with(tile_sprite.clone())
                .build();
        }
    }
}

pub struct Unit {}

impl Unit {
    fn new() -> Self {
        Unit {}
    }
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_units(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mage_sprite = SpriteRender::new(sprite_sheet_handle, 0);

    let hex = HexCoord::new(0, 0);
    let pixel: Vector2<f32> = hex.into();
    let mut transform = Transform::default();
    transform
        .set_translation_x(pixel.x)
        .set_translation_y(pixel.y)
        .set_translation_z(5.0);
    world
        .create_entity()
        .with(Unit::new())
        .with(Selectable::default())
        .with(HexCoord::new(0, 0))
        .with(transform)
        .with(mage_sprite.clone())
        .build();

    let hex = HexCoord::new(1, 2);
    let pixel: Vector2<f32> = hex.into();
    let mut transform = Transform::default();
    transform
        .set_translation_x(pixel.x)
        .set_translation_y(pixel.y)
        .set_translation_z(5.0);
    world
        .create_entity()
        .with(Unit::new())
        .with(Selectable::default())
        .with(HexCoord::new(1, 2))
        .with(transform)
        .with(mage_sprite)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };
    world.insert(sheet_handle.clone());
    sheet_handle
}
