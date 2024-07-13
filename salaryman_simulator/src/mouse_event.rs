use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

use crate::components::{MouseHoverHint, MouseInput, MouseSelectable};

pub fn add_mouse_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MouseInput::default());

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    let color = Color::rgba(1.0, 1.0, 1.0, 0.1);
    commands.spawn((
        MouseHoverHint,
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    ));
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
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Visibility), With<MouseHoverHint>>,
        Query<&Transform, With<MouseSelectable>>,
    )>,
) {
    let q_mouse_input = q_mouse_inputs.single();

    let mut target_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let mut is_hovered = false;
    for selectable_transform in param_set
        .p1()
        .iter()
    {
        let x = q_mouse_input
            .world_position
            .x
            - selectable_transform
                .translation
                .x;
        let y = q_mouse_input
            .world_position
            .y
            - selectable_transform
                .translation
                .y;

        if x.abs() < 50.0 && y.abs() < 50.0 {
            println!("Mouse is on the object!");
            target_transform = selectable_transform.clone();
            is_hovered = true;
            break;
        }
    }

    for (mut hint_transform, mut hint_visibility) in param_set
        .p0()
        .iter_mut()
    {
        if !is_hovered {
            *hint_visibility = Visibility::Hidden;
            return;
        } else {
            *hint_visibility = Visibility::Visible;
            hint_transform
                .translation
                .x = target_transform
                .translation
                .x;
            hint_transform
                .translation
                .y = target_transform
                .translation
                .y;
        }
    }
}
