use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Hidden, Transform},
    ecs::{Component, DenseVecStorage, NullStorage, World},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::hex_grid::{HexCoord, HEX_SIZE};

pub struct WizCiv;

impl SimpleState for WizCiv {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        world.register::<Unit>();
        world.insert(MouseState::default());
        world.insert(WorldData{ radius: WORLD_HEX_RADIUS});

        initialise_selection(world, sprite_sheet_handle.clone());
        initialise_tiles(world, sprite_sheet_handle.clone(), WORLD_HEX_RADIUS);
        initialise_units(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(Default)]
pub struct WorldData {
    pub radius: i16,
}

impl WorldData {
    pub fn world_pixel_size(&self) -> f32 {
        (self.radius * 3 + 1) as f32 * HEX_SIZE
    }
}

#[derive(Default)]
pub struct MouseState {
    pub hex: Option<HexCoord>,
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

#[derive(Default)]
pub struct Selection;

impl Component for Selection {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Selectable {
    pub selected: bool,
}

impl Component for Selectable {
    type Storage = DenseVecStorage<Self>;
}

pub const WORLD_HEX_RADIUS: i16 = 30;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_z(100.);
    //transform.set_scale(Vector3::new(2., 2., 2.));

    world
        .create_entity()
        .with(Camera::standard_2d(HEX_SIZE * 10., HEX_SIZE * 10.))
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

fn initialise_tiles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, world_radius: i16) {
    let tile_sprite = SpriteRender::new(sprite_sheet_handle, 1);
    for i in -world_radius..(world_radius + 1) {
        for j in -world_radius..(world_radius + 1) {
            if let Some(hex) = HexCoord::new(i, j, world_radius) {
                let mut transform = Transform::default();
                let (x, y) = hex.world_coords();
                transform
                    .set_translation_x(x)
                    .set_translation_y(y);
                world
                    .create_entity()
                    .with(Tile::grass())
                    .with(transform)
                    .with(tile_sprite.clone())
                    .build();
            }
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

fn initialise_selection(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let highlight_sprite = SpriteRender::new(sprite_sheet_handle, 2);
    world
        .create_entity()
        .with(Transform::default())
        .with(Selection)
        .with(Hidden)
        .with(highlight_sprite)
        .build();
}

fn initialise_units(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mage_sprite = SpriteRender::new(sprite_sheet_handle, 0);

    let hex = HexCoord::new(0, 0, WORLD_HEX_RADIUS).unwrap();
    let (x,y) = hex.world_coords();
    let mut transform = Transform::default();
    transform
        .set_translation_x(x)
        .set_translation_y(y)
        .set_translation_z(5.0);
    world
        .create_entity()
        .with(Unit::new())
        .with(Selectable::default())
        .with(transform)
        .with(mage_sprite.clone())
        .build();

    let hex = HexCoord::new(1, 2, WORLD_HEX_RADIUS).unwrap();
    let (x,y) = hex.world_coords();
    let mut transform = Transform::default();
    transform
        .set_translation_x(x)
        .set_translation_y(y)
        .set_translation_z(5.0);
    world
        .create_entity()
        .with(Unit::new())
        .with(Selectable::default())
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
