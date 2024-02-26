use std::collections::HashMap;

use crate::animation::animate::{AnimationIndices, AnimationTimer};

use super::{game::GameState, player::Player};
use bevy::prelude::*;

const FPS: f32 = 10.;

#[derive(Component, Debug, Clone)]
pub struct Zombie {
    pub id: String,
    pub hit_box: Vec3,
    pub direction: Vec3,
    pub postion: Vec3,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
}

#[derive(Component, Debug, Clone)]
pub struct ZombieEntity {
    pub entity_map: HashMap<String, Entity>,
}

pub fn zombie_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let asset = asset_server.clone();
    let texture = asset.load("sprites/zombies/zombie.sprite.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 9, 12, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout.clone());

    let mut zombie_entity_map = HashMap::new();

    let zombie = Zombie {
        id: "Z001".into(),
        hit_box: Vec3::new(16., 32., 0.),
        direction: Vec3::new(0., 0., 0.),
        postion: Vec3::new(320., 320., 0.),
        layout: texture_atlas_layout.clone(),
        animation_indices: AnimationIndices { first: 0, last: 0 },
    };

    let zombie_entity = commands
        .spawn((
            zombie.clone(),
            SpriteSheetBundle {
                texture: texture.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 91,
                },
                transform: Transform {
                    translation: Vec3::new(320., 320., 0.),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(1. / FPS, TimerMode::Repeating)),
        ))
        .id();

    zombie_entity_map.insert(zombie.clone().id, zombie_entity);

    commands.spawn(ZombieEntity {
        entity_map: zombie_entity_map.clone(),
    });
}

pub fn zombie_move(
    time: Res<Time>,
    mut zombie_query: Query<(&mut Zombie, &mut Transform)>,
    player_query: Query<&Player>,
) {
    for player in player_query.iter() {
        for (mut zombie, mut transform) in zombie_query.iter_mut() {
            let dif = player.postion - zombie.postion;
            let distance = dif.length();
            let new_pos = dif.normalize();

            if distance > 32. {
                transform.translation += new_pos * 100. * time.delta_seconds();
                zombie.postion = transform.translation;

                zombie.direction = player.direction;
            }
        }
    }
}

pub fn zombie_animate(mut query: Query<(&mut Zombie, &mut TextureAtlas)>) {
    for (mut zombie, mut atlas) in query.iter_mut() {
        if zombie.direction == Vec3::new(0., 0., 0.) {
            atlas.index = 91;
            zombie.animation_indices = AnimationIndices {
                first: 91,
                last: 91,
            };
        }

        if zombie.direction == Vec3::new(0., 1., 0.) {
            if atlas.index <= 72 || atlas.index >= 80 {
                atlas.index = 72;
                zombie.animation_indices = AnimationIndices {
                    first: 72,
                    last: 80,
                };
            }
        }

        if zombie.direction == Vec3::new(1., 0., 0.) {
            if atlas.index <= 99 || atlas.index >= 107 {
                atlas.index = 99;
                zombie.animation_indices = AnimationIndices {
                    first: 99,
                    last: 107,
                };
            }
        }

        if zombie.direction == Vec3::new(0., -1., 0.) {
            if atlas.index <= 91 || atlas.index >= 98 {
                atlas.index = 91;
                zombie.animation_indices = AnimationIndices {
                    first: 91,
                    last: 98,
                };
            }
        }

        if zombie.direction == Vec3::new(-1., 0., 0.) {
            if atlas.index <= 81 || atlas.index >= 89 {
                atlas.index = 81;
                zombie.animation_indices = AnimationIndices {
                    first: 81,
                    last: 89,
                };
            }
        }
    }
}

pub fn zombie_confine(mut query: Query<&mut Transform>) {
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

pub fn zombie_kill(zombie_query: Query<&Zombie>, mut player_query: Query<&mut Player>) {
    for zombie in zombie_query.iter() {
        for mut player in player_query.iter_mut() {
            if (player.postion.x - zombie.postion.x).abs() < player.hit_box.x
                && (player.postion.y - zombie.postion.y).abs() < player.hit_box.y
            {
                player.is_dead = true;
            }
        }
    }
}

pub fn despawn_zombie(
    mut commands: Commands,
    game_query: Query<&GameState>,
    zombie_query: Query<&Zombie>,
    zombie_entity_query: Query<&ZombieEntity>,
) {
    for game in game_query.iter() {
        if *game == GameState::Win {
            for zombie in zombie_query.iter() {
                for zombie_entity in zombie_entity_query.iter() {
                    match zombie_entity.entity_map.get(&zombie.id) {
                        Some(zombie_entity) => {
                            commands.entity(*zombie_entity).despawn();
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
