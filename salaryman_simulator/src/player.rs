use bevy::{input::*, prelude::*};

use crate::components::{Desk, InteractionHintUI, Person};

#[derive(Component)]
pub struct Player;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Person), With<Player>>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * player.speed * 0.02;
        }
    }
}

pub fn player_check_collision(
    query: Query<(Entity, &Transform, &Person), With<Player>>,
    desk_query: Query<(Entity, &Transform), With<Desk>>,
    mut interaction_hint: Query<(&mut Visibility, &mut Text, &InteractionHintUI)>,
) {
    for (_, player_transform, _) in query.iter() {
        for (_, desk_transform) in desk_query.iter() {
            let distance = player_transform
                .translation
                .distance(desk_transform.translation);
            println!("Distance: {}", distance);
            if distance < 60.0 {
                for (mut visibility, mut _text, _hint) in interaction_hint.iter_mut() {
                    if *visibility != Visibility::Visible {
                        *visibility = Visibility::Visible;
                        _text.sections[0].value = _hint
                            .text
                            .clone();
                    }
                }
            } else {
                for (mut visibility, _, _) in interaction_hint.iter_mut() {
                    if *visibility == Visibility::Visible {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}
