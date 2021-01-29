use amethyst::{
    core::{math::Vector3, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use crate::wizciv::WorldData;

const MOVE_SPEED: f32 = 15.0;
const ZOOM_SPEED: f32 = 0.5;
const ZOOM_MIN: f32 = 1.0;
const ZOOM_MAX: f32 = 3.0;

#[derive(SystemDesc)]
pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, WorldData>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, world_data, input): Self::SystemData) {
        let max_position = world_data.world_pixel_size() / 2.;
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let zoom = transform.scale().x;
            if let Some(mv_amount) = input.axis_value("camera_horizontal") {
                if mv_amount != 0.0 {
                    let new_x = transform.translation().x + (mv_amount as f32 * MOVE_SPEED * zoom);
                    if new_x > -max_position && new_x < max_position {
                        transform.set_translation_x(new_x);
                    }
                }
            }
            if let Some(mv_amount) = input.axis_value("camera_vertical") {
                if mv_amount != 0.0 {
                    let new_y = transform.translation().y + (mv_amount as f32 * MOVE_SPEED * zoom);
                    if new_y > -max_position && new_y < max_position {
                        transform.set_translation_y(new_y);
                    }
                }
            }
            if let Some(mv_amount) = input.axis_value("camera_zoom") {
                if mv_amount != 0.0 {
                    let zoom_delta = ZOOM_SPEED * mv_amount;
                    let zoom_scale = (zoom + zoom_delta).max(ZOOM_MIN).min(ZOOM_MAX);
                    transform.set_scale(Vector3::new(zoom_scale, zoom_scale, zoom_scale));
                }
            }
        }
    }
}
