use crate::animation::animate::{AnimationIndices, AnimationTimer};

use super::direction::Direction;
use bevy::prelude::*;

const FPS: f32 = 10.;

#[derive(Component)]
pub struct Zombie {
    pub hit_box: Vec3,
    pub direction: Direction,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
    pub postion: Vec3,
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

    commands.spawn((
        Zombie {
            hit_box: Vec3::new(16., 32., 0.),
            direction: Direction::None,
            layout: texture_atlas_layout.clone(),
            animation_indices: AnimationIndices { first: 0, last: 0 },
            postion: Vec3::new(0., 0., 0.),
        },
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
    ));
}

pub fn zombie_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Zombie, &mut Transform)>,
) {
    for (mut zombie, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            zombie.direction = Direction::Up;

            transform.translation.y += 200. * time.delta_seconds();
            zombie.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            zombie.direction = Direction::Right;

            transform.translation.x += 200. * time.delta_seconds();
            zombie.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            zombie.direction = Direction::Down;

            transform.translation.y += -200. * time.delta_seconds();
            zombie.postion = transform.translation;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            zombie.direction = Direction::Left;

            transform.translation.x += -200. * time.delta_seconds();
            zombie.postion = transform.translation;
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
