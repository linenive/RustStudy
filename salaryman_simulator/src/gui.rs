use bevy::prelude::*;

pub mod components;

use components::{InteractionHintUI, PopUpUI, StatusHUD};

#[derive(Resource)]
pub struct MyFont(Handle<Font>);

pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("/System/Library/Fonts/Supplemental/AppleGothic.ttf");

    commands.insert_resource(MyFont(font));
}

pub fn add_gui(mut commands: Commands, font: Res<MyFont>) {
    add_text(&mut commands, &font);
    add_hud(&mut commands, &font);
    add_pop_up(&mut commands, &font);
}

// 게임 오버 팝업을 업데이트
pub fn update_pop_up(
    mut query: Query<(&mut Text, &mut Transform), With<PopUpUI>>,
    time: Res<Time>,
) {
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

// 게임오버 팝업을 추가
fn add_pop_up(commands: &mut Commands, font: &Res<MyFont>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                font: font
                    .0
                    .clone(),
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

// 상호작용 힌트를 추가
fn add_text(commands: &mut Commands, font: &Res<MyFont>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "안녕하세요, Bevy!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: font
                        .0
                        .clone(),
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

// 캐릭터의 상태를 표시하는 HUD를 추가
fn add_hud(commands: &mut Commands, font: &Res<MyFont>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                font: font
                    .0
                    .clone(),
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

fn add_choice_ui(commands: &mut Commands, font: &Res<MyFont>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                font: font
                    .0
                    .clone(),
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
