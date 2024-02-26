use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub texture: Handle<Image>,
}

pub fn item_setup(mut commands: Commands, query: Query<&Window>, asset_server: Res<AssetServer>) {
    for window in query.iter() {
        let items = vec![
            Item {
                name: "Water".to_string(),
                texture: asset_server.clone().load("sprites/items/37.png"),
            },
            Item {
                name: "Milk".to_string(),
                texture: asset_server.clone().load("sprites/items/41.png"),
            },
            Item {
                name: "Burger".to_string(),
                texture: asset_server.clone().load("sprites/items/0.png"),
            },
            Item {
                name: "Cabbage".to_string(),
                texture: asset_server.clone().load("sprites/items/63.png"),
            },
        ];

        let window_width = window.width() / 2.;
        let window_height = window.height() / 2.;

        for item in items {
            let (mut rand_x, mut rand_y) = rand_item_position(window_width, window_height);

            while (rand_x == 0. && rand_y == 0.)
                && (rand_x < -window_width + 64.
                    || rand_x > window_width - 64.
                    || rand_y < -window_height + 64.
                    || rand_y > window_height - 64.)
            {
                (rand_x, rand_y) = rand_item_position(window_width, window_height);
            }

            commands.spawn((
                item.clone(),
                SpriteSheetBundle {
                    texture: item.clone().texture,
                    transform: Transform {
                        translation: Vec3::new(rand_x, rand_y, -10.),
                        scale: Vec3::splat(1.2),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

pub fn rand_item_position(window_width: f32, window_height: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let mut nums: Vec<f32> = vec![-1., 1.];
    nums.shuffle(&mut rng);

    let rand_x = nums[0] * random::<f32>() * window_width;
    let rand_y = nums[0] * random::<f32>() * window_height;

    (rand_x, rand_y)
}
