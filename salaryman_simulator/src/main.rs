use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod components;
pub mod gui;
pub mod mouse_event;
pub mod player;

use components::{Desk, Interactable, InteractionTarget, InteractionType, Person, Salary, Worker};
use gui::components::{ChoiceUI, StatusHUD};
use player::Player;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, HelloPlugin));
    // 인스펙터에 표시하기 위해 타입들을 등록
    app.register_type::<Vec<String>>();
    app.register_type::<Person>();
    app.register_type::<Worker>();
    app.register_type::<Salary>();
    app.register_type::<InteractionTarget>();
    app.register_type::<ChoiceUI>();

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugins(WorldInspectorPlugin::new());
    // Startup system (cameras)
    app.add_systems(Startup, sprite_setup);
    // Run the app
    app.run();
}

fn sprite_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_system_entity(mut commands: Commands) {
    commands.spawn((
        Name::new("InteractTarget"),
        InteractionTarget {
            is_interactable: false,
            target: Entity::PLACEHOLDER,
            target_transform: Transform::from_xyz(0.0, 0.0, 0.0),
            interaction_type: InteractionType::Invalid,
        },
    ));
}

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 20.0 }));
    let color = Color::hsl(0.0, 0.95, 0.7);
    commands.spawn((
        Player,
        Person::default(),
        Name::new("Player"),
        Worker::default(),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn add_person(
    name: &str,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 20.0 }));
    let color = Color::hsl(40.0, 0.35, 0.7);
    let random_transform = Transform::from_xyz(
        rand::random::<f32>() * 800.0 - 400.0,
        rand::random::<f32>() * 800.0 - 400.0,
        0.0,
    );
    let random_salary = Salary {
        amount: rand::random::<f32>() * 3000.0,
        currency: "KRW".to_string(),
    };
    commands.spawn((
        Person::default(),
        Name::new(name.to_string()),
        Worker {
            salary: random_salary,
        },
        Interactable {
            interaction_type: InteractionType::SalaryMan,
        },
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: random_transform,
            ..default()
        },
    ));
}

fn add_desk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    let color = Color::hsl(0.0, 0.0, 0.5);
    commands.spawn((
        Desk,
        Interactable {
            interaction_type: InteractionType::Work,
        },
        Name::new("Desk"),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
    ));
}

fn add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    add_person("Alice", &mut commands, &mut meshes, &mut materials);
    add_person("Bob", &mut commands, &mut meshes, &mut materials);
    add_person("Charlie", &mut commands, &mut meshes, &mut materials);
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer
        .0
        .tick(time.delta())
        .just_finished()
    {
        for name in &query {
            println!("hello {}!", name.as_str());
        }
    }
}

fn update_people(mut query: Query<&mut Person>) {
    for mut person in &mut query {
        if person.hp <= 0 || person.san <= 0 {
            person.is_dead = true;
        }
    }
}

// 캐릭터의 상태를 표시하는 HUD
fn update_hud(
    mut query: Query<&mut Text, With<StatusHUD>>,
    player_query: Query<&Person, With<Player>>,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for mut _text in query.iter_mut() {
        for player in player_query.iter() {
            _text.sections[0].value = format!("체력: {}\n정신력: {}\n", player.hp, player.san);
            _text.sections[0]
                .value
                .push_str(&format!(
                    "지난 시간: {:.1}\n",
                    time.elapsed_seconds() as f32
                ));
        }

        if let Some(position) = q_windows
            .single()
            .cursor_position()
        {
            _text.sections[0]
                .value
                .push_str(format!("Cursor [{:.1}, {:.1}]", position.x, position.y).as_str());
        } else {
            _text.sections[0]
                .value
                .push_str("Cursor is not in the game window.");
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(
                Startup,
                (
                    add_system_entity,
                    add_player,
                    add_people,
                    add_desk,
                    gui::setup_font,
                    gui::add_gui,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    update_people,
                    greet_people,
                    player::player_check_collision,
                    update_hud,
                    gui::update_pop_up,
                    gui::update_choice_ui,
                    player::player_movement,
                    player::interact,
                    player::dead_player,
                    mouse_event::mouse_motion,
                )
                    .chain(),
            );
    }
}
