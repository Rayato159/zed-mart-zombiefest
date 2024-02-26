use std::collections::HashMap;

use crate::animation::animate::{AnimationIndices, AnimationTimer};

use super::player::Player;
use bevy::prelude::*;

const FPS: f32 = 10.;

#[derive(Component, Debug, Clone)]
pub struct Zombie {
    pub id: String,
    pub hit_box: Vec3,
    pub direction_vector: Vec3,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
    pub postion: Vec3,
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
        direction_vector: Vec3::new(0., 0., 0.),
        layout: texture_atlas_layout.clone(),
        animation_indices: AnimationIndices { first: 0, last: 0 },
        postion: Vec3::new(0., 0., 0.),
    };

    let zombie_entity = commands.spawn((
        zombie.clone(),
        SpriteSheetBundle {
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 19,
            },
            transform: Transform {
                translation: Vec3::new(320., 320., 0.),
                scale: Vec3::splat(1.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(1. / FPS, TimerMode::Repeating)),
    )).id();

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

            if distance > 32. {
                zombie.direction_vector = transform.translation;
                transform.translation += 100. * time.delta_seconds();
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

pub fn despawn_zombie(
    mut commands: Commands,
    player_query: Query<&Player>,
    zombie_query: Query<&Zombie>,
    zombie_entity_query: Query<&ZombieEntity>,
) {
    for player in player_query.iter() {
        if player.is_win {
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