use bevy::{app::PluginGroupBuilder, prelude::*};
use zedmartzombiefest::camera::camera::camera_setup;
use zedmartzombiefest::map::{
    map::map_setup,
    soundtrack::{music, volume},
};
use zedmartzombiefest::objects::{
    item::item_setup,
    player::{
        animate_sprite, character_confine, character_direction, character_move, character_stop,
        player_setup,
    },
};

const WINDOW_TITLE: &str = "Zed Mart Zombiefest";
const WINDOW_RESOLUTION: Vec2 = Vec2::new(640., 640.);

fn main() {
    let default_plugins = default_plugins_setup();

    App::new()
        .add_plugins(default_plugins.build())
        .add_systems(
            Startup,
            (map_setup, music, camera_setup, player_setup, item_setup),
        )
        .add_systems(
            Update,
            (
                character_move,
                character_direction,
                character_stop,
                character_confine,
                animate_sprite,
                volume,
            ),
        )
        .run();
}

fn default_plugins_setup() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                resolution: WINDOW_RESOLUTION.into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest())
}
