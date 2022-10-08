use std::sync::Arc;

use letterbox::{
    game::{
        astar::{manhattan_heuristic, AStar, EncodedMatrix},
        debug_image::DebugImage,
        node::Node,
    },
    Matrix,
};
use tokio::{join, spawn};

#[tokio::main]
async fn main() {
    let mut m = Matrix::new(24, 240, Node::open());

    m[(0, 4)] = Node::closed();
    m[(1, 4)] = Node::closed();
    m[(2, 4)] = Node::closed();
    m[(3, 4)] = Node::closed();
    m[(4, 4)] = Node::closed();

    m[(2, 6)] = Node::closed();
    m[(3, 6)] = Node::closed();
    m[(4, 6)] = Node::closed();
    m[(5, 6)] = Node::closed();
    m[(6, 6)] = Node::closed();
    m[(7, 6)] = Node::closed();
    m[(9, 6)] = Node::closed();

    let enc: EncodedMatrix = m.clone().into();

    enc.to_file("test.lb").unwrap();

    let mut m_ff: Matrix<Node> = EncodedMatrix::from_file("test.lb").into();

    m_ff.entangle((16, 0), (5, 7));

    let m = Arc::new(m_ff);
    let m_1 = m.clone();
    let m_2 = m.clone();

    let task_1 = spawn(async move { m_1.astar((5, 239), (0, 0), &manhattan_heuristic) });

    let task_2 = spawn(async move { m_2.astar((0, 0), (9, 9), &manhattan_heuristic) });

    let (a, b) = join!(task_1, task_2);

    println!("b {:?}", b.unwrap());

    println!("import {:?}", Node::from(0b1001));

    let n: Node = 0b1001.into();

    println!("max {:?}", n);

    if let Ok(result) = a {
        if let Some(path) = result {
            println!("size! {}", path.len());
            m.debug_image("test.png", path);
        }
    }
}
