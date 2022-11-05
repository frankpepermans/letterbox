use std::ops::{Index, IndexMut};

use bevy::prelude::Component;

use super::coordinates::{Coordinates, CreateCoordinates};

#[derive(Component, Debug, Clone)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        let len = rows * cols;
        let vec = vec![default_value; len];

        Self { vec, rows, cols }
    }

    pub fn contains(&self, coordinates: Coordinates) -> bool {
        self.rows > coordinates.0 && self.cols > coordinates.1
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> Index<Coordinates> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Coordinates) -> &Self::Output {
        let (row, col) = (index.row(), index.col());
        assert!(row < self.rows);
        assert!(col < self.cols);

        &self.vec[row * self.cols + col]
    }
}

impl<T> IndexMut<Coordinates> for Matrix<T> {
    fn index_mut(&mut self, index: Coordinates) -> &mut Self::Output {
        let (row, col) = (index.row(), index.col());
        assert!(row < self.rows);
        assert!(col < self.cols);

        &mut self.vec[row * self.cols + col]
    }
}
