#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

pub mod app;
pub mod core;
pub mod entities;
pub mod game;
pub mod input;
pub mod renderer;
pub mod systems;
pub mod traits;

pub use app::{run, App, AppConfig};
pub use core::{Bounds, Direction, GameState, Vec2};
pub use entities::{Bullet, Food, Snake, SnakeMoveResult};
pub use game::{Game, GameBuilder, GameConfig};
pub use input::{DefaultInputMapper, GameAction, InputMapper};
pub use renderer::{GameRenderer, MinimalRenderer, RenderConfig};
pub use systems::{BulletPool, EventQueue, GameEvent, Scope};
pub use traits::{
    Active, BoundedTickable, Collidable, GameEntity, Moveable, Positioned, Renderable, Targetable,
    Tickable,
};

#[cfg(test)]
mod tests;
