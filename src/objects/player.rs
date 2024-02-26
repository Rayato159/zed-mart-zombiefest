use crate::animation::animate::{AnimationIndices, AnimationTimer};

use super::{direction::Direction, item::Item};
use bevy::prelude::*;

const FPS: f32 = 10.;

#[derive(Component)]
pub struct Player {
    pub username: String,
    pub hit_box: Vec3,
    pub is_dead: bool,
    pub direction: Direction,
    pub is_moving: bool,
    pub items: Vec<Item>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
    pub postion: Vec3,
    pub is_win: bool,
}

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let asset = asset_server.clone();
    let texture = asset.load("sprites/characters/me.sprite.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 9, 12, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

    commands.spawn((
        Player {
            username: "Me".to_string(),
            hit_box: Vec3::new(16., 32., 0.),
            is_dead: false,
            direction: Direction::None,
            is_moving: false,
            items: vec![],
            layout: texture_atlas_layout.clone(),
            animation_indices: AnimationIndices { first: 0, last: 0 },
            postion: Vec3::new(0., 0., 0.),
            is_win: false,
        },
        SpriteSheetBundle {
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 19,
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(1. / FPS, TimerMode::Repeating)),
    ));
}

pub fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            player.direction = Direction::Up;
            player.is_moving = true;

            transform.translation.y += 200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            player.direction = Direction::Right;
            player.is_moving = true;

            transform.translation.x += 200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            player.direction = Direction::Down;
            player.is_moving = true;

            transform.translation.y += -200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            player.direction = Direction::Left;
            player.is_moving = true;

            transform.translation.x += -200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.just_released(KeyCode::KeyW)
            || keyboard_input.just_released(KeyCode::KeyD)
            || keyboard_input.just_released(KeyCode::KeyS)
            || keyboard_input.just_released(KeyCode::KeyA)
        {
            player.is_moving = false;
        }
    }
}

pub fn player_direction(mut query: Query<(&mut Player, &mut TextureAtlas)>) {
    for (mut player, mut atlas) in query.iter_mut() {
        match player.direction {
            Direction::Up => {
                if atlas.index <= 72 || atlas.index >= 80 {
                    atlas.index = 72;
                    player.animation_indices = AnimationIndices {
                        first: 72,
                        last: 80,
                    };
                }
            }
            Direction::Right => {
                if atlas.index <= 99 || atlas.index >= 107 {
                    atlas.index = 99;
                    player.animation_indices = AnimationIndices {
                        first: 99,
                        last: 107,
                    };
                }
            }
            Direction::Down => {
                if atlas.index <= 91 || atlas.index >= 98 {
                    atlas.index = 91;
                    player.animation_indices = AnimationIndices {
                        first: 91,
                        last: 98,
                    };
                }
            }
            Direction::Left => {
                if atlas.index <= 81 || atlas.index >= 89 {
                    atlas.index = 81;
                    player.animation_indices = AnimationIndices {
                        first: 81,
                        last: 89,
                    };
                }
            }
            Direction::None => {
                atlas.index = 19;
                player.animation_indices = AnimationIndices {
                    first: 19,
                    last: 19,
                };
            }
        };
    }
}

pub fn player_stop(mut query: Query<(&mut Player, &mut TextureAtlas)>) {
    for (mut player, mut atlas) in query.iter_mut() {
        match player.direction {
            Direction::Up => {
                if player.animation_indices.first >= 72
                    && player.animation_indices.first <= 80
                    && !player.is_moving
                {
                    atlas.index = 72;
                    player.animation_indices = AnimationIndices {
                        first: 72,
                        last: 72,
                    };
                }
            }
            Direction::Right => {
                if player.animation_indices.first >= 99
                    && player.animation_indices.first <= 107
                    && !player.is_moving
                {
                    atlas.index = 99;
                    player.animation_indices = AnimationIndices {
                        first: 99,
                        last: 99,
                    };
                }
            }
            Direction::Down => {
                if player.animation_indices.first >= 91
                    && player.animation_indices.first <= 98
                    && !player.is_moving
                {
                    atlas.index = 91;
                    player.animation_indices = AnimationIndices {
                        first: 91,
                        last: 91,
                    };
                }
            }
            Direction::Left => {
                if player.animation_indices.first >= 81
                    && player.animation_indices.first <= 89
                    && !player.is_moving
                {
                    atlas.index = 81;
                    player.animation_indices = AnimationIndices {
                        first: 81,
                        last: 81,
                    };
                }
            }
            Direction::None => {
                atlas.index = 19;
                player.animation_indices = AnimationIndices {
                    first: 19,
                    last: 19,
                };
            }
        };
    }
}

pub fn player_confine(mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x < -320. + 16. {
            transform.translation.x = -320. + 16.;
        }
        if transform.translation.x > 320. - 16. {
            transform.translation.x = 320. - 16.;
        }
        if transform.translation.y < -320. + 32. {
            transform.translation.y = -320. + 32.;
        }
        if transform.translation.y > 320. - 20. {
            transform.translation.y = 320. - 20.;
        }
    }
}

pub fn is_player_win(mut commands: Commands, mut player_query: Query<&mut Player>) {
    for mut player in player_query.iter_mut() {
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

            player.is_win = true;
        }
    }
}