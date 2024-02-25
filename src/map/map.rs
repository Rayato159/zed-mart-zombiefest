use bevy::prelude::*;

pub fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor_texture = asset_server.load("map/floor.png");

    commands.spawn((
        SpriteBundle {
            texture: floor_texture.clone(),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.,
        },
    ));
}
