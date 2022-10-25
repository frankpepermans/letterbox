use bevy::prelude::*;

use crate::{
    game::node::Node,
    game::{matrix::Matrix, node::Entry},
    GridSize, NodeSize, Position, UserCursorPressedState, UserPosition,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(node_system)
            .add_system(layout_grid_system)
            .add_system(render_grid_system.after(layout_grid_system))
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

fn setup_system(mut commands: Commands, size: Res<GridSize>) {
    let rows = size.0 .0;
    let cols = size.0 .1;
    let mut m = Matrix::new(rows, cols, Node::open());

    prepare_grid(&mut m);

    commands.spawn().insert(m).insert(UserPosition {
        coordinates: None,
        cursor_pressed_state: None,
        target_modification: None,
    });
}

fn node_system(
    mut commands: Commands,
    size: Res<GridSize>,
    node_size: Res<NodeSize>,
    query: Query<&Matrix<Node>, Added<Matrix<Node>>>,
) {
    for matrix in &query {
        let cols = size.0 .1;

        matrix.vec.iter().enumerate().for_each(|(index, node)| {
            let row = index / cols;
            let col = index % cols;

            commands
                .spawn()
                .insert(*node)
                .insert(Position((row, col)))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(node_size.0 .0, node_size.0 .1)),
                        ..default()
                    },
                    ..default()
                });
        });
    }
}

fn layout_grid_system(
    window: Res<WindowDescriptor>,
    node_size: Res<NodeSize>,
    mut query: Query<(&Position, &mut Transform), Or<(Changed<Node>, Changed<Position>)>>,
) {
    for (position, mut transform) in &mut query {
        transform.translation.x =
            -window.width / 2. + position.0 .1 as f32 * node_size.0 .0 + node_size.0 .0 / 2.;
        transform.translation.y =
            window.height / 2. - position.0 .0 as f32 * node_size.0 .1 - node_size.0 .1 / 2.;
    }
}

fn render_grid_system(
    pos_query: Query<&UserPosition>,
    mut query: Query<(&Node, &Position, &mut Sprite)>,
) {
    for user_position in &pos_query {
        for (node, position, mut sprite) in &mut query {
            if user_position.coordinates == Some(position.0) {
                sprite.color = match node[Entry::LEFT] {
                    true => Color::rgb(0.25, 0.5, 0.75),
                    false => Color::rgb(0.75, 0.5, 0.25),
                };
            } else {
                sprite.color = match node[Entry::LEFT] {
                    true => Color::rgb(0.25, 0.25, 0.75),
                    false => Color::rgb(0.75, 0.25, 0.25),
                };
            }
        }
    }
}

fn update_user_position_coordinates_system(
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    mut query: Query<(&mut Matrix<Node>, &mut UserPosition)>,
) {
    if let Some(window) = windows.get_primary() {
        let h = window.height();

        if let Some(pos) = window.cursor_position() {
            let row = (h - pos[1]) / node_size.0 .0;
            let col = pos[0] / node_size.0 .1;
            let coordinates = (row.floor() as usize, col.floor() as usize);

            for (matrix, mut user_position) in &mut query {
                let mut val = user_position.coordinates;

                if matrix.contains(coordinates) {
                    val = Some(coordinates);
                }

                if user_position.coordinates != val {
                    *user_position = UserPosition {
                        coordinates: val,
                        cursor_pressed_state: user_position.cursor_pressed_state,
                        target_modification: user_position.target_modification,
                    };
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
    mut query: Query<(&mut Matrix<Node>, &mut UserPosition), Changed<UserPosition>>,
) {
    for (mut matrix, mut user_position) in &mut query {
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

fn prepare_grid(m: &mut Matrix<Node>) {
    m[(2, 2)] = Node::closed();
    m[(3, 2)] = Node::closed();
    m[(4, 2)] = Node::closed();
    m[(5, 2)] = Node::closed();
    m[(7, 2)] = Node::closed();

    m[(0, 4)] = Node::closed();
    m[(1, 4)] = Node::closed();
    m[(2, 4)] = Node::closed();
    m[(3, 4)] = Node::closed();
    m[(4, 4)] = Node::closed();

    m[(6, 4)] = Node::closed();
    m[(7, 4)] = Node::closed();
    m[(9, 4)] = Node::closed();
    m[(10, 4)] = Node::closed();
    m[(11, 4)] = Node::closed();

    m[(2, 6)] = Node::closed();
    m[(3, 6)] = Node::closed();
    m[(4, 6)] = Node::closed();
    m[(5, 6)] = Node::closed();
    m[(6, 6)] = Node::closed();
    m[(7, 6)] = Node::closed();
    m[(9, 6)] = Node::closed();

    m[(4, 5)] = Node::closed();
    m[(4, 6)] = Node::closed();
    m[(4, 7)] = Node::closed();
    m[(4, 8)] = Node::closed();
    m[(4, 9)] = Node::closed();
    m[(4, 10)] = Node::closed();
    m[(4, 11)] = Node::closed();

    m[(0, 14)] = Node::closed();
    m[(1, 14)] = Node::closed();
    m[(2, 14)] = Node::closed();
    m[(3, 14)] = Node::closed();
    m[(4, 14)] = Node::closed();
}
