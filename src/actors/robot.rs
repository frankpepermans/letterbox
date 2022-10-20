use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game::matrix::Matrix,
    game::{
        astar::{manhattan_heuristic, AStar},
        node::Node,
    },
    AnimationSequence, AnimationValue, EndPosition, Path, PathPosition, PathTraversalIndex, Robot,
    RobotCount, RobotMappingBundle, SinglePathVector, StartPosition, Velocity,
};

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(setup_path)
            .add_system(calc_path)
            .add_system(increment_path_traversal)
            .add_system(render_path)
            .add_system(traverse_path);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, robot_count: Res<RobotCount>) {
    (0..robot_count.0).for_each(|index| {
        commands
            .spawn()
            .insert(Robot)
            .insert_bundle(RobotMappingBundle {
                start_position: StartPosition((index as usize, 0)),
                end_position: EndPosition((3, 5)),
                path: Path(None),
            })
            .insert(SinglePathVector {
                from: StartPosition((index as usize, 0)),
                to: EndPosition((index as usize, 100)),
            })
            .insert(AnimationSequence {
                duration: Duration::from_millis(200),
                range_values: (AnimationValue(0.), AnimationValue(1.)),
                velocity: Velocity(0.5),
                snap: None,
            })
            .insert(PathTraversalIndex(0))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                ..default()
            });
    });
}

fn calc_path(
    m_query: Query<(&Matrix<Node>,)>,
    mut query: Query<
        (
            &StartPosition,
            &EndPosition,
            &mut Path,
            &mut PathTraversalIndex,
        ),
        Or<(Added<Robot>, Changed<StartPosition>, Changed<EndPosition>)>,
    >,
) {
    for (matrix,) in &m_query {
        for (start, goal, mut path, mut path_traversal_index) in &mut query {
            let p = matrix.astar(start.0, goal.0, &manhattan_heuristic);

            *path = Path(p.clone());

            if let Some(p) = p {
                *path_traversal_index = PathTraversalIndex(0);
            }
        }
    }
}

fn increment_path_traversal(
    time: Res<Time>,
    mut query: Query<
        (
            &Path,
            &PathTraversalIndex,
            &mut AnimationSequence,
            &mut SinglePathVector,
        ),
        Or<(Changed<PathTraversalIndex>,)>,
    >,
) {
    for (path, path_traversal_index, mut animation_sequence, mut single_path_vector) in &mut query {
        if let Some(path) = &path.0 {
            if path_traversal_index.0 < path.len() - 1 {
                *animation_sequence = AnimationSequence {
                    duration: Duration::from_millis(200),
                    range_values: (AnimationValue(0.), AnimationValue(1.)),
                    velocity: Velocity(0.5),
                    snap: Some(time.time_since_startup()),
                };
                *single_path_vector = SinglePathVector {
                    from: StartPosition(path[path_traversal_index.0].clone()),
                    to: EndPosition(path[path_traversal_index.0 + 1].clone()),
                };
            }
        }
    }
}

fn setup_path(mut commands: Commands, query: Query<(&Path,), Or<(Changed<Path>,)>>) {
    for (path,) in &query {
        if let Some(path) = path.0.clone() {
            for coordinates in path {
                commands
                    .spawn()
                    .insert(PathPosition(coordinates))
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        ..default()
                    });
            }
        }
    }
}

fn render_path(
    window: Res<WindowDescriptor>,
    mut query: Query<(&PathPosition, &mut Transform, &mut Sprite), Or<(Changed<PathPosition>,)>>,
) {
    for (position, mut transform, mut sprite) in &mut query {
        let (w, h) = (window.width, window.height);

        transform.translation.x = -w / 2. + position.0 .1 as f32 * 20.0;
        transform.translation.y = h / 2. - position.0 .0 as f32 * 20.0;
        transform.translation.z = 10.;

        sprite.color = Color::rgba(0.25, 0.75, 0.25, 0.1,);
    }
}

fn traverse_path(
    time: Res<Time>,
    window: Res<WindowDescriptor>,
    mut query: Query<(
        &SinglePathVector,
        &AnimationSequence,
        &mut Transform,
        &mut PathTraversalIndex,
    )>,
) {
    for (single_path_vector, animation_sequence, mut transform, mut path_traversal_index) in
        &mut query
    {
        if let Some(start_duration) = animation_sequence.snap {
            let delta = time.time_since_startup() - start_duration;
            let delta_factor =
                delta.as_millis() as f32 / animation_sequence.duration.as_millis() as f32;

            if delta_factor <= 1.0 {
                let row_0 = single_path_vector.from.0 .0 as f32;
                let row_1 = single_path_vector.to.0 .0 as f32;
                let col_0 = single_path_vector.from.0 .1 as f32;
                let col_1 = single_path_vector.to.0 .1 as f32;
                let position = (
                    row_0 + (row_1 - row_0) * delta_factor,
                    col_0 + (col_1 - col_0) * delta_factor,
                );
                let (w, h) = (window.width, window.height);

                transform.translation.x = -w / 2. + position.1 as f32 * 20.0;
                transform.translation.y = h / 2. - position.0 as f32 * 20.0;
                transform.translation.z = 100.;
            } else {
                *path_traversal_index = PathTraversalIndex(path_traversal_index.0 + 1);
            }
        }
    }
}
