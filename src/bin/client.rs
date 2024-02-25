use bevy::{app::PluginGroupBuilder, prelude::*};
use zedmartzombiefest::objects::player::{
    animate_sprite, character_confine, character_direction, character_move, is_player_moving,
    player_setup,
};
use zedmartzombiefest::camera::camera::camera_setup;
use zedmartzombiefest::map::map::map_setup;

const WINDOW_TITLE: &str = "Zed Mart Zombiefest";
const WINDOW_RESOLUTION: Vec2 = Vec2::new(640., 640.);

fn main() {
    let default_plugins = default_plugins_setup();

    App::new()
        .add_plugins(default_plugins.build())
        .add_systems(Startup, (map_setup, camera_setup, player_setup))
        .add_systems(
            Update,
            (
                character_move,
                character_direction,
                is_player_moving,
                character_confine,
                animate_sprite,
            ),
        )
        .add_systems(Update, animate_sprite)
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
