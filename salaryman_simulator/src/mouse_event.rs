use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{MouseInput, MouseSelectable};

pub fn add_mouse_input(mut commands: Commands) {
    commands.spawn((MouseInput::default(),));
}

pub fn listen_mouse_input(
    mut q_mouse_inputs: Query<&mut MouseInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut q_mouse_input = q_mouse_inputs.single_mut();
    let q_window = q_windows.single();
    if let Some(position) = q_window.cursor_position() {
        q_mouse_input.camera_position = position;
    }

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = q_window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| {
            ray.origin
                .truncate()
        })
    {
        q_mouse_input.world_position = world_position;
    }
}

pub fn mouse_event(
    q_mouse_inputs: Query<&MouseInput>,
    mut query: Query<&Transform, With<MouseSelectable>>,
) {
    let q_mouse_input = q_mouse_inputs.single();

    for transform in query.iter() {
        let x = q_mouse_input
            .world_position
            .x
            - transform
                .translation
                .x;
        let y = q_mouse_input
            .world_position
            .y
            - transform
                .translation
                .y;

        if x.abs() < 50.0 && y.abs() < 50.0 {
            println!("Mouse is on the object!");
            continue;
        }
    }
}
