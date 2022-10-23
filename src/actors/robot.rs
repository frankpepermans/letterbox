use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game::{
        astar::{manhattan_heuristic, AStar},
        node::Node,
    },
    game::{coordinates::Coordinates, matrix::Matrix},
    NodeSize, RobotCount,
};

#[derive(Component)]
struct StartPosition(Coordinates);

#[derive(Component)]
struct EndPosition(Coordinates);

#[derive(Component)]
struct Path(Option<Vec<Coordinates>>);

#[derive(Component)]
struct TraversalIndex(Option<usize>);

#[derive(Bundle)]
struct PathInstructionsBundle {
    start_position: StartPosition,
    end_position: EndPosition,
}

#[derive(Bundle)]
struct PathBundle {
    path: Path,
    traversal_index: TraversalIndex,
}

#[derive(Component)]
pub struct AnimationSequence {
    pub snap: Option<Duration>,
    pub duration: Duration,
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(calc_path)
            .add_system(increment_path_traversal)
            .add_system(traverse_path);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, node_size: Res<NodeSize>, robot_count: Res<RobotCount>) {
    (0..robot_count.0).for_each(|index| {
        commands
            .spawn()
            .insert_bundle(PathInstructionsBundle {
                start_position: StartPosition((index as usize, 0)),
                end_position: EndPosition((index as usize, 49)),
            })
            .insert_bundle(PathBundle {
                path: Path(None),
                traversal_index: TraversalIndex(None),
            })
            .insert(AnimationSequence {
                duration: Duration::from_millis(60),
                snap: None,
            })
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(node_size.0 .0, node_size.0 .1)),
                    ..default()
                },
                ..default()
            });
    });
}

fn calc_path(
    m_query: Query<(&Matrix<Node>,), Changed<Matrix<Node>>>,
    mut query: Query<
        (&StartPosition, &EndPosition, &mut Path, &mut TraversalIndex),
        With<AnimationSequence>,
    >,
) {
    for (matrix,) in &m_query {
        for (start_position, end_position, mut path, mut traversal_index) in &mut query {
            let p = matrix.astar(start_position.0, end_position.0, &manhattan_heuristic);

            *path = Path(p);

            if traversal_index.0.is_none() {
                *traversal_index = TraversalIndex(Some(0));
            }
        }
    }
}

fn increment_path_traversal(
    time: Res<Time>,
    mut query: Query<(&Path, &mut TraversalIndex, &mut AnimationSequence), Changed<TraversalIndex>>,
) {
    for (path, mut traversal_index, mut animation_sequence) in &mut query {
        if let (Some(path), Some(index)) = (&path.0, traversal_index.0) {
            if index < path.len() - 1 {
                *animation_sequence = AnimationSequence {
                    duration: animation_sequence.duration,
                    snap: Some(time.time_since_startup()),
                };
            } else {
                *traversal_index = TraversalIndex(Some(0));
            }
        }
    }
}

fn traverse_path(
    time: Res<Time>,
    window: Res<WindowDescriptor>,
    node_size: Res<NodeSize>,
    mut query: Query<(
        &Path,
        &AnimationSequence,
        &mut Transform,
        &mut TraversalIndex,
    )>,
) {
    for (path, animation_sequence, mut transform, mut traversal_index) in &mut query {
        let params = (&path.0, traversal_index.0, animation_sequence.snap);

        if let (Some(path), Some(index), Some(start_duration)) = params {
            if index < path.len() - 1 {
                let delta = time.time_since_startup() - start_duration;
                let mut delta_factor =
                    delta.as_millis() as f32 / animation_sequence.duration.as_millis() as f32;
                let at_end = delta_factor > 1.;

                delta_factor = delta_factor.clamp(0., 1.);

                let from = path[index];
                let to = path[index + 1];
                let row_0 = from.0 as f32;
                let row_1 = to.0 as f32;
                let col_0 = from.1 as f32;
                let col_1 = to.1 as f32;
                let position = (
                    row_0 + (row_1 - row_0) * delta_factor,
                    col_0 + (col_1 - col_0) * delta_factor,
                );
                let (w, h) = (window.width, window.height);

                transform.translation.x =
                    -w / 2. + position.1 as f32 * node_size.0 .0 + node_size.0 .0 / 2.;
                transform.translation.y =
                    h / 2. - position.0 as f32 * node_size.0 .1 - node_size.0 .1 / 2.;
                transform.translation.z = 100.;

                if at_end {
                    *traversal_index = TraversalIndex(Some(index + 1));
                }
            }
        }
    }
}
