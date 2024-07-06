use bevy::prelude::*;

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
