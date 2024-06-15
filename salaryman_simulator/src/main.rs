use bevy::{
    input::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, HelloPlugin));
    app.register_type::<Person>(); // 인스펙터에 표시하기 위해 Person 타입을 등록

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugins(WorldInspectorPlugin::new());
    // Startup system (cameras)
    app.add_systems(Startup, sprite_setup);
    app.add_systems(Update, player_movement);
    // Run the app
    app.run();
}

const X_EXTENT: f32 = 600.;

fn sprite_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Reflect, Component)]
struct Person {
    speed: f32,
}

impl Default for Person {
    fn default() -> Self {
        Person { speed: 200.0 }
    }
}

#[derive(Component)]
struct Desk;

fn add_desk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    let color = Color::hsl(0.0, 0.0, 0.5);
    commands.spawn((
        Desk,
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
    commands.spawn(TextBundle {
        text: Text::from_section(
            "안녕하세요, Bevy!",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                font: asset_server.load("/System/Library/Fonts/Supplemental/AppleGothic.ttf"),
            },
        ),
        ..Default::default()
    });
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person::default(), Name::new("Elaina Proctor")));
    commands.spawn((Person::default(), Name::new("Renzo Hume")));
    commands.spawn((Person::default(), Name::new("Zayna Nieves")));
}

#[derive(Component)]
struct Player;

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
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
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

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Person), With<Player>>,
) {
    for (mut transform, player) in query.iter_mut() {
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

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * player.speed * 0.02;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(
                Startup,
                (add_player, add_people, add_desk, add_text).chain(),
            )
            .add_systems(Update, (update_people, greet_people).chain());
    }
}
