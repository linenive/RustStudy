use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod components;
pub mod player;

use components::{Desk, Interactable, InteractionHintUI, Person, PopUpUI, StatusHUD};
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
    commands.spawn((
        Person::default(),
        Name::new(name.to_string()),
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

fn add_pop_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                font: asset_server.load("/System/Library/Fonts/Supplemental/AppleGothic.ttf"),
                ..Default::default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        }),
        PopUpUI {
            text: "당신은 죽었습니다.".to_string(),
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

fn update_hud(
    mut query: Query<&mut Text, With<StatusHUD>>,
    player_query: Query<&Person, With<Player>>,
    time: Res<Time>,
) {
    for mut _text in query.iter_mut() {
        for player in player_query.iter() {
            _text.sections[0].value = format!("체력: {}\n정신력: {}", player.hp, player.san);
            _text.sections[0]
                .value
                .push_str(&format!(
                    "\n지난 시간: {:.2}",
                    time.elapsed_seconds() as f32
                ));
        }
    }
}

fn update_pop_up(mut query: Query<(&mut Text, &mut Transform), With<PopUpUI>>, time: Res<Time>) {
    for (mut _text, mut _transform) in query.iter_mut() {
        let million = time.elapsed_seconds() % 8.0;
        if million < 4.0 {
            let scale = ((time.elapsed_seconds() * 30.0).sin() + 2.1) * 1.0;
            _transform
                .scale
                .x = scale;
            _transform
                .scale
                .y = scale;
        } else {
            _text.sections[0]
                .style
                .color = Color::RED;
            // 회전
            _transform.rotation = Quat::from_rotation_z((time.elapsed_seconds() * 2.0).tan());
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
                    add_player, add_people, add_desk, add_text, add_hud, add_pop_up,
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
                    update_pop_up,
                    player::dead_player,
                )
                    .chain(),
            );
    }
}
