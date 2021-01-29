use amethyst::{
    core::{
        math::{Point3, Vector2},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::hex_grid::HexCoord;
use crate::wizciv::{MouseState, WorldData};

#[derive(SystemDesc, Default)]
pub struct MouseHexSystem;

impl<'s> System<'s> for MouseHexSystem {
    type SystemData = (
        Write<'s, MouseState>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, WorldData>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut mouse_state, transforms, cameras, world_data, input, dimensions): Self::SystemData) {
        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());

        let mouse_hex: Option<HexCoord> = (&cameras, &transforms)
            .join()
            .next()
            .and_then(|(c, t)| input.mouse_position().map(|mp| (c, t, mp)))
            .and_then(|(camera, transform, (screen_x, screen_y))| {
                let world_point = camera
                    .screen_to_world_point(
                        Point3::new(screen_x, screen_y, 0.0),
                        screen_dimensions,
                        transform,
                    );
                HexCoord::from_xy(world_point.x, world_point.y, world_data.radius)
            });

        mouse_state.hex = mouse_hex;

        mouse_state.left_state.was_down = mouse_state.left_state.is_down;
        mouse_state.left_state.is_down = input.mouse_button_is_down(MouseButton::Left);

        mouse_state.right_state.was_down = mouse_state.right_state.is_down;
        mouse_state.right_state.is_down = input.mouse_button_is_down(MouseButton::Right);
    }
}
