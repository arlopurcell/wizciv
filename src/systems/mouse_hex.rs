use amethyst::{
    assets::Handle,
    core::{
        math::{Point3, Vector2, Vector3},
        SystemDesc, Transform,
    },
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Entities, Entity, Join, NullStorage, Read, ReadExpect,
        ReadStorage, System, SystemData, World, WriteStorage, Write,
    },
    input::{InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::hex_grid::HexCoord;
use crate::wizciv::MouseState;

#[derive(SystemDesc, Default)]
pub struct MouseHexSystem;

impl<'s> System<'s> for MouseHexSystem {
    type SystemData = (
        Write<'s, MouseState>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            mut mouse_state,
            transforms,
            cameras,
            input,
            dimensions,
        ): Self::SystemData,
    ) {
        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());

        let mouse_hex: Option<HexCoord> = (&cameras, &transforms).join().next().and_then(|(c, t)| input.mouse_position().map(|mp| (c, t, mp))).map(|(camera, transform, (screen_x, screen_y))| camera.screen_to_world_point(Point3::new(screen_x, screen_y, 0.0), screen_dimensions, transform).into());

        if let Some(mouse_hex) = mouse_hex {
            mouse_state.hex = mouse_hex;
        }

        mouse_state.left_state.was_down = mouse_state.left_state.is_down;
        mouse_state.left_state.is_down = input.mouse_button_is_down(MouseButton::Left);

        mouse_state.right_state.was_down = mouse_state.right_state.is_down;
        mouse_state.right_state.is_down = input.mouse_button_is_down(MouseButton::Right);
    }
}
