use bevy::prelude::*;

use crate::{game::coordinates::Coordinates, Position};
pub struct AssetsPlugin;

#[derive(Resource)]
pub(crate) struct GridTextures {
    floor_tile_0: Handle<Image>,
    floor_tile_1: Handle<Image>,
    floor_tile_2: Handle<Image>,
    floor_tile_3: Handle<Image>,
    floor_tile_4: Handle<Image>,
    floor_tile_5: Handle<Image>,
    floor_tile_6: Handle<Image>,
    floor_tile_7: Handle<Image>,
    floor_tile_8: Handle<Image>,
    //
    wall_tile_1111: Handle<Image>,
    wall_tile_0000: Handle<Image>,
    wall_tile_1000: Handle<Image>,
    wall_tile_0100: Handle<Image>,
    wall_tile_0010: Handle<Image>,
    wall_tile_0001: Handle<Image>,
    wall_tile_1100: Handle<Image>,
    wall_tile_1010: Handle<Image>,
    wall_tile_0110: Handle<Image>,
    wall_tile_0101: Handle<Image>,
    wall_tile_0011: Handle<Image>,
    wall_tile_1001: Handle<Image>,
    wall_tile_1110: Handle<Image>,
    wall_tile_0111: Handle<Image>,
    wall_tile_1011: Handle<Image>,
    wall_tile_1101: Handle<Image>,
}

impl GridTextures {
    pub(crate) fn random_floor_tile(&self, position: &Position) -> Handle<Image> {
        match (position.0 .0 * position.0 .1) % 9 {
            0 => self.floor_tile_0.clone(),
            1 => self.floor_tile_1.clone(),
            2 => self.floor_tile_2.clone(),
            3 => self.floor_tile_3.clone(),
            4 => self.floor_tile_4.clone(),
            5 => self.floor_tile_5.clone(),
            6 => self.floor_tile_6.clone(),
            7 => self.floor_tile_7.clone(),
            _ => self.floor_tile_8.clone(),
        }
    }

    pub(crate) fn resolve_wall_tile(
        &self,
        left: &Option<Coordinates>,
        top: &Option<Coordinates>,
        right: &Option<Coordinates>,
        bottom: &Option<Coordinates>,
    ) -> Handle<Image> {
        match (
            left.is_some(),
            top.is_some(),
            right.is_some(),
            bottom.is_some(),
        ) {
            (false, false, false, false) => self.wall_tile_0000.clone(),
            (true, false, false, false) => self.wall_tile_1000.clone(),
            (false, true, false, false) => self.wall_tile_0100.clone(),
            (false, false, true, false) => self.wall_tile_0010.clone(),
            (false, false, false, true) => self.wall_tile_0001.clone(),
            (true, true, false, false) => self.wall_tile_1100.clone(),
            (false, true, true, false) => self.wall_tile_0110.clone(),
            (false, false, true, true) => self.wall_tile_0011.clone(),
            (true, false, false, true) => self.wall_tile_1001.clone(),
            (true, false, true, false) => self.wall_tile_1010.clone(),
            (false, true, false, true) => self.wall_tile_0101.clone(),
            (true, true, true, false) => self.wall_tile_1110.clone(),
            (false, true, true, true) => self.wall_tile_0111.clone(),
            (true, false, true, true) => self.wall_tile_1011.clone(),
            (true, true, false, true) => self.wall_tile_1101.clone(),
            (true, true, true, true) => self.wall_tile_1111.clone(),
        }
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GridTextures {
        floor_tile_0: asset_server.load("floor_0.png"),
        floor_tile_1: asset_server.load("floor_1.png"),
        floor_tile_2: asset_server.load("floor_2.png"),
        floor_tile_3: asset_server.load("floor_3.png"),
        floor_tile_4: asset_server.load("floor_4.png"),
        floor_tile_5: asset_server.load("floor_5.png"),
        floor_tile_6: asset_server.load("floor_6.png"),
        floor_tile_7: asset_server.load("floor_7.png"),
        floor_tile_8: asset_server.load("floor_8.png"),
        //
        wall_tile_0000: asset_server.load("wall_0000.png"),
        wall_tile_1000: asset_server.load("wall_1000.png"),
        wall_tile_0100: asset_server.load("wall_0100.png"),
        wall_tile_0010: asset_server.load("wall_0010.png"),
        wall_tile_0001: asset_server.load("wall_0001.png"),
        wall_tile_1100: asset_server.load("wall_1100.png"),
        wall_tile_1010: asset_server.load("wall_1010.png"),
        wall_tile_0110: asset_server.load("wall_0110.png"),
        wall_tile_0101: asset_server.load("wall_0101.png"),
        wall_tile_0011: asset_server.load("wall_0011.png"),
        wall_tile_1001: asset_server.load("wall_1001.png"),
        wall_tile_1110: asset_server.load("wall_1110.png"),
        wall_tile_0111: asset_server.load("wall_0111.png"),
        wall_tile_1011: asset_server.load("wall_1011.png"),
        wall_tile_1101: asset_server.load("wall_1101.png"),
        wall_tile_1111: asset_server.load("wall_1111.png"),
    });
}
