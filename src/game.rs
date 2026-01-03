use crate::core::{Bounds, Direction, Edge, GameState};
use crate::entities::{Food, Snake, SnakeMoveResult};
use crate::systems::{BulletPool, DeathCause, EventQueue, GameEvent, Scope};
use crate::traits::{BoundedTickable, Damageable, EdgeSpawnable, Positioned, Segmented};
use std::marker::PhantomData;

pub struct GameConfig {
    pub bounds: Bounds,
    pub bullet_pool_capacity: usize,
    pub event_queue_capacity: usize,
    pub bullet_cooldown_ticks: u8,
}

impl GameConfig {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            bounds: Bounds::new(width, height),
            bullet_pool_capacity: 16,
            event_queue_capacity: 32,
            bullet_cooldown_ticks: 3,
        }
    }
}

pub struct Uninitialized;
pub struct WithBounds;
pub struct Configured;

pub struct GameBuilder<State> {
    config: GameConfig,
    _state: PhantomData<State>,
}

impl GameBuilder<Uninitialized> {
    pub fn new() -> Self {
        Self {
            config: GameConfig::new(0, 0),
            _state: PhantomData,
        }
    }

    pub fn with_bounds(self, width: i16, height: i16) -> GameBuilder<WithBounds> {
        GameBuilder {
            config: GameConfig::new(width, height),
            _state: PhantomData,
        }
    }
}

impl GameBuilder<WithBounds> {
    #[allow(dead_code)]
    pub fn with_bullet_capacity(mut self, capacity: usize) -> Self {
        self.config.bullet_pool_capacity = capacity;
        self
    }

    #[allow(dead_code)]
    pub fn with_event_capacity(mut self, capacity: usize) -> Self {
        self.config.event_queue_capacity = capacity;
        self
    }

    #[allow(dead_code)]
    pub fn with_bullet_cooldown(mut self, ticks: u8) -> Self {
        self.config.bullet_cooldown_ticks = ticks;
        self
    }

    pub fn configure(self) -> GameBuilder<Configured> {
        GameBuilder {
            config: self.config,
            _state: PhantomData,
        }
    }

    pub fn build(self) -> Game {
        self.configure().build()
    }
}

impl GameBuilder<Configured> {
    pub fn build(self) -> Game {
        Game::with_config(self.config)
    }
}

impl Default for GameBuilder<Uninitialized> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Game {
    snake: Snake,
    food: Food,
    bullets: BulletPool,
    scope: Scope,
    events: EventQueue,
    state: GameState,
    score: u32,
    bounds: Bounds,
    bullet_cooldown: u8,
    bullet_cooldown_max: u8,
}

impl Game {
    pub fn new(width: i16, height: i16) -> Self {
        Self::with_config(GameConfig::new(width, height))
    }

    pub fn with_config(config: GameConfig) -> Self {
        let snake_pos = config.bounds.center();

        let mut game = Self {
            snake: Snake::new(snake_pos),
            food: Food::default(),
            bullets: BulletPool::new(config.bullet_pool_capacity),
            scope: Scope::new(),
            events: EventQueue::new(config.event_queue_capacity),
            state: GameState::Playing,
            score: 0,
            bounds: config.bounds,
            bullet_cooldown: 0,
            bullet_cooldown_max: config.bullet_cooldown_ticks,
        };

        game.spawn_food();
        game.update_scope();
        game
    }

    pub fn bounds(&self) -> (i16, i16) {
        (self.bounds.width, self.bounds.height)
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    fn spawn_food(&mut self) {
        use rand::Rng;
        loop {
            let edge = Edge::ALL[rand::rng().random_range(0..4)];
            let food = Food::spawn_at_edge(edge, self.bounds.to_vec2(), Default::default());
            if !self.snake.contains_position(food.position()) {
                self.food = food;
                break;
            }
        }
    }

    fn update_scope(&mut self) {
        self.scope.update(
            self.snake.head(),
            self.snake.direction(),
            self.food.position(),
            self.bounds.to_vec2(),
        );
    }

    pub fn tick(&mut self) {
        if !self.state.is_active() {
            return;
        }

        self.bullet_cooldown = self.bullet_cooldown.saturating_sub(1);

        self.food.tick(self.bounds.to_vec2());
        self.check_food_snake_collision();
        self.check_bullet_food_collisions();
        self.bullets.tick(self.bounds.to_vec2());
        self.update_scope();
    }

    fn check_food_snake_collision(&mut self) {
        let food_pos = self.food.position();
        if food_pos == self.snake.head() {
            self.collect_food(false);
        } else if let Some(damage) = self.snake.damage_at_position(food_pos) {
            if damage.is_significant() {
                self.events.push(GameEvent::SnakeDamaged {
                    position: food_pos,
                    segments_lost: damage.segments_lost,
                });
            }
            self.spawn_food();
        }
    }

    pub fn move_snake(&mut self) {
        if !self.state.is_active() {
            return;
        }

        match self.snake.tick(self.bounds.to_vec2()) {
            SnakeMoveResult::Moved(pos) => {
                if pos == self.food.position() {
                    self.collect_food(false);
                }
            }
            SnakeMoveResult::HitWall => {
                self.state = GameState::Dead;
                self.events.push(GameEvent::SnakeDied {
                    cause: DeathCause::HitWall,
                });
            }
            SnakeMoveResult::HitSelf => {
                self.state = GameState::Dead;
                self.events.push(GameEvent::SnakeDied {
                    cause: DeathCause::HitSelf,
                });
            }
        }

        self.update_scope();
    }

    fn check_bullet_food_collisions(&mut self) {
        let food_pos = self.food.position();
        if self
            .bullets
            .check_collision_before_tick(food_pos, self.bounds.to_vec2())
        {
            self.collect_food(true);
        }
    }

    fn collect_food(&mut self, by_bullet: bool) {
        self.score += 1;
        self.snake.grow();
        self.events.push(GameEvent::FoodCollected {
            position: self.food.position(),
            by_bullet,
        });
        self.spawn_food();
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.snake.set_direction(dir);
    }

    pub fn fire(&mut self) -> bool {
        if !self.state.is_active() || self.bullet_cooldown > 0 {
            return false;
        }

        let head = self.snake.head();
        let dir = self.snake.direction();
        let spawn_pos = head + dir.to_vec2();

        if !spawn_pos.in_bounds(self.bounds.to_vec2()) {
            return false;
        }

        if self.bullets.spawn(spawn_pos, dir) {
            self.bullet_cooldown = self.bullet_cooldown_max;
            self.events.push(GameEvent::BulletFired {
                position: spawn_pos,
                direction: dir,
            });
            return true;
        }

        false
    }

    #[allow(dead_code)]
    pub fn can_fire(&self) -> bool {
        self.state.is_active() && self.bullet_cooldown == 0
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => {
                self.events.push(GameEvent::GamePaused);
                GameState::Paused
            }
            GameState::Paused => {
                self.events.push(GameEvent::GameResumed);
                GameState::Playing
            }
            GameState::Dead => GameState::Dead,
        };
    }

    pub fn restart(&mut self) {
        let bounds = self.bounds;
        let bullet_cap = self.bullets.capacity();
        *self = Self::with_config(GameConfig {
            bounds,
            bullet_pool_capacity: bullet_cap,
            event_queue_capacity: 32,
            bullet_cooldown_ticks: self.bullet_cooldown_max,
        });
        self.events.push(GameEvent::GameRestarted);
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn food(&self) -> &Food {
        &self.food
    }

    pub fn bullets(&self) -> &BulletPool {
        &self.bullets
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    #[allow(dead_code)]
    pub fn events(&mut self) -> &mut EventQueue {
        &mut self.events
    }

    pub fn is_scope_aligned(&self) -> bool {
        self.scope.is_aligned()
    }
}
