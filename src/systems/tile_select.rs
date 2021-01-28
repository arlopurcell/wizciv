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
    left_mouse_was_down: bool,
    right_mouse_was_down: bool,
}

impl TileSelectSystem {
    pub fn new() -> Self {
        TileSelectSystem {
            left_mouse_was_down: false,
            right_mouse_was_down: false,
        }
    }
}

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Selection>,
        WriteStorage<'s, Selectable>,
        WriteStorage<'s, HexCoord>,
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
            mut hex_coords,
            mut transforms,
            cameras,
            input,
            dimensions,
            mut sprite_renders,
            sprite_sheet_handle,
        ): Self::SystemData,
    ) {
        let left_mouse_is_down = input.mouse_button_is_down(MouseButton::Left);
        let right_mouse_is_down = input.mouse_button_is_down(MouseButton::Right);

        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());
        let mut selection_transform = None;

        let mouse_hex: Option<HexCoord> = (&cameras, &transforms).join().next().and_then(|(c, t)| input.mouse_position().map(|mp| (c, t, mp))).map(|(camera, transform, (screen_x, screen_y))| camera.screen_to_world_point(Point3::new(screen_x, screen_y, 0.0), screen_dimensions, transform).into());

        if let Some(mouse_hex) = mouse_hex {
            if self.left_mouse_was_down && !left_mouse_is_down {
                for (entity, _selection) in (&*entities, &selections).join() {
                    entities.delete(entity);
                }
                for (entity, selectable, hex_coord, transform) in
                    (&*entities, &mut selectables, &hex_coords, &transforms).join()
                    {
                        selectable.selected = *hex_coord == mouse_hex;
                        if selectable.selected {
                            selection_transform = Some(transform.clone());
                        }
                    }
            }

            if self.right_mouse_was_down && !right_mouse_is_down {
                let mut moved = false;
                let (mouse_x, mouse_y) = mouse_hex.world_coords();
                //let mouse_vector: Vector2<f32> = mouse_hex.into();
                for (entity, selectable, mut hex_coord, mut transform) in
                    (&*entities, &mut selectables, &mut hex_coords, &mut transforms).join()
                    {
                        if selectable.selected {
                            if mouse_hex.is_adjacent(hex_coord) {
                                transform.set_translation_x(mouse_x).set_translation_y(mouse_y);
                                *hex_coord = mouse_hex;
                                moved = true;
                            }
                        }
                    }
                if moved {
                    for (entity, _selection) in (&*entities, &selections).join() {
                        entities.delete(entity);
                    }
                }
            }
        }

        /*
        for (camera, transform) in (&cameras, &transforms).join() {
            if let Some((screen_x, screen_y)) = input.mouse_position() {
                let world_point = camera.screen_to_world_point(
                    Point3::new(screen_x, screen_y, 0.0),
                    screen_dimensions,
                    transform,
                );
                let mouse_hex: HexCoord = world_point.into();
            }
        }
        */

        if let Some(transform) = selection_transform {
            let highlight_sprite = SpriteRender::new(sprite_sheet_handle.clone(), 2);
            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(Selection, &mut selections)
                .with(highlight_sprite, &mut sprite_renders)
                .build();
        }

        self.left_mouse_was_down = left_mouse_is_down;
        self.right_mouse_was_down = right_mouse_is_down;
    }
}
