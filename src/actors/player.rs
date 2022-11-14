use bevy::prelude::*;

use crate::{Player, Position};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(Player {})
        .insert(Position((20, 20)));
}
