use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use letterbox::game::{
    astar::{manhattan_heuristic, AStar},
    matrix::Matrix,
    node::{Entry, Node},
};

fn astar(s: usize) -> Option<Vec<(usize, usize)>> {
    let mut m = Matrix::new(s, s, Node::open());

    m[(1, 9)][Entry::LEFT] = false;
    m[(1, 9)][Entry::TOP] = false;

    m.astar(
        (0, 0),
        (s - 1, s - 1),
        &manhattan_heuristic,
        &HashMap::new(),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("astar 100", |b| b.iter(|| astar(black_box(100))));
    c.bench_function("astar 1000", |b| b.iter(|| astar(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
