use super::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, Eq)]
pub struct PathNode {
    pub index: Coordinates,
    pub f: i32,
    pub h: i32,
    pub g: i32,
    pub parent: Option<Coordinates>,
}

impl PathNode {
    pub fn initial<H>(index: Coordinates, goal: Coordinates, heuristic: H) -> Self
    where
        H: Fn(&Coordinates, &Coordinates) -> i32,
    {
        let h = heuristic(&index, &goal);

        PathNode {
            index: index,
            parent: None,
            f: h + 1,
            g: 1,
            h: h,
        }
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl From<Coordinates> for PathNode {
    fn from(value: Coordinates) -> Self {
        PathNode {
            index: value,
            f: 0,
            h: 0,
            g: 0,
            parent: None,
        }
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}
