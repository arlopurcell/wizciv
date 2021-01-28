use std::ops::Deref;
use amethyst::{
    core::{Transform, SystemDesc, math::{Vector3, Vector2, Point3}},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage, Entity, Entities, Component, NullStorage, DenseVecStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::hex_grid::HexCoord;

/*
#[derive(Default)]
pub struct Selected;

impl Component for Selected {
    type Storage = NullStorage<Self>;
}
*/

#[derive(Default)]
pub struct Selectable {
    selected: bool,
}

impl Component for Selectable {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct TileSelectSystem {
    mouse_was_down: bool,
}

impl TileSelectSystem {
    pub fn new() -> Self {
        TileSelectSystem { mouse_was_down: false }
    }
}

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        //WriteStorage<'s, Selected>,
        WriteStorage<'s, Selectable>,
        ReadStorage<'s, HexCoord>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (entities, mut selectables, hex_coords, transforms, cameras, input, dimensions): Self::SystemData) {
        let mouse_is_down = input.mouse_button_is_down(MouseButton::Left);

        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());
        for (camera, transform) in (&cameras, &transforms).join() {
            if let Some((screen_x, screen_y)) = input.mouse_position() {
                let world_point = camera.screen_to_world_point(Point3::new(screen_x, screen_y, 0.0), screen_dimensions, transform);
                let hex: HexCoord = world_point.into();
                if self.mouse_was_down && !mouse_is_down {
                    for (entity, selectable, hex_coord) in (&*entities, &mut selectables, &hex_coords).join() {
                        selectable.selected = *hex_coord == hex;
                        if selectable.selected {
                            println!("Selected {:?}", entity);
                        }
                    }
                }
            }
        }

        self.mouse_was_down = mouse_is_down;
    }
}

