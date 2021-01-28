use amethyst::{
    assets::Handle,
    core::{
        math::{Point3, Vector2, Vector3},
        SystemDesc, Transform,
    },
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Entities, Entity, Join, NullStorage, Read, ReadExpect,
        ReadStorage, System, SystemData, World, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::hex_grid::HexCoord;

#[derive(Default)]
pub struct Selection;

impl Component for Selection {
    type Storage = NullStorage<Self>;
}

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
        TileSelectSystem {
            mouse_was_down: false,
        }
    }
}

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Selection>,
        WriteStorage<'s, Selectable>,
        ReadStorage<'s, HexCoord>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut selections,
            mut selectables,
            hex_coords,
            mut transforms,
            cameras,
            input,
            dimensions,
            mut sprite_renders,
            sprite_sheet_handle,
        ): Self::SystemData,
    ) {
        let mouse_is_down = input.mouse_button_is_down(MouseButton::Left);

        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());
        let mut selection_transform = None;
        for (camera, transform) in (&cameras, &transforms).join() {
            if let Some((screen_x, screen_y)) = input.mouse_position() {
                let world_point = camera.screen_to_world_point(
                    Point3::new(screen_x, screen_y, 0.0),
                    screen_dimensions,
                    transform,
                );
                let hex: HexCoord = world_point.into();
                if self.mouse_was_down && !mouse_is_down {
                    for (entity, _selection) in (&*entities, &selections).join() {
                        entities.delete(entity);
                    }
                    for (entity, selectable, hex_coord, transform) in
                        (&*entities, &mut selectables, &hex_coords, &transforms).join()
                    {
                        selectable.selected = *hex_coord == hex;
                        if selectable.selected {
                            selection_transform = Some(transform.clone());
                        }
                    }
                }
            }
        }

        if let Some(transform) = selection_transform {
            let highlight_sprite = SpriteRender::new(sprite_sheet_handle.clone(), 2);
            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(Selection, &mut selections)
                .with(highlight_sprite, &mut sprite_renders)
                .build();
        }

        self.mouse_was_down = mouse_is_down;
    }
}
