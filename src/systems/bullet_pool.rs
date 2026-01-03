use crate::core::{Direction, Vec2};
use crate::entities::Bullet;
use crate::traits::{Active, BoundedTickable, Moveable, Positioned, Renderable};
use ratatui::buffer::Buffer;

pub struct BulletPool {
    bullets: Vec<Bullet>,
    max_active: usize,
}

impl BulletPool {
    pub fn new(max_active: usize) -> Self {
        Self {
            bullets: Vec::new(),
            max_active,
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(16)
    }

    pub fn spawn(&mut self, position: Vec2, direction: Direction) -> bool {
        self.cleanup();
        self.bullets.push(Bullet::new(position, direction));
        true
    }

    pub fn tick(&mut self, bounds: Vec2) {
        for bullet in &mut self.bullets {
            bullet.tick(bounds);
        }
        self.cleanup();
    }

    pub fn check_collision_before_tick(&mut self, target: Vec2, bounds: Vec2) -> bool {
        for bullet in &mut self.bullets {
            if !bullet.is_active() {
                continue;
            }

            let current = bullet.position();
            let vel = bullet.velocity();

            if current == target {
                bullet.deactivate();
                return true;
            }

            let speed = vel.x.abs().max(vel.y.abs());
            if speed > 0 {
                let dir_x = vel.x.signum();
                let dir_y = vel.y.signum();

                for i in 1..=speed {
                    let check_pos = Vec2::new(current.x + dir_x * i, current.y + dir_y * i);

                    if !check_pos.in_bounds(bounds) {
                        break;
                    }

                    if check_pos == target {
                        bullet.deactivate();
                        return true;
                    }
                }
            }
        }
        false
    }

    fn cleanup(&mut self) {
        self.bullets.retain(|b| b.is_active());
    }

    pub fn iter(&self) -> impl Iterator<Item = &Bullet> {
        self.bullets.iter().filter(|b| b.is_active())
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bullet> {
        self.bullets.iter_mut().filter(|b| b.is_active())
    }

    pub fn active_count(&self) -> usize {
        self.bullets.iter().filter(|b| b.is_active()).count()
    }

    pub fn capacity(&self) -> usize {
        self.max_active
    }

    pub fn available(&self) -> usize {
        usize::MAX
    }

    pub fn check_collision(&mut self, target: Vec2) -> bool {
        for bullet in &mut self.bullets {
            if !bullet.is_active() {
                continue;
            }

            if bullet.position() == target {
                bullet.deactivate();
                return true;
            }

            let vel = bullet.velocity();
            let speed = vel.x.abs().max(vel.y.abs());
            if speed > 1 {
                let dir_x = vel.x.signum();
                let dir_y = vel.y.signum();
                let prev_pos = bullet.position() - vel;

                for i in 0..speed {
                    let check_pos = Vec2::new(prev_pos.x + dir_x * i, prev_pos.y + dir_y * i);
                    if check_pos == target {
                        bullet.deactivate();
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl Default for BulletPool {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

impl Renderable for BulletPool {
    fn render(&self, offset: Vec2, buf: &mut Buffer) {
        for bullet in self.iter() {
            bullet.render(offset, buf);
        }
    }
}
