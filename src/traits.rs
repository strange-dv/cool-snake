use crate::core::{Edge, SegmentIndex, Vec2};
use ratatui::buffer::Buffer;

pub trait Positioned {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
}

pub trait Segmented: Positioned {
    fn segment_count(&self) -> usize;
    fn segment_at(&self, index: SegmentIndex) -> Option<Vec2>;
    fn contains_position(&self, pos: Vec2) -> bool;

    fn find_segment(&self, pos: Vec2) -> Option<SegmentIndex> {
        (0..self.segment_count())
            .map(SegmentIndex::from)
            .find(|&idx| self.segment_at(idx) == Some(pos))
    }

    fn body_contains(&self, pos: Vec2) -> bool {
        self.find_segment(pos).is_some_and(|idx| idx.is_body())
    }
}

pub trait Damageable: Segmented {
    type DamageResult;
    fn damage_at(&mut self, index: SegmentIndex) -> Self::DamageResult;
    fn damage_at_position(&mut self, pos: Vec2) -> Option<Self::DamageResult> {
        self.find_segment(pos).map(|idx| self.damage_at(idx))
    }
}

pub trait EdgeSpawnable: Sized {
    type Config;
    fn spawn_at_edge(edge: Edge, bounds: Vec2, config: Self::Config) -> Self;
    fn spawn_at_random_edge(bounds: Vec2, config: Self::Config) -> Self {
        use rand::Rng;
        let edge = Edge::ALL[rand::rng().random_range(0..4)];
        Self::spawn_at_edge(edge, bounds, config)
    }
}

pub trait Moveable: Positioned {
    fn velocity(&self) -> Vec2;
    fn set_velocity(&mut self, vel: Vec2);

    fn apply_movement(&mut self) {
        let pos = self.position();
        let vel = self.velocity();
        self.set_position(pos + vel);
    }
}

pub trait Collidable: Positioned {
    fn collides_with<T: Positioned>(&self, other: &T) -> bool {
        self.position() == other.position()
    }

    fn collides_with_any<'a, T: Positioned + 'a>(
        &self,
        others: impl Iterator<Item = &'a T>,
    ) -> bool {
        let pos = self.position();
        others.into_iter().any(|o| o.position() == pos)
    }
}

pub trait Renderable {
    fn render(&self, offset: Vec2, buf: &mut Buffer);
}

pub trait GameEntity: Positioned + Renderable {}

impl<T: Positioned + Renderable> GameEntity for T {}

pub trait Targetable: Positioned {
    fn is_valid_target(&self) -> bool;
}

pub trait Tickable {
    fn tick(&mut self);
}

pub trait BoundedTickable {
    fn tick(&mut self, bounds: Vec2);
}

pub trait Active {
    fn is_active(&self) -> bool;
    fn deactivate(&mut self);
}

pub trait Spawnable<Config> {
    fn spawn(config: Config) -> Self;
}
