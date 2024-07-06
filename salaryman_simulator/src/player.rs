use bevy::{input::*, prelude::*};

use crate::components::{Interactable, Person};
use crate::gui::components::{InteractionHintUI, PopUpUI};

#[derive(Component)]
pub struct Player {
    pub is_interactable: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            is_interactable: false,
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Person, &Player)>,
) {
    for (mut transform, mut person, player) in query.iter_mut() {
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
        if keyboard_input.pressed(KeyCode::KeyE) {
            if player.is_interactable {
                person.san -= 10;
            }
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * person.speed * 0.02;
        }
    }
}

pub fn player_check_collision(
    mut query: Query<(&Transform, &mut Player)>,
    interactable_query: Query<(Entity, &Transform), With<Interactable>>,
    mut interaction_hint: Query<(&mut Visibility, &mut Text, &InteractionHintUI)>,
) {
    for (player_transform, mut player) in query.iter_mut() {
        for (_, interactable_transform) in interactable_query.iter() {
            let distance = player_transform
                .translation
                .distance(interactable_transform.translation);
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
                player.is_interactable = true;
            } else {
                for (mut visibility, _, _) in interaction_hint.iter_mut() {
                    if *visibility == Visibility::Visible {
                        *visibility = Visibility::Hidden;
                    }
                }
                player.is_interactable = false;
            }
        }
    }
}

pub fn dead_player(
    query: Query<(Entity, &Person), With<Player>>,
    mut pop_up_ui: Query<(&mut Visibility, &mut Text, &PopUpUI)>,
) {
    for (_, person) in query.iter() {
        if person.is_dead {
            for (mut visibility, mut _text, _pop_up) in pop_up_ui.iter_mut() {
                if *visibility != Visibility::Visible {
                    *visibility = Visibility::Visible;
                    _text.sections[0].value = _pop_up
                        .text
                        .clone();
                }
            }
        }
    }
}
