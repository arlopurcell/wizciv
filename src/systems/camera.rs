use amethyst::{
    core::{Transform, SystemDesc, math::Vector3},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

const MOVE_SPEED: f32 = 10.0;
const ZOOM_SPEED: f32 = 0.2;
const ZOOM_MIN: f32 = 0.1;
const ZOOM_MAX: f32 = 3.0;


#[derive(SystemDesc)]
pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, input): Self::SystemData) {
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("camera_horizontal") {
                if mv_amount != 0.0 {
                    transform.prepend_translation_x(mv_amount as f32 * MOVE_SPEED);
                }
            }
            if let Some(mv_amount) = input.axis_value("camera_vertical") {
                if mv_amount != 0.0 {
                    transform.prepend_translation_y(mv_amount as f32 * MOVE_SPEED);
                }
            }
            if let Some(mv_amount) = input.axis_value("camera_zoom") {
                if mv_amount != 0.0 {
                    let z_scale = ZOOM_SPEED * mv_amount;
                    let scale = transform.scale();
                    let scale = Vector3::new(
                        (scale.x + z_scale).max(ZOOM_MIN).min(ZOOM_MAX),
                        (scale.y + z_scale).max(ZOOM_MIN).min(ZOOM_MAX),
                        (scale.z + z_scale).max(ZOOM_MIN).min(ZOOM_MAX),
                    );
                    transform.set_scale(scale);
                }
            }
        }
    }
}

