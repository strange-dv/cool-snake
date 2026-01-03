use crate::core::{Direction, Edge, Vec2};
use crate::traits::{
    Active, BoundedTickable, EdgeSpawnable, Moveable, Positioned, Renderable, Targetable,
};
use rand::Rng;
use ratatui::{buffer::Buffer, style::Color, style::Style};

#[derive(Clone, Copy, Debug)]
pub struct FoodConfig {
    pub speed_multiplier: i16,
    pub color: Color,
}

impl FoodConfig {
    pub const fn new() -> Self {
        Self {
            speed_multiplier: 1,
            color: Color::Rgb(138, 43, 226),
        }
    }

    #[allow(dead_code)]
    pub const fn with_speed(mut self, multiplier: i16) -> Self {
        self.speed_multiplier = multiplier;
        self
    }

    #[allow(dead_code)]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for FoodConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Food {
    position: Vec2,
    velocity: Vec2,
    active: bool,
    config: FoodConfig,
}

impl Food {
    pub fn new(position: Vec2) -> Self {
        Self::with_config(position, FoodConfig::default())
    }

    pub fn with_config(position: Vec2, config: FoodConfig) -> Self {
        Self {
            position,
            velocity: Vec2::zero(),
            active: true,
            config,
        }
    }

    fn compute_spawn_position(edge: Edge, bounds: Vec2) -> Vec2 {
        let mut rng = rand::rng();
        match edge {
            Edge::Top => Vec2::new(rng.random_range(0..bounds.x), 0),
            Edge::Bottom => Vec2::new(rng.random_range(0..bounds.x), bounds.y - 1),
            Edge::Left => Vec2::new(0, rng.random_range(0..bounds.y)),
            Edge::Right => Vec2::new(bounds.x - 1, rng.random_range(0..bounds.y)),
        }
    }

    fn compute_velocity(edge: Edge, config: &FoodConfig) -> Vec2 {
        let base_velocity: Direction = edge.into();
        base_velocity.to_vec2() * config.speed_multiplier
    }

    pub fn respawn_from_random_edge(&mut self, bounds: Vec2) {
        let edge = Edge::ALL[rand::rng().random_range(0..4)];
        self.position = Self::compute_spawn_position(edge, bounds);
        self.velocity = Self::compute_velocity(edge, &self.config);
        self.active = true;
    }

    pub fn is_out_of_bounds(&self, bounds: Vec2) -> bool {
        !self.position.in_bounds(bounds)
    }
}

impl Default for Food {
    fn default() -> Self {
        Self::new(Vec2::zero())
    }
}

impl EdgeSpawnable for Food {
    type Config = FoodConfig;

    fn spawn_at_edge(edge: Edge, bounds: Vec2, config: Self::Config) -> Self {
        let position = Self::compute_spawn_position(edge, bounds);
        let velocity = Self::compute_velocity(edge, &config);
        Self {
            position,
            velocity,
            active: true,
            config,
        }
    }
}

impl Positioned for Food {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
        self.active = true;
    }
}

impl Moveable for Food {
    fn velocity(&self) -> Vec2 {
        self.velocity
    }

    fn set_velocity(&mut self, vel: Vec2) {
        self.velocity = vel;
    }
}

impl Active for Food {
    fn is_active(&self) -> bool {
        self.active
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

impl Targetable for Food {
    fn is_valid_target(&self) -> bool {
        self.active
    }
}

impl BoundedTickable for Food {
    fn tick(&mut self, bounds: Vec2) {
        if !self.active {
            return;
        }
        self.position = self.position + self.velocity;
        if self.is_out_of_bounds(bounds) {
            self.respawn_from_random_edge(bounds);
        }
    }
}

impl Renderable for Food {
    fn render(&self, offset: Vec2, buf: &mut Buffer) {
        if !self.active {
            return;
        }
        let (x, y) = self.position.to_screen(offset);
        buf.set_string(x, y, "▓▓", Style::default().fg(self.config.color));
    }
}
