use std::ops::{Index, IndexMut};

use super::coordinates::{Coordinates, CreateCoordinates};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    pub rows: usize,
    pub cols: usize,
    pub entanglements: Vec<(Coordinates, Coordinates)>,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        let len = rows * cols;
        let vec = vec![default_value; len];

        Self {
            vec,
            rows,
            cols,
            entanglements: Vec::new(),
        }
    }

    pub fn entangle(&mut self, left: Coordinates, right: Coordinates) {
        self.entanglements.push((left, right));
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
