use bevy::prelude::*;

use super::player::Player;

#[derive(Component, Debug, PartialEq)]
pub enum GameState {
    Start,
    Win,
    GameOver,
}

#[derive(Component, Debug)]
pub struct GoalText;

pub fn start(mut commands: Commands) {
    commands.spawn(GameState::Start);
}

pub fn goal_text_setup(mut commands: Commands) {
    let text = String::from("Your items: 0/4");

    commands.spawn((
        TextBundle::from_section(
            text.clone(),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(16.),
            left: Val::Px(16.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }),
        GoalText,
    ));
}

pub fn update_goal_text(
    player_query: Query<&Player>,
    mut goal_text_query: Query<(&GoalText, &mut Text)>,
) {
    for player in player_query.iter() {
        for (_, mut text) in goal_text_query.iter_mut() {
            text.sections[0].value = format!("Your items: {}/4", player.items.len());
        }
    }
}

pub fn is_win(
    mut commands: Commands,
    mut player_query: Query<&Player>,
    mut game_query: Query<&mut GameState>,
) {
    for mut game in game_query.iter_mut() {
        for player in player_query.iter_mut() {
            if player.items.len() == 4 {
                commands.spawn((TextBundle::from_section(
                    "You Win!",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(270.),
                    left: Val::Px(130.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }),));

                *game = GameState::Win;
            }
        }
    }
}

pub fn is_game_over(
    mut commands: Commands,
    mut player_query: Query<&Player>,
    mut game_query: Query<&mut GameState>,
) {
    for mut game in game_query.iter_mut() {
        for player in player_query.iter_mut() {
            if player.is_dead {
                commands.spawn((TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(270.),
                    left: Val::Px(110.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }),));

                *game = GameState::GameOver;
            }
        }
    }
}

pub fn exit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit);
    }
}
