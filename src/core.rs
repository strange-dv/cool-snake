use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16,
}

impl Vec2 {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn magnitude_squared(&self) -> i32 {
        (self.x as i32 * self.x as i32) + (self.y as i32 * self.y as i32)
    }

    pub fn dot(&self, other: Vec2) -> i32 {
        (self.x as i32 * other.x as i32) + (self.y as i32 * other.y as i32)
    }

    pub fn in_bounds(&self, bounds: Vec2) -> bool {
        self.x >= 0 && self.x < bounds.x && self.y >= 0 && self.y < bounds.y
    }

    pub fn to_screen(&self, offset: Vec2) -> (u16, u16) {
        (
            offset.x as u16 + self.x as u16 * 2,
            offset.y as u16 + self.y as u16,
        )
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i16> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub const fn to_vec2(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_opposite(self, other: Self) -> bool {
        self.opposite() == other
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    Dead,
}

impl GameState {
    pub fn is_active(&self) -> bool {
        matches!(self, GameState::Playing)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, GameState::Dead)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub width: i16,
    pub height: i16,
}

impl Bounds {
    pub const fn new(width: i16, height: i16) -> Self {
        Self { width, height }
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.width / 2, self.height / 2)
    }
}

impl From<(i16, i16)> for Bounds {
    fn from((w, h): (i16, i16)) -> Self {
        Self::new(w, h)
    }
}

impl From<Vec2> for Bounds {
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

impl Edge {
    pub const ALL: [Edge; 4] = [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];

    pub const fn opposite(self) -> Self {
        match self {
            Edge::Top => Edge::Bottom,
            Edge::Bottom => Edge::Top,
            Edge::Left => Edge::Right,
            Edge::Right => Edge::Left,
        }
    }

    pub const fn to_direction(self) -> Direction {
        match self {
            Edge::Top => Direction::Down,
            Edge::Bottom => Direction::Up,
            Edge::Left => Direction::Right,
            Edge::Right => Direction::Left,
        }
    }

    pub const fn is_horizontal(self) -> bool {
        matches!(self, Edge::Left | Edge::Right)
    }

    pub const fn is_vertical(self) -> bool {
        matches!(self, Edge::Top | Edge::Bottom)
    }

    pub const fn axis(self) -> Axis {
        match self {
            Edge::Top | Edge::Bottom => Axis::Vertical,
            Edge::Left | Edge::Right => Axis::Horizontal,
        }
    }
}

impl From<Edge> for Direction {
    fn from(edge: Edge) -> Self {
        edge.to_direction()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub const fn perpendicular(self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

impl From<Direction> for Axis {
    fn from(dir: Direction) -> Self {
        if dir.is_horizontal() {
            Axis::Horizontal
        } else {
            Axis::Vertical
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SegmentIndex(usize);

impl SegmentIndex {
    pub const HEAD: Self = Self(0);

    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn is_head(self) -> bool {
        self.0 == 0
    }

    pub const fn is_body(self) -> bool {
        self.0 > 0
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<usize> for SegmentIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<SegmentIndex> for usize {
    fn from(index: SegmentIndex) -> Self {
        index.0
    }
}
