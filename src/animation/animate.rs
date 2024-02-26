use bevy::prelude::*;

use crate::objects::{player::Player, zombie::Zombie};

#[derive(Component, Debug, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (player, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == player.animation_indices.last {
                player.animation_indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

pub fn animate_zombie_sprite(
    time: Res<Time>,
    mut query: Query<(&Zombie, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (zombie, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == zombie.animation_indices.last {
                zombie.animation_indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}
