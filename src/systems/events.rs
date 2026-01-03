use crate::core::{Direction, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    FoodCollected {
        position: Vec2,
        by_bullet: bool,
    },
    BulletFired {
        position: Vec2,
        direction: Direction,
    },
    SnakeDied {
        cause: DeathCause,
    },
    SnakeDamaged {
        position: Vec2,
        segments_lost: usize,
    },
    GamePaused,
    GameResumed,
    GameRestarted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    HitWall,
    HitSelf,
}

pub struct EventQueue {
    events: Vec<Option<GameEvent>>,
    write_idx: usize,
    read_idx: usize,
    capacity: usize,
}

impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: (0..capacity).map(|_| None).collect(),
            write_idx: 0,
            read_idx: 0,
            capacity,
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(32)
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events[self.write_idx] = Some(event);
        self.write_idx = (self.write_idx + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<GameEvent> {
        if self.read_idx == self.write_idx && self.events[self.read_idx].is_none() {
            return None;
        }

        let event = self.events[self.read_idx].take();
        if event.is_some() {
            self.read_idx = (self.read_idx + 1) % self.capacity;
        }
        event
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&GameEvent> {
        self.events[self.read_idx].as_ref()
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for event in &mut self.events {
            *event = None;
        }
        self.write_idx = 0;
        self.read_idx = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.read_idx == self.write_idx && self.events[self.read_idx].is_none()
    }

    pub fn drain(&mut self) -> EventDrainIterator<'_> {
        EventDrainIterator { queue: self }
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

pub struct EventDrainIterator<'a> {
    queue: &'a mut EventQueue,
}

impl Iterator for EventDrainIterator<'_> {
    type Item = GameEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}
