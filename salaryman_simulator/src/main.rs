use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod components;
pub mod player;

use components::{Desk, Interactable, InteractionHintUI, Person, StatusHUD};
use player::Player;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, HelloPlugin));
    app.register_type::<Person>(); // 인스펙터에 표시하기 위해 Person 타입을 등록

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugins(WorldInspectorPlugin::new());
    // Startup system (cameras)
    app.add_systems(Startup, sprite_setup);
    app.add_systems(Update, player::player_movement);
    // Run the app
    app.run();
}

fn sprite_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 20.0 }));
    let color = Color::hsl(0.0, 0.95, 0.7);
    commands.spawn((
        Player::default(),
        Person::default(),
        Name::new("Player"),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
        Interactable,
        Name::new("Desk"),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
    ));
}

fn add_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "안녕하세요, Bevy!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: asset_server.load("/System/Library/Fonts/Supplemental/AppleGothic.ttf"),
                },
            ),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        InteractionHintUI {
            text: "[E]를 눌러 상호작용하기".to_string(),
        },
    ));
}

fn add_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                font: asset_server.load("/System/Library/Fonts/Supplemental/AppleGothic.ttf"),
                ..Default::default()
            },
        )
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..Default::default()
        }),
        StatusHUD,
    ));
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person::default(), Name::new("Elaina Proctor")));
    commands.spawn((Person::default(), Name::new("Renzo Hume")));
    commands.spawn((Person::default(), Name::new("Zayna Nieves")));
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

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.as_str() == "Elaina Proctor" {
            name.set("Elaina Hume");
            break; // We don’t need to change any other names
        }
    }
}

fn update_hud(
    mut query: Query<&mut Text, With<StatusHUD>>,
    player_query: Query<&Person, With<Player>>,
) {
    for mut _text in query.iter_mut() {
        for player in player_query.iter() {
            _text.sections[0].value = format!("체력: {}\n정신력: {}", player.hp, player.san);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(
                Startup,
                (add_player, add_people, add_desk, add_text, add_hud).chain(),
            )
            .add_systems(
                Update,
                (
                    update_people,
                    greet_people,
                    player::player_check_collision,
                    update_hud,
                )
                    .chain(),
            );
    }
}
