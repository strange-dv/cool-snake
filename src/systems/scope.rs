use crate::core::{Direction, Vec2};
use crate::traits::Renderable;
use ratatui::{buffer::Buffer, style::Color, style::Style};

pub struct ScopeConfig {
    pub aligned_color: Color,
    pub unaligned_color: Color,
    pub dot_spacing: usize,
}

impl Default for ScopeConfig {
    fn default() -> Self {
        Self {
            aligned_color: Color::Green,
            unaligned_color: Color::White,
            dot_spacing: 2,
        }
    }
}

pub struct Scope {
    origin: Vec2,
    direction: Direction,
    target: Option<Vec2>,
    bounds: Vec2,
    is_aligned: bool,
    config: ScopeConfig,
}

impl Scope {
    pub fn new() -> Self {
        Self::with_config(ScopeConfig::default())
    }

    pub fn with_config(config: ScopeConfig) -> Self {
        Self {
            origin: Vec2::zero(),
            direction: Direction::Right,
            target: None,
            bounds: Vec2::zero(),
            is_aligned: false,
            config,
        }
    }

    pub fn update(&mut self, origin: Vec2, direction: Direction, target: Vec2, bounds: Vec2) {
        self.origin = origin;
        self.direction = direction;
        self.target = Some(target);
        self.bounds = bounds;
        self.is_aligned = self.check_alignment();
    }

    fn check_alignment(&self) -> bool {
        let Some(target) = self.target else {
            return false;
        };

        match self.direction {
            Direction::Up => self.origin.x == target.x && target.y < self.origin.y,
            Direction::Down => self.origin.x == target.x && target.y > self.origin.y,
            Direction::Left => self.origin.y == target.y && target.x < self.origin.x,
            Direction::Right => self.origin.y == target.y && target.x > self.origin.x,
        }
    }

    pub fn is_aligned(&self) -> bool {
        self.is_aligned
    }

    pub fn ray_cast(&self) -> RayCastIterator {
        RayCastIterator {
            current: self.origin,
            direction: self.direction.to_vec2(),
            bounds: self.bounds,
            started: false,
        }
    }

    #[allow(dead_code)]
    pub fn distance_to_target(&self) -> Option<i32> {
        self.target.map(|t| {
            let diff = t - self.origin;
            diff.magnitude_squared()
        })
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Scope {
    fn render(&self, offset: Vec2, buf: &mut Buffer) {
        let color = if self.is_aligned {
            self.config.aligned_color
        } else {
            self.config.unaligned_color
        };

        let style = Style::default().fg(color);
        let glyph = "::";

        for (i, point) in self.ray_cast().enumerate() {
            if i % self.config.dot_spacing == 0 {
                let (x, y) = point.to_screen(offset);
                buf.set_string(x, y, glyph, style);
            }
        }
    }
}

pub struct RayCastIterator {
    current: Vec2,
    direction: Vec2,
    bounds: Vec2,
    started: bool,
}

impl Iterator for RayCastIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
        }
        self.current = self.current + self.direction;

        if self.current.in_bounds(self.bounds) {
            Some(self.current)
        } else {
            None
        }
    }
}

impl RayCastIterator {
    #[allow(dead_code)]
    pub fn hits_target(&mut self, target: Vec2) -> bool {
        self.any(|pos| pos == target)
    }
}
