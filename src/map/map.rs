use bevy::prelude::*;

pub fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor_texture = asset_server.load("map/floor.png");

    commands.spawn((
        SpriteBundle {
            texture: floor_texture.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -20.),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.,
        },
    ));
}
