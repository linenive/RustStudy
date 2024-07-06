use bevy::{input::*, prelude::*};

use crate::components::{Interactable, InteractionTarget, Person};
use crate::gui::components::{InteractionHintUI, PopUpUI};

#[derive(Component)]
pub struct Player;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    interactable_target_query: Query<(Entity, &InteractionTarget)>,
    mut query: Query<(&mut Transform, &mut Person, &Player)>,
) {
    for (mut transform, mut person, _) in query.iter_mut() {
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
            for (_, interact_target) in interactable_target_query.iter() {
                if interact_target.is_interactable {
                    person.san -= 10;
                }
            }
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * person.speed * 0.02;
        }
    }
}

pub fn player_check_collision(
    query: Query<(&Transform, &Player)>,
    interactable_query: Query<(Entity, &Transform, &Name), With<Interactable>>,
    mut interaction_target_query: Query<(Entity, &mut InteractionTarget)>,
    mut interaction_hint: Query<(&mut Visibility, &mut Text, &InteractionHintUI)>,
) {
    for (player_transform, _) in query.iter() {
        let max_distance = 60.0;
        let mut closest_distance = max_distance;
        let mut closest_target = Entity::PLACEHOLDER;
        let mut closest_name = "".to_string();

        for (target, interactable_transform, name) in interactable_query.iter() {
            let distance = player_transform
                .translation
                .distance(interactable_transform.translation);
            if distance < closest_distance {
                closest_distance = distance;
                closest_target = target;
                closest_name = name
                    .as_str()
                    .to_string();
            }
        }

        println!("Closest Daistance: {}", closest_distance);

        // 인터랙션 가능할 만큼 충분히 가까운 대상이 존재함
        if closest_distance < max_distance {
            for (_, mut interact_target) in interaction_target_query.iter_mut() {
                interact_target.is_interactable = true;

                if interact_target.target != closest_target {
                    interact_target.target = closest_target;
                    for (mut visibility, mut _text, _hint) in interaction_hint.iter_mut() {
                        *visibility = Visibility::Visible;
                        _text.sections[0].value =
                            format!("{} ({})", _hint.text, closest_name).to_string();
                    }
                }
            }
        }
        // 인터랙션 가능한 대상이 없음
        else {
            for (mut visibility, _, _) in interaction_hint.iter_mut() {
                if *visibility == Visibility::Visible {
                    *visibility = Visibility::Hidden;
                }
            }

            for (_, mut interact_target) in interaction_target_query.iter_mut() {
                interact_target.is_interactable = false;
                interact_target.target = Entity::PLACEHOLDER;
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
