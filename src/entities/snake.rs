use crate::core::{Direction, SegmentIndex, Vec2};
use crate::traits::{Damageable, Positioned, Renderable, Segmented};
use ratatui::{buffer::Buffer, style::Color, style::Style};
use std::collections::VecDeque;

pub struct Snake {
    segments: VecDeque<Vec2>,
    direction: Direction,
    pending_direction: Direction,
    grow_pending: u32,
}

impl Snake {
    pub fn new(position: Vec2) -> Self {
        Self {
            segments: VecDeque::from([position]),
            direction: Direction::Right,
            pending_direction: Direction::Right,
            grow_pending: 0,
        }
    }

    pub fn head(&self) -> Vec2 {
        self.segments.front().copied().unwrap_or_default()
    }

    pub fn center(&self) -> Vec2 {
        let len = self.segments.len();
        if len == 0 {
            return Vec2::zero();
        }
        self.segments
            .get(len / 2)
            .copied()
            .unwrap_or_else(|| self.head())
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if !self.direction.is_opposite(dir) {
            self.pending_direction = dir;
        }
    }

    pub fn grow(&mut self) {
        self.grow_pending += 1;
    }

    pub fn tick(&mut self, bounds: Vec2) -> SnakeMoveResult {
        self.direction = self.pending_direction;
        let new_head = self.head() + self.direction.to_vec2();

        if !new_head.in_bounds(bounds) {
            return SnakeMoveResult::HitWall;
        }

        if self.segments.iter().any(|&s| s == new_head) {
            return SnakeMoveResult::HitSelf;
        }

        self.segments.push_front(new_head);

        if self.grow_pending > 0 {
            self.grow_pending -= 1;
        } else {
            self.segments.pop_back();
        }

        SnakeMoveResult::Moved(new_head)
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        self.segments.iter().any(|&s| s == pos)
    }

    pub fn segments(&self) -> &VecDeque<Vec2> {
        &self.segments
    }

    pub fn length(&self) -> usize {
        self.segments.len()
    }

    fn truncate_to_index(&mut self, index: SegmentIndex) -> DamageResult {
        let segments_lost = self.segments.len().saturating_sub(index.as_usize() + 1);
        self.segments.truncate(index.as_usize() + 1);
        self.grow_pending = 0;
        DamageResult { segments_lost }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageResult {
    pub segments_lost: usize,
}

impl DamageResult {
    pub fn is_significant(&self) -> bool {
        self.segments_lost > 0
    }
}

impl Positioned for Snake {
    fn position(&self) -> Vec2 {
        self.head()
    }

    fn set_position(&mut self, pos: Vec2) {
        if let Some(head) = self.segments.front_mut() {
            *head = pos;
        }
    }
}

impl Segmented for Snake {
    fn segment_count(&self) -> usize {
        self.segments.len()
    }

    fn segment_at(&self, index: SegmentIndex) -> Option<Vec2> {
        self.segments.get(index.as_usize()).copied()
    }

    fn contains_position(&self, pos: Vec2) -> bool {
        self.contains(pos)
    }
}

impl Damageable for Snake {
    type DamageResult = DamageResult;

    fn damage_at(&mut self, index: SegmentIndex) -> Self::DamageResult {
        if index.is_head() {
            return DamageResult { segments_lost: 0 };
        }
        self.truncate_to_index(index)
    }
}

impl Renderable for Snake {
    fn render(&self, offset: Vec2, buf: &mut Buffer) {
        let head_style = Style::default().fg(Color::Rgb(200, 255, 200));
        let body_style = Style::default().fg(Color::White);

        for (i, segment) in self.segments.iter().enumerate() {
            let (x, y) = segment.to_screen(offset);
            let style = if i == 0 { head_style } else { body_style };
            buf.set_string(x, y, "▓▓", style);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnakeMoveResult {
    Moved(Vec2),
    HitWall,
    HitSelf,
}

impl SnakeMoveResult {
    pub fn is_fatal(&self) -> bool {
        matches!(self, SnakeMoveResult::HitWall | SnakeMoveResult::HitSelf)
    }

    pub fn new_position(&self) -> Option<Vec2> {
        match self {
            SnakeMoveResult::Moved(pos) => Some(*pos),
            _ => None,
        }
    }
}
