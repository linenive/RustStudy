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

#[derive(Reflect, Debug, Clone, Eq, PartialEq, Hash)]
pub enum InteractionType {
    Invalid,
    Work,
    Damage,
}

#[derive(Reflect, Component)]
pub struct InteractionTarget {
    pub is_interactable: bool,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

#[derive(Reflect, Component)]
pub struct Worker {
    pub salary: Salary,
}

impl Default for Worker {
    fn default() -> Self {
        Worker {
            salary: Salary {
                amount: 1000.0,
                currency: "KRW".to_string(),
            },
        }
    }
}

#[derive(Reflect, Component)]
pub struct Salary {
    pub amount: f32,
    pub currency: String,
}

impl Default for Salary {
    fn default() -> Self {
        Salary {
            amount: 0.0,
            currency: "KRW".to_string(),
        }
    }
}

impl Salary {
    pub fn in_man_won(&self) -> String {
        format!("{}만원", self.amount)
    }

    pub fn in_won(&self) -> String {
        format!("{}원", self.amount * 10000.0)
    }
}

#[derive(Component)]
pub struct Desk;

#[derive(Component)]
pub struct Interactable {
    pub interaction_type: InteractionType,
}

impl Default for Interactable {
    fn default() -> Self {
        Interactable {
            interaction_type: InteractionType::Invalid,
        }
    }
}
