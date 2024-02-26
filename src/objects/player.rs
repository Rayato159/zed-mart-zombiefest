use std::collections::HashMap;

use crate::animation::animate::{AnimationIndices, AnimationTimer};

use super::{game::GameState, item::Item};
use bevy::prelude::*;

const FPS: f32 = 10.;

#[derive(Component, Clone)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub hit_box: Vec3,
    pub is_dead: bool,
    pub direction: Vec3,
    pub is_moving: bool,
    pub items: Vec<Item>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
    pub postion: Vec3,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerEntity {
    pub entity_map: HashMap<String, Entity>,
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

    let mut player_entity_map = HashMap::new();

    let player = Player {
        id: "P001".into(),
        username: "Me".to_string(),
        hit_box: Vec3::new(32., 32., 0.),
        is_dead: false,
        direction: Vec3::new(0., 0., 0.),
        is_moving: false,
        items: vec![],
        layout: texture_atlas_layout.clone(),
        animation_indices: AnimationIndices { first: 0, last: 0 },
        postion: Vec3::new(0., 0., 0.),
    };

    let player_entity = commands
        .spawn((
            player.clone(),
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
        ))
        .id();

    player_entity_map.insert(player.clone().id, player_entity);

    commands.spawn(PlayerEntity {
        entity_map: player_entity_map.clone(),
    });
}

pub fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            player.direction = Vec3::new(0., 1., 0.);
            player.is_moving = true;

            transform.translation.y += 200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            player.direction = Vec3::new(1., 0., 0.);
            player.is_moving = true;

            transform.translation.x += 200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            player.direction = Vec3::new(0., -1., 0.);
            player.is_moving = true;

            transform.translation.y += -200. * time.delta_seconds();
            player.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            player.direction = Vec3::new(-1., 0., 0.);
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
        if player.direction == Vec3::new(0., 1., 0.) {
            if atlas.index <= 72 || atlas.index >= 80 {
                atlas.index = 72;
                player.animation_indices = AnimationIndices {
                    first: 72,
                    last: 80,
                };
            }
        }

        if player.direction == Vec3::new(1., 0., 0.) {
            if atlas.index <= 99 || atlas.index >= 107 {
                atlas.index = 99;
                player.animation_indices = AnimationIndices {
                    first: 99,
                    last: 107,
                };
            }
        }

        if player.direction == Vec3::new(0., -1., 0.) {
            if atlas.index <= 91 || atlas.index >= 98 {
                atlas.index = 91;
                player.animation_indices = AnimationIndices {
                    first: 91,
                    last: 98,
                };
            }
        }

        if player.direction == Vec3::new(-1., 0., 0.) {
            if atlas.index <= 81 || atlas.index >= 89 {
                atlas.index = 81;
                player.animation_indices = AnimationIndices {
                    first: 81,
                    last: 89,
                };
            }
        }

        if player.direction == Vec3::new(0., 0., 0.) {
            atlas.index = 19;
            player.animation_indices = AnimationIndices {
                first: 19,
                last: 19,
            };
        };
    }
}

pub fn player_stop(mut query: Query<(&mut Player, &mut TextureAtlas)>) {
    for (mut player, mut atlas) in query.iter_mut() {
        if player.direction == Vec3::new(0., 1., 0.) {
            if atlas.index >= 72 && atlas.index <= 80 && !player.is_moving {
                atlas.index = 72;
                player.animation_indices = AnimationIndices {
                    first: 72,
                    last: 72,
                };
            }
        }

        if player.direction == Vec3::new(1., 0., 0.) {
            if atlas.index >= 99 && atlas.index <= 107 && !player.is_moving {
                atlas.index = 99;
                player.animation_indices = AnimationIndices {
                    first: 99,
                    last: 99,
                };
            }
        }

        if player.direction == Vec3::new(0., -1., 0.) {
            if atlas.index >= 91 && atlas.index <= 98 && !player.is_moving {
                atlas.index = 91;
                player.animation_indices = AnimationIndices {
                    first: 91,
                    last: 91,
                };
            }
        }

        if player.direction == Vec3::new(-1., 0., 0.) {
            if atlas.index >= 81 && atlas.index <= 89 && !player.is_moving {
                atlas.index = 81;
                player.animation_indices = AnimationIndices {
                    first: 81,
                    last: 81,
                };
            }
        }

        if player.direction == Vec3::new(0., 0., 0.) {
            atlas.index = 19;
            player.animation_indices = AnimationIndices {
                first: 19,
                last: 19,
            };
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

pub fn despawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_query: Query<&GameState>,
    player_query: Query<&Player>,
    player_entity_query: Query<&PlayerEntity>,
) {
    for game in game_query.iter() {
        if *game == GameState::GameOver {
            for player in player_query.iter() {
                for player_entity in player_entity_query.iter() {
                    match player_entity.entity_map.get(&player.id) {
                        Some(player_entity) => {
                            let asset = asset_server.clone();
                            let sound_effect = asset.load("audios/dead.ogg");

                            commands.spawn((AudioBundle {
                                source: sound_effect.clone(),
                                settings: PlaybackSettings {
                                    mode: bevy::audio::PlaybackMode::Despawn,
                                    ..default()
                                },
                                ..default()
                            },));

                            commands.entity(*player_entity).despawn();
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
