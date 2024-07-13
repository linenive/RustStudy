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
    pub is_visible: bool,
    pub tranform: Transform,
}

#[derive(Component)]
pub struct ChoiceItem {
    pub index: usize,
}
