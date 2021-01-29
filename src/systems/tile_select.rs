use amethyst::{
    core::{Hidden, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::hex_grid::HexCoord;
use crate::wizciv::{MouseState, Selectable, Selection, WorldData};

#[derive(SystemDesc)]
pub struct TileSelectSystem;

impl<'s> System<'s> for TileSelectSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, MouseState>,
        Read<'s, WorldData>,
        ReadStorage<'s, Selection>,
        WriteStorage<'s, Selectable>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            world_data,
            selections,
            mut selectables,
            mut transforms,
            mut hiddens,
        ): Self::SystemData,
    ) {
        let mut new_selection_transform = None;
        let mut hide_selection = None;
        if mouse_state.left_state.is_clicked() {
            hide_selection = Some(true);
            if let Some(mouse_hex) = mouse_state.hex {
                // Find the selectable they clicked on
                for (_entity, selectable, transform) in
                    (&*entities, &mut selectables, &transforms).join()
                {
                    let selectable_hex = HexCoord::from_xy(transform.translation().x, transform.translation().y, world_data.radius).expect("Selectable should be within the world bounds");
                    selectable.selected = selectable_hex == mouse_hex;
                    if selectable.selected {
                        new_selection_transform = Some(transform.clone());
                        hide_selection = Some(false);
                    }
                }
            }
        }
        if let Some(transform) = new_selection_transform {
            let selection_transform = (&selections, &mut transforms)
                .join()
                .next()
                .expect("There should be a selection component with a transform")
                .1;
            *selection_transform = transform;
        }

        if mouse_state.right_state.is_clicked() {
            if let Some(mouse_hex) = mouse_state.hex {
                let (mouse_x, mouse_y) = mouse_hex.world_coords();
                for (_entity, selectable, transform) in
                    (&*entities, &mut selectables, &mut transforms).join()
                    {
                        let selectable_hex = HexCoord::from_xy(transform.translation().x, transform.translation().y, world_data.radius).expect("Selectable should be within the world bounds");
                        if selectable.selected {
                            if mouse_hex.is_adjacent(&selectable_hex) {
                                transform
                                    .set_translation_x(mouse_x)
                                    .set_translation_y(mouse_y);
                                selectable.selected = false;
                                //moved = true;
                                hide_selection = Some(true);
                            }
                        }
                    }
            }
        }

        if let Some(hide_selection) = hide_selection {
            let selection_entity = (&*entities, &selections)
                .join()
                .next()
                .expect("There should be a selection entity")
                .0;
            if hide_selection {
                hiddens
                    .insert(selection_entity, Hidden)
                    .expect("Hiding stuff shouldn't fail");
            } else {
                hiddens.remove(selection_entity);
            }
        }
    }
}
