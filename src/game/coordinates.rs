use crate::Position;

pub type Coordinates = (usize, usize);

pub trait CreateCoordinates {
    fn new(row: usize, col: usize) -> Self;
    fn row(&self) -> usize;
    fn col(&self) -> usize;
}

impl CreateCoordinates for Coordinates {
    fn new(row: usize, col: usize) -> Self {
        (row, col)
    }

    fn row(&self) -> usize {
        self.0
    }

    fn col(&self) -> usize {
        self.1
    }
}

impl Into<Position> for Coordinates {
    fn into(self) -> Position {
        Position(self)
    }
}
