use bevy::{app::PluginGroupBuilder, prelude::*};
use zedmartzombiefest::camera::camera::camera_setup;
use zedmartzombiefest::map::{
    map::map_setup,
    soundtrack::{music, volume},
};
use zedmartzombiefest::objects::{
    item::item_setup,
    player::{
        animate_sprite, player_confine, player_direction, player_move, player_stop,
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
            (map_setup, item_setup, music, camera_setup, player_setup),
        )
        .add_systems(
            Update,
            (
                player_move,
                player_direction,
                player_stop,
                player_confine,
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
