use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Read, System, SystemData, World},
    prelude::*,
    input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
    core::SystemDesc,
    derive::SystemDesc,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct WizCiv;

impl SimpleState for WizCiv {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        world.register::<Unit>();

        initialise_tiles(world, sprite_sheet_handle.clone());
        initialise_units(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub const X_TILE_NUM: usize = 10;
pub const Y_TILE_NUM: usize = 10;

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
    pub x: u16,
    pub y: u16,
}

impl Tile {
    fn grass(x: u16, y: u16) -> Self {
        Self::new(TileType::Grass, x, y)
    }

    fn new(tile_type: TileType, x: u16, y: u16) -> Self {
        Tile { tile_type, x, y }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_tiles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let tile_sprite = SpriteRender::new(sprite_sheet_handle, 1);
    for i in 0..X_TILE_NUM {
        for j in 0..Y_TILE_NUM {
            let (i, j) = (i as u16, j as u16);
            let mut transform = Transform::default();
            let (x, y) = indexes_to_coordinates(i, j);
            transform.set_translation_xyz(x, y, 0.0);
            world
                .create_entity()
                .with(Tile::grass(i, j))
                .with(transform)
                .with(tile_sprite.clone())
                .build();
        }
    }
}

pub struct Unit {
    pub x: u16,
    pub y: u16,
}

impl Unit {
    fn new(x: u16, y: u16) -> Self {
        Unit { x, y }
    }
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_units(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mage_sprite = SpriteRender::new(sprite_sheet_handle, 0);

    let (i, j) = (5, 5);
    let (x, y) = indexes_to_coordinates(i, j);
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 5.0);
    world
        .create_entity()
        .with(Unit::new(i, j))
        .with(transform)
        .with(mage_sprite.clone())
        .build();

    let (i, j) = (7, 2);
    let (x, y) = indexes_to_coordinates(i, j);
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 5.0);
    world
        .create_entity()
        .with(Unit::new(i, j))
        .with(transform)
        .with(mage_sprite)
        .build();
}

fn indexes_to_coordinates(i: u16, j: u16) -> (f32, f32) {
    let x = i as f32 * HEX_SIZE;
    let y = j as f32 * HEX_HEIGHT - ((i & 1) as f32 * (HEX_HEIGHT / 2.0));
    (x, y)
}

fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
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

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

