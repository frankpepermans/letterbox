use crate::Matrix;

use super::coordinates::{Coordinates, CreateCoordinates};
use super::node::Node;

pub trait Movement {
    fn nearest_neighbours(&self, index: Coordinates) -> Vec<Option<Coordinates>>;
    fn left(&self, index: Coordinates) -> Option<Coordinates>;
    fn up(&self, index: Coordinates) -> Option<Coordinates>;
    fn right(&self, index: Coordinates) -> Option<Coordinates>;
    fn down(&self, index: Coordinates) -> Option<Coordinates>;
}

impl Movement for Matrix<Node> {
    #[inline(always)]
    fn nearest_neighbours(&self, index: Coordinates) -> Vec<Option<Coordinates>> {
        let mut v = self
            .entanglements
            .iter()
            .filter(|e| e.0 == index || e.1 == index)
            .map(|e| if index == e.0 { Some(e.1) } else { Some(e.0) })
            .collect::<Vec<Option<(usize, usize)>>>();

        v.push(self.left(index));
        v.push(self.up(index));
        v.push(self.right(index));
        v.push(self.down(index));

        v
    }

    #[inline(always)]
    fn left(&self, index: Coordinates) -> Option<Coordinates> {
        if index.col() > 0 {
            let index = (index.row(), index.col() - 1);
            let node = &self[index];

            node.right.then_some(index)
        } else {
            None
        }
    }

    #[inline(always)]
    fn up(&self, index: Coordinates) -> Option<Coordinates> {
        if index.row() > 0 {
            let index = (index.row() - 1, index.col());
            let node = &self[index];

            node.bottom.then_some(index)
        } else {
            None
        }
    }

    #[inline(always)]
    fn right(&self, index: Coordinates) -> Option<Coordinates> {
        if index.col() < self.cols() - 1 {
            let index = (index.row(), index.col() + 1);
            let node = &self[index];

            node.left.then_some(index)
        } else {
            None
        }
    }

    #[inline(always)]
    fn down(&self, index: Coordinates) -> Option<Coordinates> {
        if index.row() < self.rows() - 1 {
            let index = (index.row() + 1, index.col());
            let node = &self[index];

            node.top.then_some(index)
        } else {
            None
        }
    }
}
