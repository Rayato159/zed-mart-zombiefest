use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asset = asset_server.clone();
    commands.spawn((
        AudioBundle {
            source: asset.load("audios/background_music.ogg"),
            ..default()
        },
        BackgroundMusic,
    ));
}

pub fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&AudioSink, &BackgroundMusic)>,
) {
    for (sink, _) in query.iter() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
        }
    }
}
