use bevy::prelude::*;

#[derive(Reflect, Component)]
pub struct Person {
    pub speed: f32,
    pub hp: i32,
    pub san: i32,
    pub is_dead: bool,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            speed: 200.0,
            hp: 100,
            san: 100,
            is_dead: false,
        }
    }
}

#[derive(Component)]
pub struct Desk;

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct InteractionHintUI {
    pub text: String,
}

#[derive(Component)]
pub struct PopUpUI {
    pub text: String,
}

#[derive(Component)]
pub struct StatusHUD;
