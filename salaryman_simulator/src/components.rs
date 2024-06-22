use bevy::prelude::*;

#[derive(Reflect, Component)]
pub struct Person {
    pub speed: f32,
}

impl Default for Person {
    fn default() -> Self {
        Person { speed: 200.0 }
    }
}

#[derive(Component)]
pub struct Desk;

#[derive(Component)]
pub struct InteractionHintUI {
    pub text: String,
}
