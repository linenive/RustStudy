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

#[derive(Component, Reflect)]
pub struct ChoiceUI {
    pub choices: Vec<String>,
}
