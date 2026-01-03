use crate::core::{Direction, Vec2};
use crate::traits::{Active, BoundedTickable, Collidable, Moveable, Positioned, Renderable};
use ratatui::{buffer::Buffer, style::Color, style::Style};

#[allow(dead_code)]
pub struct BulletConfig {
    pub max_lifetime: u8,
    pub speed: i16,
}

impl Default for BulletConfig {
    fn default() -> Self {
        Self {
            max_lifetime: 50,
            speed: 2,
        }
    }
}

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    active: bool,
    lifetime: u8,
    max_lifetime: u8,
}

impl Bullet {
    pub fn new(position: Vec2, direction: Direction) -> Self {
        Self::with_config(position, direction, BulletConfig::default())
    }

    pub fn with_config(position: Vec2, direction: Direction, config: BulletConfig) -> Self {
        let velocity = direction.to_vec2() * config.speed;
        Self {
            position,
            velocity,
            active: true,
            lifetime: config.max_lifetime,
            max_lifetime: config.max_lifetime,
        }
    }

    #[allow(dead_code)]
    pub fn lifetime(&self) -> u8 {
        self.lifetime
    }

    #[allow(dead_code)]
    pub fn max_lifetime(&self) -> u8 {
        self.max_lifetime
    }

    pub fn lifetime_fraction(&self) -> f32 {
        if self.max_lifetime == 0 {
            return 0.0;
        }
        self.lifetime as f32 / self.max_lifetime as f32
    }
}

impl Positioned for Bullet {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }
}

impl Moveable for Bullet {
    fn velocity(&self) -> Vec2 {
        self.velocity
    }

    fn set_velocity(&mut self, vel: Vec2) {
        self.velocity = vel;
    }
}

impl Active for Bullet {
    fn is_active(&self) -> bool {
        self.active && self.lifetime > 0
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

impl BoundedTickable for Bullet {
    fn tick(&mut self, bounds: Vec2) {
        if !self.is_active() {
            return;
        }

        self.lifetime = self.lifetime.saturating_sub(1);

        let next = self.position + self.velocity;
        if !next.in_bounds(bounds) {
            self.active = false;
            return;
        }

        self.position = next;
    }
}

impl Collidable for Bullet {}

impl Renderable for Bullet {
    fn render(&self, offset: Vec2, buf: &mut Buffer) {
        if !self.is_active() {
            return;
        }

        let intensity = (self.lifetime_fraction() * 255.0) as u8;
        let (x, y) = self.position.to_screen(offset);
        buf.set_string(
            x,
            y,
            "██",
            Style::default().fg(Color::Rgb(255, intensity, 0)),
        );
    }
}
