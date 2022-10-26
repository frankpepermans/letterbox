use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    game::{
        astar::{manhattan_heuristic, AStar},
        node::{Entry, Node},
    },
    game::{coordinates::Coordinates, matrix::Matrix},
    NodeSize, Position, RobotCount,
};

#[derive(Component)]
struct StartPosition(Coordinates);

#[derive(Component)]
struct EndPosition(Coordinates);

#[derive(Component)]
struct Path(Option<Vec<Coordinates>>);

#[derive(Component)]
struct DefaultPath(Option<Vec<Coordinates>>);

#[derive(Component)]
struct TraversalIndex(Option<usize>);

#[derive(Bundle)]
struct PathInstructionsBundle {
    start_position: StartPosition,
    end_position: EndPosition,
    current_position: Position,
}

#[derive(Bundle)]
struct PathBundle {
    path: Path,
    default_path: DefaultPath,
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
            .add_system(traverse_path.after(calc_path))
            .add_system(increment_path_traversal.after(traverse_path));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands, node_size: Res<NodeSize>, robot_count: Res<RobotCount>) {
    let mut rng = rand::thread_rng();

    (0..robot_count.0).for_each(|_| {
        let index = (rng.gen::<f32>() * 24.) as usize;

        commands
            .spawn()
            .insert_bundle(PathInstructionsBundle {
                start_position: StartPosition((index, 0)),
                end_position: EndPosition((index, 49)),
                current_position: Position((index, 0)),
            })
            .insert_bundle(PathBundle {
                path: Path(None),
                default_path: DefaultPath(None),
                traversal_index: TraversalIndex(None),
            })
            .insert(AnimationSequence {
                duration: Duration::from_millis(25 + (rng.gen::<f32>() * 200.) as u64),
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
    m_query: Query<&Matrix<Node>, Changed<Matrix<Node>>>,
    n_query: Query<(&Position, &Node), Changed<Node>>,
    mut query: Query<
        (
            &StartPosition,
            &Position,
            &EndPosition,
            &mut Path,
            &mut DefaultPath,
            &mut TraversalIndex,
        ),
        With<AnimationSequence>,
    >,
) {
    for matrix in &m_query {
        for (
            start_position,
            current_position,
            end_position,
            mut path,
            mut default_path,
            mut traversal_index,
        ) in &mut query
        {
            let mut affects_path = false;
            let no_path = path.0.is_none() || traversal_index.0.is_none();

            for (position, node) in &n_query {
                if node[Entry::LEFT] {
                    break;
                }

                let p_1 = &default_path.0.to_owned().unwrap_or_default();
                let p_2 = &path.0.to_owned().unwrap_or_default();

                if p_1.contains(&position.0.to_owned()) || p_2.contains(&position.0.to_owned()) {
                    affects_path = true;

                    break;
                }
            }

            if affects_path || no_path {
                let is_traversing = start_position.0 != current_position.0;

                *default_path = DefaultPath(matrix.astar(
                    start_position.0,
                    end_position.0,
                    &manhattan_heuristic,
                ));

                if is_traversing {
                    *path = Path(default_path.0.to_owned());
                    *traversal_index = TraversalIndex(
                        default_path
                            .0
                            .to_owned()
                            .unwrap_or_default()
                            .iter()
                            .position(|it| it == &current_position.0),
                    );

                    if path.0.is_none() || traversal_index.0.is_none() {
                        *path = Path(matrix.astar(
                            current_position.0,
                            end_position.0,
                            &manhattan_heuristic,
                        ));
                        *traversal_index = TraversalIndex(Some(0));
                    }
                } else if no_path {
                    *path = Path(default_path.0.to_owned());
                    *traversal_index = TraversalIndex(Some(0));
                }
            }
        }
    }
}

fn increment_path_traversal(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Path,
            &mut DefaultPath,
            &mut TraversalIndex,
            &mut AnimationSequence,
            &mut Position,
            &mut StartPosition,
            &mut EndPosition,
        ),
        Or<(Changed<TraversalIndex>, Changed<Path>)>,
    >,
) {
    for (
        mut path,
        mut default_path,
        mut traversal_index,
        mut animation_sequence,
        mut current_position,
        mut start_position,
        mut end_position,
    ) in &mut query
    {
        if let (Some(p), Some(index)) = (&path.0, traversal_index.0) {
            if index < p.len() - 1 {
                let did_pos_change = current_position.0 != p[index];

                if did_pos_change {
                    *current_position = Position(p[index]);
                }

                let at_end = did_pos_change
                    || match animation_sequence.snap {
                        Some(snap) => {
                            (time.time_since_startup() - snap).as_millis()
                                >= animation_sequence.duration.as_millis()
                        }
                        None => true,
                    };

                if at_end {
                    *animation_sequence = AnimationSequence {
                        duration: animation_sequence.duration,
                        snap: Some(time.time_since_startup()),
                    };
                }
            } else {
                let tmp = start_position.0;
                let inverted_default_path = match default_path.0.to_owned() {
                    Some(path) => Some(path.into_iter().rev().collect::<Vec<_>>()),
                    _ => None,
                };

                *start_position = StartPosition(end_position.0);
                *end_position = EndPosition(tmp);
                *path = Path(inverted_default_path.to_owned());
                *default_path = DefaultPath(inverted_default_path.to_owned());
                *current_position = Position(start_position.0);
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
