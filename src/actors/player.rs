use bevy::prelude::*;

use crate::{NodeSize, Player, Position};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(update_player_position_system)
            .add_system(render);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _asset_server: Res<AssetServer>,
) {
    commands
        .spawn_empty()
        .insert(Player {})
        .insert(Position((20, 20)))
        .insert(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        });
}

fn update_player_position_system(
    key_code: Res<Input<KeyCode>>,
    mut query: Query<&mut Position, With<Player>>,
) {
    for mut position in &mut query {
        if key_code.just_pressed(KeyCode::Left) {
            *position = Position((position.0 .0, position.0 .1 - 1));
        } else if key_code.just_pressed(KeyCode::Right) {
            *position = Position((position.0 .0, position.0 .1 + 1));
        } else if key_code.just_pressed(KeyCode::Up) {
            *position = Position((position.0 .0 - 1, position.0 .1));
        } else if key_code.just_pressed(KeyCode::Down) {
            *position = Position((position.0 .0 + 1, position.0 .1));
        }
    }
}

fn render(
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    mut query: Query<(&Position, &mut Transform), With<Player>>,
) {
    let window = windows.primary();

    for (position, mut transform) in &mut query {
        let (w, h) = (window.width(), window.height());

        transform.translation.x =
            -w / 2. + position.0 .1 as f32 * node_size.0 .0 + node_size.0 .0 / 2.;
        transform.translation.y =
            h / 2. - position.0 .0 as f32 * node_size.0 .1 - node_size.0 .1 / 2.;
        transform.translation.z = 101.;
    }
}
