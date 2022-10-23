use bevy::prelude::*;

use crate::{
    game::node::Node,
    game::{matrix::Matrix, node::Entry},
    GridSize, Position,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(node_system)
            .add_system(render_grid_system)
            .add_system(input_system);
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

    commands.spawn().insert(m);
}

fn node_system(
    mut commands: Commands,
    size: Res<GridSize>,
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
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..default()
                    },
                    ..default()
                });
        });
    }
}

fn render_grid_system(
    window: Res<WindowDescriptor>,
    mut query: Query<
        (&Node, &Position, &mut Transform, &mut Sprite),
        Or<(Changed<Node>, Changed<Position>)>,
    >,
) {
    for (node, position, mut transform, mut sprite) in &mut query {
        let (w, h) = (window.width, window.height);

        transform.translation.x = -w / 2. + position.0 .1 as f32 * 20.0;
        transform.translation.y = h / 2. - position.0 .0 as f32 * 20.0;

        sprite.color = match node[Entry::LEFT] {
            true => Color::rgb(0.25, 0.25, 0.75),
            false => Color::rgb(0.75, 0.25, 0.25),
        };
    }
}

fn input_system(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut lookup_query: Query<(&mut Node, &Position)>,
    mut query: Query<(&mut Matrix<Node>,)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            let (_w, h) = (window.width(), window.height());

            if let Some(pos) = window.cursor_position() {
                let row = (h - pos[1] + 10.) / 20.;
                let col = (pos[0] + 10.) / 20.;
                let coordinates = (row.floor() as usize, col.floor() as usize);

                for (mut matrix,) in &mut query {
                    if matrix[coordinates].left {
                        matrix[coordinates] = Node::closed();
                    } else {
                        matrix[coordinates] = Node::open();
                    }

                    for (mut node, position) in &mut lookup_query {
                        if position.0 .0 == coordinates.0 && position.0 .1 == coordinates.1 {
                            *node = matrix[coordinates];
                        }
                    }
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
