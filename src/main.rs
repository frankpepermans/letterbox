use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use letterbox::{
    game::coordinates::Coordinates,
    plugin::{assets::AssetsPlugin, enemy::EnemyPlugin, grid::GridPlugin, player::PlayerPlugin},
    EnemyCount, EnemySprites, GridSize, NodeSize, PlayerSprites,
};

// (rows, cols)
const GRID_SIZE: (usize, usize) = (64, 64);
// (width, height) in pixels
const NODE_SIZE: (f32, f32) = (32., 32.);

fn main() {
    App::new()
        .insert_resource(GridSize(GRID_SIZE))
        .insert_resource(NodeSize(NODE_SIZE))
        .insert_resource(EnemyCount(200))
        .add_startup_system(setup_system)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1024.,
                        height: 768.,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    },

                    ..default()
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AssetsPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
        },
        ..Default::default()
    });

    commands.insert_resource(PlayerSprites::init(&asset_server, &mut texture_atlases));
    commands.insert_resource(EnemySprites::init(&asset_server, &mut texture_atlases));
}

#[derive(Component)]
struct Player(Coordinates);
