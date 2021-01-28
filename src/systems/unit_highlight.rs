use amethyst::{
    core::SystemDesc,
    assets::Handle,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entity, Entities},
    renderer::{SpriteSheet, SpriteRender},
};

use crate::systems::tile_select::Selectable;

#[derive(SystemDesc)]
pub struct UnitHighlightSystem;

impl<'s> System<'s> for UnitHighlightSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Selectable>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(&mut self, (entities, selectables, mut sprite_renders, sheet_handle): Self::SystemData) {
        for (entity, selectable) in (&*entities, &selectables).join() {
            //let highlight_sprite = SpriteRender::new(sheet_handle.clone(), 2);
            //sprite_renders.insert(entity, highlight_sprite);
        }
    }
}
