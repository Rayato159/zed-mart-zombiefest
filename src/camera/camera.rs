use bevy::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
