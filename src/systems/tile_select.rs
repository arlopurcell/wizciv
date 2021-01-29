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
use crate::wizciv::MouseState;

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
        Read<'s, MouseState>,
        WriteStorage<'s, Selection>,
        WriteStorage<'s, Selectable>,
        WriteStorage<'s, HexCoord>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut selections,
            mut selectables,
            mut hex_coords,
            mut transforms,
            mut sprite_renders,
            sprite_sheet_handle,
        ): Self::SystemData,
    ) {
        let mut selection_transform = None;
        if mouse_state.left_state.is_clicked() {
            // TODO hide and show selection with z position
            for (entity, _selection) in (&*entities, &selections).join() {
                entities.delete(entity);
            }
            // TODO remove hex_coords component and just calculate from transform
            for (entity, selectable, hex_coord, transform) in
                (&*entities, &mut selectables, &hex_coords, &transforms).join()
                {
                    selectable.selected = *hex_coord == mouse_state.hex;
                    if selectable.selected {
                        selection_transform = Some(transform.clone());
                    }
                }
        }

        if mouse_state.right_state.is_clicked() {
            let mut moved = false;
            let (mouse_x, mouse_y) = mouse_state.hex.world_coords();
            // TODO remove hex_coords component and just calculate from transform
            for (entity, selectable, mut hex_coord, mut transform) in
                (&*entities, &mut selectables, &mut hex_coords, &mut transforms).join()
                {
                    if selectable.selected {
                        if mouse_state.hex.is_adjacent(hex_coord) {
                            transform.set_translation_x(mouse_x).set_translation_y(mouse_y);
                            *hex_coord = mouse_state.hex;
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

        if let Some(transform) = selection_transform {
            let highlight_sprite = SpriteRender::new(sprite_sheet_handle.clone(), 2);
            entities
                .build_entity()
                .with(transform, &mut transforms)
                .with(Selection, &mut selections)
                .with(highlight_sprite, &mut sprite_renders)
                .build();
        }
    }
}
