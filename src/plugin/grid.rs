use bevy::prelude::*;
use mapgen::{
    AreaStartingPosition, CellularAutomata, CullUnreachable, DistantExit, MapBuilder,
    NoiseGenerator, XStart, YStart,
};
use rand::prelude::*;

use crate::{
    game::{coordinates::Coordinates, matrix::Matrix, node::Entry},
    game::{movement::Movement, node::Node},
    GridSize, LivePosition, NodeSize, PlayerPosition, Position, UserCursorPressedState,
    UserPosition,
};

use super::assets::GridTextures;

#[derive(Resource)]
pub struct OpenNodes(pub Vec<Coordinates>);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(layout_grid_system)
            .add_system(render_grid_system.after(layout_grid_system))
            .add_system(render_user_position_system.after(layout_grid_system))
            .add_system(update_user_position_coordinates_system)
            .add_system(update_user_position_cursor_pressed_system)
            .add_system(
                modify_single_node_system.after(update_user_position_cursor_pressed_system),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, size: Res<GridSize>, node_size: Res<NodeSize>) {
    let rows = size.0 .0;
    let cols = size.0 .1;
    let mut m = Matrix::new(rows, cols, Node::open());
    let open_nodes = prepare_grid(&size, &mut m);

    commands.insert_resource(OpenNodes(open_nodes));

    commands
        .spawn(UserPosition {
            coordinates: None,
            cursor_pressed_state: None,
            target_modification: None,
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 1.0, 0.2, 0.5),
                custom_size: Some(Vec2::new(node_size.0 .0, node_size.0 .1)),
                ..default()
            },
            ..default()
        });

    let cols = size.0 .1;

    m.vec.iter().enumerate().for_each(|(index, node)| {
        let row = index / cols;
        let col = index % cols;

        commands
            .spawn_empty()
            .insert(*node)
            .insert(Position((row, col)))
            .insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(node_size.0 .0, node_size.0 .1)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 99.,
                    },
                    ..default()
                },
                ..default()
            });
    });

    commands.insert_resource(m);
}

fn layout_grid_system(
    node_size: Res<NodeSize>,
    mut query: Query<(&Position, &mut Transform)>,
    p_query: Query<&LivePosition, Or<(Changed<PlayerPosition>, Changed<LivePosition>)>>,
) {
    for live_position in &p_query {
        for (position, mut transform) in &mut query {
            transform.translation.x =
                (position.0 .1 as f32 - live_position.0 .1) * node_size.0 .0 + node_size.0 .0 / 2.;
            transform.translation.y =
                (live_position.0 .0 - position.0 .0 as f32) * node_size.0 .1 - node_size.0 .1 / 2.;
        }
    }
}

fn render_grid_system(
    grid_textures: Res<GridTextures>,
    matrix: Res<Matrix<Node>>,
    mut query: Query<(&Node, &Position, &mut Handle<Image>), Changed<Node>>,
) {
    for (node, position, mut handle) in &mut query {
        *handle = match node[Entry::LEFT] {
            true => grid_textures.random_floor_tile(&position),
            false => {
                let left = matrix.left(&position.0);
                let top = matrix.up(&position.0);
                let right = matrix.right(&position.0);
                let bottom = matrix.down(&position.0);

                grid_textures.resolve_wall_tile(&left, &top, &right, &bottom)
            }
        };
    }
}

fn render_user_position_system(
    node_size: Res<NodeSize>,
    mut pos_query: Query<(&UserPosition, &mut Transform), Changed<UserPosition>>,
    query: Query<&Position, With<Node>>,
    p_query: Query<&LivePosition, Or<(Changed<PlayerPosition>, Changed<LivePosition>)>>,
) {
    for (user_position, mut transform) in &mut pos_query {
        for position in &query {
            if user_position.coordinates == Some(position.0) {
                for live_position in &p_query {
                    transform.translation.x = (position.0 .1 as f32 - live_position.0 .1)
                        * node_size.0 .0
                        + node_size.0 .0 / 2.;
                    transform.translation.y = (live_position.0 .0 - position.0 .0 as f32)
                        * node_size.0 .1
                        - node_size.0 .1 / 2.;
                    transform.translation.z = 100.;
                }
            }
        }
    }
}

fn update_user_position_coordinates_system(
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    matrix: Res<Matrix<Node>>,
    mut query: Query<&mut UserPosition>,
    n_query: Query<(&Position, &Transform), With<Node>>,
) {
    if let Some(window) = windows.get_primary() {
        let w = window.width() / 2.;
        let h = window.height() / 2.;

        if let Some(pos) = window.cursor_position() {
            for (position, transform) in &n_query {
                if pos.x - w >= transform.translation.x - node_size.0 .0 / 2.
                    && pos.x - w < transform.translation.x + node_size.0 .1 - node_size.0 .0 / 2.
                    && pos.y - h >= transform.translation.y - node_size.0 .1 / 2.
                    && pos.y - h < transform.translation.y + node_size.0 .0 - node_size.0 .1 / 2.
                {
                    for mut user_position in &mut query {
                        let mut val = user_position.coordinates;

                        if matrix.contains(position.0) {
                            val = Some(position.0);
                        }

                        if user_position.coordinates != val {
                            *user_position = UserPosition {
                                coordinates: val,
                                cursor_pressed_state: user_position.cursor_pressed_state,
                                target_modification: user_position.target_modification,
                            };
                        }

                        return;
                    }
                }
            }
        }
    }
}

fn update_user_position_cursor_pressed_system(
    mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut UserPosition>,
) {
    for mut user_position in &mut query {
        if user_position.coordinates.is_some() {
            let mut cursor_pressed_state = user_position.cursor_pressed_state;
            let mut target_modification = user_position.target_modification;

            if mouse.just_pressed(MouseButton::Left) {
                cursor_pressed_state = Some(UserCursorPressedState::DOWN);
            } else if mouse.just_released(MouseButton::Left) {
                cursor_pressed_state = Some(UserCursorPressedState::UP);
                target_modification = None;
            }

            if user_position.cursor_pressed_state != cursor_pressed_state {
                *user_position = UserPosition {
                    coordinates: user_position.coordinates,
                    cursor_pressed_state: cursor_pressed_state,
                    target_modification: target_modification,
                };
            }
        }
    }
}

fn modify_single_node_system(
    mut lookup_query: Query<(&mut Node, &Position)>,
    mut query: Query<&mut UserPosition, Changed<UserPosition>>,
    mut matrix: ResMut<Matrix<Node>>,
) {
    for mut user_position in &mut query {
        if let (Some(coordinates), Some(cursor_pressed_state)) = (
            user_position.coordinates,
            user_position.cursor_pressed_state,
        ) {
            if cursor_pressed_state == UserCursorPressedState::DOWN && matrix.contains(coordinates)
            {
                let target_modification =
                    user_position
                        .target_modification
                        .unwrap_or(if matrix[coordinates].left {
                            Node::closed()
                        } else {
                            Node::open()
                        });

                matrix[coordinates] = target_modification;

                for (mut node, position) in &mut lookup_query {
                    if position.0 == coordinates {
                        *node = matrix[coordinates];
                    } else if coordinates.0 > 0 && position.0 == (coordinates.0 - 1, coordinates.1)
                    {
                        *node = node.clone();
                    } else if position.0 == (coordinates.0 + 1, coordinates.1) {
                        *node = node.clone();
                    } else if coordinates.1 > 0 && position.0 == (coordinates.0, coordinates.1 - 1)
                    {
                        *node = node.clone();
                    } else if position.0 == (coordinates.0, coordinates.1 + 1) {
                        *node = node.clone();
                    }
                }

                if user_position.target_modification != Some(target_modification) {
                    *user_position = UserPosition {
                        coordinates: user_position.coordinates,
                        cursor_pressed_state: user_position.cursor_pressed_state,
                        target_modification: Some(target_modification),
                    };
                }
            }
        }
    }
}

// see https://github.com/klangner/mapgen.rs/blob/master/demo/src/lib.rs
fn prepare_grid(size: &Res<GridSize>, m: &mut Matrix<Node>) -> Vec<Coordinates> {
    let mut t_rng = rand::thread_rng();
    let mut rng = StdRng::seed_from_u64(t_rng.gen());
    let rows = size.0 .0;
    let cols = size.0 .1;
    let map = MapBuilder::new(rows, cols)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::LEFT, YStart::BOTTOM))
        .with(CullUnreachable::new())
        .with(DistantExit::new())
        .build_with_rng(&mut rng);
    let mut open_nodes = Vec::new();
    let mut row = 0;
    let mut col = 0;

    map.tiles.into_iter().for_each(|it| {
        let coordinates = (row, col);

        if it.is_blocked() {
            m[coordinates] = Node::closed();
        } else {
            open_nodes.push(coordinates);
        }

        if col + 1 < m.cols {
            col += 1;
        } else {
            row += 1;
            col = 0;
        }
    });

    open_nodes
}
