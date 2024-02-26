use std::collections::HashMap;

use bevy::prelude::*;
use rand::prelude::*;

use super::player::Player;

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub texture: Handle<Image>,
    pub position: Vec3,
}

#[derive(Component, Debug, Clone)]
pub struct ItemEntity {
    pub entity_map: HashMap<String, Entity>,
}

pub fn item_setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let mut item_entity_map = HashMap::new();

    for window in window_query.iter() {
        let items = vec![
            Item {
                id: "37".to_string(),
                name: "Water".to_string(),
                texture: asset_server.clone().load("sprites/items/37.png"),
                position: Vec3::new(0., 0., 0.),
            },
            Item {
                id: "41".to_string(),
                name: "Milk".to_string(),
                texture: asset_server.clone().load("sprites/items/41.png"),
                position: Vec3::new(0., 0., 0.),
            },
            Item {
                id: "0".to_string(),
                name: "Burger".to_string(),
                texture: asset_server.clone().load("sprites/items/0.png"),
                position: Vec3::new(0., 0., 0.),
            },
            Item {
                id: "63".to_string(),
                name: "Cabbage".to_string(),
                texture: asset_server.clone().load("sprites/items/63.png"),
                position: Vec3::new(0., 0., 0.),
            },
        ];

        let window_width = window.width() / 2.;
        let window_height = window.height() / 2.;

        for mut item in items {
            let (mut rand_x, mut rand_y) = rand_item_position(window_width, window_height);

            while (rand_x == 0. && rand_y == 0.)
                && (rand_x < -window_width + 64.
                    || rand_x > window_width - 64.
                    || rand_y < -window_height + 64.
                    || rand_y > window_height - 64.)
            {
                (rand_x, rand_y) = rand_item_position(window_width, window_height);
            }

            let new_position = Vec3::new(rand_x, rand_y, -10.);

            item.position = new_position.clone();

            let item_entity = commands
                .spawn((
                    item.clone(),
                    SpriteSheetBundle {
                        texture: item.clone().texture,
                        transform: Transform {
                            translation: new_position.clone(),
                            scale: Vec3::splat(1.2),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();

            item_entity_map.insert(item.id.clone(), item_entity);

            commands.spawn(ItemEntity {
                entity_map: item_entity_map.clone(),
            });
        }
    }
}

pub fn collect_item(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    item_query: Query<&Item>,
    item_entity_query: Query<&ItemEntity>,
) {
    for mut player in player_query.iter_mut() {
        for item in item_query.iter() {
            if (player.postion.x - item.position.x).abs() < player.hit_box.x
                && (player.postion.y - item.position.y).abs() < player.hit_box.y
            {
                player.items.push(item.clone());

                for item_entity in item_entity_query.iter() {
                    match item_entity.entity_map.get(&item.id) {
                        Some(item_entity) => {
                            commands.entity(*item_entity).despawn();
                        }
                        None => {}
                    }
                }
            }
        }
    }
}

fn rand_item_position(window_width: f32, window_height: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let mut nums: Vec<f32> = vec![-1., 1.];
    nums.shuffle(&mut rng);

    let rand_x = nums[0] * random::<f32>() * window_width;
    let rand_y = nums[0] * random::<f32>() * window_height;

    (rand_x, rand_y)
}
