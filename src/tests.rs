mod core_tests {
    use crate::core::{Bounds, Direction, GameState, Vec2};

    #[test]
    fn vec2_new_creates_correct_values() {
        let v = Vec2::new(5, 10);
        assert_eq!(v.x, 5);
        assert_eq!(v.y, 10);
    }

    #[test]
    fn vec2_zero_is_origin() {
        let v = Vec2::zero();
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
    }

    #[test]
    fn vec2_add_works() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        let c = a + b;
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
    }

    #[test]
    fn vec2_sub_works() {
        let a = Vec2::new(5, 7);
        let b = Vec2::new(2, 3);
        let c = a - b;
        assert_eq!(c.x, 3);
        assert_eq!(c.y, 4);
    }

    #[test]
    fn vec2_mul_scalar_works() {
        let v = Vec2::new(2, 3);
        let result = v * 4;
        assert_eq!(result.x, 8);
        assert_eq!(result.y, 12);
    }

    #[test]
    fn vec2_magnitude_squared_works() {
        let v = Vec2::new(3, 4);
        assert_eq!(v.magnitude_squared(), 25);
    }

    #[test]
    fn vec2_dot_product_works() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(a.dot(b), 11);
    }

    #[test]
    fn vec2_in_bounds_true_when_inside() {
        let v = Vec2::new(5, 5);
        let bounds = Vec2::new(10, 10);
        assert!(v.in_bounds(bounds));
    }

    #[test]
    fn vec2_in_bounds_false_when_at_boundary() {
        let v = Vec2::new(10, 5);
        let bounds = Vec2::new(10, 10);
        assert!(!v.in_bounds(bounds));
    }

    #[test]
    fn vec2_in_bounds_false_when_negative() {
        let v = Vec2::new(-1, 5);
        let bounds = Vec2::new(10, 10);
        assert!(!v.in_bounds(bounds));
    }

    #[test]
    fn vec2_to_screen_converts_correctly() {
        let v = Vec2::new(5, 3);
        let offset = Vec2::new(1, 1);
        let (x, y) = v.to_screen(offset);
        assert_eq!(x, 11);
        assert_eq!(y, 4);
    }

    #[test]
    fn direction_to_vec2_up() {
        let v = Direction::Up.to_vec2();
        assert_eq!(v, Vec2::new(0, -1));
    }

    #[test]
    fn direction_to_vec2_down() {
        let v = Direction::Down.to_vec2();
        assert_eq!(v, Vec2::new(0, 1));
    }

    #[test]
    fn direction_to_vec2_left() {
        let v = Direction::Left.to_vec2();
        assert_eq!(v, Vec2::new(-1, 0));
    }

    #[test]
    fn direction_to_vec2_right() {
        let v = Direction::Right.to_vec2();
        assert_eq!(v, Vec2::new(1, 0));
    }

    #[test]
    fn direction_opposite_works() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn direction_is_opposite_true() {
        assert!(Direction::Up.is_opposite(Direction::Down));
        assert!(Direction::Left.is_opposite(Direction::Right));
    }

    #[test]
    fn direction_is_opposite_false() {
        assert!(!Direction::Up.is_opposite(Direction::Left));
        assert!(!Direction::Up.is_opposite(Direction::Up));
    }

    #[test]
    fn direction_is_horizontal() {
        assert!(Direction::Left.is_horizontal());
        assert!(Direction::Right.is_horizontal());
        assert!(!Direction::Up.is_horizontal());
        assert!(!Direction::Down.is_horizontal());
    }

    #[test]
    fn direction_is_vertical() {
        assert!(Direction::Up.is_vertical());
        assert!(Direction::Down.is_vertical());
        assert!(!Direction::Left.is_vertical());
        assert!(!Direction::Right.is_vertical());
    }

    #[test]
    fn game_state_is_active() {
        assert!(GameState::Playing.is_active());
        assert!(!GameState::Paused.is_active());
        assert!(!GameState::Dead.is_active());
    }

    #[test]
    fn game_state_is_dead() {
        assert!(GameState::Dead.is_dead());
        assert!(!GameState::Playing.is_dead());
        assert!(!GameState::Paused.is_dead());
    }

    #[test]
    fn bounds_new_creates_correct_values() {
        let b = Bounds::new(20, 15);
        assert_eq!(b.width, 20);
        assert_eq!(b.height, 15);
    }

    #[test]
    fn bounds_to_vec2() {
        let b = Bounds::new(20, 15);
        let v = b.to_vec2();
        assert_eq!(v.x, 20);
        assert_eq!(v.y, 15);
    }

    #[test]
    fn bounds_contains_inside() {
        let b = Bounds::new(10, 10);
        assert!(b.contains(Vec2::new(5, 5)));
    }

    #[test]
    fn bounds_contains_edge() {
        let b = Bounds::new(10, 10);
        assert!(!b.contains(Vec2::new(10, 5)));
    }

    #[test]
    fn bounds_center() {
        let b = Bounds::new(20, 10);
        let c = b.center();
        assert_eq!(c.x, 10);
        assert_eq!(c.y, 5);
    }
}

mod snake_tests {
    use crate::core::{Direction, Vec2};
    use crate::entities::{Snake, SnakeMoveResult};

    #[test]
    fn snake_new_at_position() {
        let snake = Snake::new(Vec2::new(5, 5));
        assert_eq!(snake.head(), Vec2::new(5, 5));
        assert_eq!(snake.length(), 1);
    }

    #[test]
    fn snake_initial_direction_is_right() {
        let snake = Snake::new(Vec2::new(5, 5));
        assert_eq!(snake.direction(), Direction::Right);
    }

    #[test]
    fn snake_set_direction_changes_pending() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.set_direction(Direction::Up);
        assert_eq!(snake.direction(), Direction::Right);
    }

    #[test]
    fn snake_cannot_reverse_direction() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.set_direction(Direction::Left);
        let _ = snake.tick(Vec2::new(20, 20));
        assert_eq!(snake.direction(), Direction::Right);
    }

    #[test]
    fn snake_tick_moves_head() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        let result = snake.tick(Vec2::new(20, 20));
        assert_eq!(result, SnakeMoveResult::Moved(Vec2::new(6, 5)));
        assert_eq!(snake.head(), Vec2::new(6, 5));
    }

    #[test]
    fn snake_tick_changes_direction() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.set_direction(Direction::Down);
        let _ = snake.tick(Vec2::new(20, 20));
        assert_eq!(snake.direction(), Direction::Down);
    }

    #[test]
    fn snake_hits_wall_at_boundary() {
        let mut snake = Snake::new(Vec2::new(9, 5));
        let result = snake.tick(Vec2::new(10, 10));
        assert_eq!(result, SnakeMoveResult::HitWall);
    }

    #[test]
    fn snake_hits_wall_negative() {
        let mut snake = Snake::new(Vec2::new(1, 5));
        snake.set_direction(Direction::Up);
        let _ = snake.tick(Vec2::new(10, 10));
        snake.set_direction(Direction::Left);
        let _ = snake.tick(Vec2::new(10, 10));
        let result = snake.tick(Vec2::new(10, 10));
        assert_eq!(result, SnakeMoveResult::HitWall);
    }

    #[test]
    fn snake_grow_increases_length() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.grow();
        let _ = snake.tick(Vec2::new(20, 20));
        assert_eq!(snake.length(), 2);
    }

    #[test]
    fn snake_grow_multiple_times() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.grow();
        snake.grow();
        let _ = snake.tick(Vec2::new(20, 20));
        let _ = snake.tick(Vec2::new(20, 20));
        assert_eq!(snake.length(), 3);
    }

    #[test]
    fn snake_contains_head() {
        let snake = Snake::new(Vec2::new(5, 5));
        assert!(snake.contains(Vec2::new(5, 5)));
    }

    #[test]
    fn snake_not_contains_other() {
        let snake = Snake::new(Vec2::new(5, 5));
        assert!(!snake.contains(Vec2::new(6, 5)));
    }

    #[test]
    fn snake_hits_self_when_long() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        for _ in 0..4 {
            snake.grow();
        }
        for _ in 0..3 {
            let _ = snake.tick(Vec2::new(20, 20));
        }
        snake.set_direction(Direction::Down);
        let _ = snake.tick(Vec2::new(20, 20));
        snake.set_direction(Direction::Left);
        let _ = snake.tick(Vec2::new(20, 20));
        snake.set_direction(Direction::Up);
        let result = snake.tick(Vec2::new(20, 20));
        assert_eq!(result, SnakeMoveResult::HitSelf);
    }

    #[test]
    fn snake_center_single_segment() {
        let snake = Snake::new(Vec2::new(5, 5));
        assert_eq!(snake.center(), Vec2::new(5, 5));
    }

    #[test]
    fn snake_center_multiple_segments() {
        let mut snake = Snake::new(Vec2::new(5, 5));
        snake.grow();
        snake.grow();
        let _ = snake.tick(Vec2::new(20, 20));
        let _ = snake.tick(Vec2::new(20, 20));
        let center = snake.center();
        assert_eq!(center, Vec2::new(6, 5));
    }

    #[test]
    fn snake_move_result_is_fatal() {
        assert!(!SnakeMoveResult::Moved(Vec2::new(0, 0)).is_fatal());
        assert!(SnakeMoveResult::HitWall.is_fatal());
        assert!(SnakeMoveResult::HitSelf.is_fatal());
    }

    #[test]
    fn snake_move_result_new_position() {
        assert_eq!(
            SnakeMoveResult::Moved(Vec2::new(5, 5)).new_position(),
            Some(Vec2::new(5, 5))
        );
        assert_eq!(SnakeMoveResult::HitWall.new_position(), None);
    }
}

mod bullet_tests {
    use crate::core::{Direction, Vec2};
    use crate::entities::Bullet;
    use crate::traits::{Active, BoundedTickable, Moveable, Positioned};

    #[test]
    fn bullet_new_at_position() {
        let bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        assert_eq!(bullet.position(), Vec2::new(5, 5));
    }

    #[test]
    fn bullet_initial_active() {
        let bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        assert!(bullet.is_active());
    }

    #[test]
    fn bullet_velocity_right() {
        let bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        assert_eq!(bullet.velocity(), Vec2::new(2, 0));
    }

    #[test]
    fn bullet_velocity_up() {
        let bullet = Bullet::new(Vec2::new(5, 5), Direction::Up);
        assert_eq!(bullet.velocity(), Vec2::new(0, -2));
    }

    #[test]
    fn bullet_tick_moves_bullet() {
        let mut bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        bullet.tick(Vec2::new(20, 20));
        assert_eq!(bullet.position(), Vec2::new(7, 5));
    }

    #[test]
    fn bullet_tick_decreases_lifetime() {
        let mut bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        let initial = bullet.lifetime();
        bullet.tick(Vec2::new(20, 20));
        assert_eq!(bullet.lifetime(), initial - 1);
    }

    #[test]
    fn bullet_deactivates_at_boundary() {
        let mut bullet = Bullet::new(Vec2::new(18, 5), Direction::Right);
        bullet.tick(Vec2::new(20, 20));
        assert!(!bullet.is_active());
    }

    #[test]
    fn bullet_deactivate_works() {
        let mut bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        bullet.deactivate();
        assert!(!bullet.is_active());
    }

    #[test]
    fn bullet_lifetime_fraction_full() {
        let bullet = Bullet::new(Vec2::new(5, 5), Direction::Right);
        assert!((bullet.lifetime_fraction() - 1.0).abs() < 0.01);
    }
}

mod bullet_pool_tests {
    use crate::core::{Direction, Vec2};
    use crate::systems::BulletPool;

    #[test]
    fn bullet_pool_new_empty() {
        let pool = BulletPool::new(10);
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn bullet_pool_spawn_adds_bullet() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        assert_eq!(pool.active_count(), 1);
    }

    #[test]
    fn bullet_pool_spawn_multiple() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        pool.spawn(Vec2::new(6, 6), Direction::Up);
        assert_eq!(pool.active_count(), 2);
    }

    #[test]
    fn bullet_pool_tick_moves_bullets() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        pool.tick(Vec2::new(20, 20));
        let bullet = pool.iter().next();
        assert!(bullet.is_some());
    }

    #[test]
    fn bullet_pool_cleanup_removes_inactive() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(18, 5), Direction::Right);
        pool.tick(Vec2::new(20, 20));
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn bullet_pool_available_is_infinite() {
        let pool = BulletPool::new(10);
        assert_eq!(pool.available(), usize::MAX);
    }

    #[test]
    fn bullet_pool_check_collision_direct_hit() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        pool.tick(Vec2::new(20, 20));
        let hit = pool.check_collision(Vec2::new(7, 5));
        assert!(hit);
    }

    #[test]
    fn bullet_pool_check_collision_misses() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        pool.tick(Vec2::new(20, 20));
        let hit = pool.check_collision(Vec2::new(10, 10));
        assert!(!hit);
    }

    #[test]
    fn bullet_pool_check_collision_line_intersect() {
        let mut pool = BulletPool::new(10);
        pool.spawn(Vec2::new(5, 5), Direction::Right);
        pool.tick(Vec2::new(20, 20));
        let hit = pool.check_collision(Vec2::new(6, 5));
        assert!(hit);
    }
}

mod food_tests {
    use crate::core::Vec2;
    use crate::entities::Food;
    use crate::traits::{Active, Positioned, Targetable};

    #[test]
    fn food_new_at_position() {
        let food = Food::new(Vec2::new(5, 5));
        assert_eq!(food.position(), Vec2::new(5, 5));
    }

    #[test]
    fn food_default_is_at_zero() {
        let food = Food::default();
        assert_eq!(food.position(), Vec2::new(0, 0));
    }

    #[test]
    fn food_is_valid_target() {
        let food = Food::new(Vec2::new(5, 5));
        assert!(food.is_valid_target());
    }

    #[test]
    fn food_deactivate_makes_invalid_target() {
        let mut food = Food::new(Vec2::new(5, 5));
        food.deactivate();
        assert!(!food.is_valid_target());
    }

    #[test]
    fn food_set_position_reactivates() {
        let mut food = Food::new(Vec2::new(5, 5));
        food.deactivate();
        food.set_position(Vec2::new(10, 10));
        assert!(food.is_active());
    }

    #[test]
    fn food_respawn_from_edge_changes_position() {
        let mut food = Food::new(Vec2::new(5, 5));
        let original = food.position();
        food.respawn_from_random_edge(Vec2::new(20, 20));
        let pos = food.position();
        let on_edge = pos.x == 0 || pos.x == 19 || pos.y == 0 || pos.y == 19;
        assert!(on_edge || pos != original);
    }
}

mod scope_tests {
    use crate::core::{Direction, Vec2};
    use crate::systems::Scope;

    #[test]
    fn scope_new_defaults() {
        let scope = Scope::new();
        assert!(!scope.is_aligned());
    }

    #[test]
    fn scope_aligned_when_target_in_line_right() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(5, 5),
            Direction::Right,
            Vec2::new(10, 5),
            Vec2::new(20, 20),
        );
        assert!(scope.is_aligned());
    }

    #[test]
    fn scope_aligned_when_target_in_line_up() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(5, 10),
            Direction::Up,
            Vec2::new(5, 3),
            Vec2::new(20, 20),
        );
        assert!(scope.is_aligned());
    }

    #[test]
    fn scope_not_aligned_when_target_behind() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(10, 5),
            Direction::Right,
            Vec2::new(5, 5),
            Vec2::new(20, 20),
        );
        assert!(!scope.is_aligned());
    }

    #[test]
    fn scope_not_aligned_when_target_off_axis() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(5, 5),
            Direction::Right,
            Vec2::new(10, 7),
            Vec2::new(20, 20),
        );
        assert!(!scope.is_aligned());
    }

    #[test]
    fn scope_ray_cast_returns_points() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(5, 5),
            Direction::Right,
            Vec2::new(10, 5),
            Vec2::new(20, 20),
        );
        let points: Vec<_> = scope.ray_cast().collect();
        assert!(!points.is_empty());
        assert_eq!(points[0], Vec2::new(6, 5));
    }

    #[test]
    fn scope_ray_cast_stops_at_bounds() {
        let mut scope = Scope::new();
        scope.update(
            Vec2::new(5, 5),
            Direction::Right,
            Vec2::new(10, 5),
            Vec2::new(10, 10),
        );
        let points: Vec<_> = scope.ray_cast().collect();
        assert_eq!(points.len(), 4);
    }
}

mod event_queue_tests {
    use crate::core::Vec2;
    use crate::systems::{EventQueue, GameEvent};

    #[test]
    fn event_queue_new_is_empty() {
        let queue = EventQueue::new(10);
        assert!(queue.is_empty());
    }

    #[test]
    fn event_queue_push_makes_non_empty() {
        let mut queue = EventQueue::new(10);
        queue.push(GameEvent::GamePaused);
        assert!(!queue.is_empty());
    }

    #[test]
    fn event_queue_pop_returns_pushed() {
        let mut queue = EventQueue::new(10);
        queue.push(GameEvent::GamePaused);
        let event = queue.pop();
        assert_eq!(event, Some(GameEvent::GamePaused));
    }

    #[test]
    fn event_queue_fifo_order() {
        let mut queue = EventQueue::new(10);
        queue.push(GameEvent::GamePaused);
        queue.push(GameEvent::GameResumed);
        assert_eq!(queue.pop(), Some(GameEvent::GamePaused));
        assert_eq!(queue.pop(), Some(GameEvent::GameResumed));
    }

    #[test]
    fn event_queue_pop_empty_returns_none() {
        let mut queue = EventQueue::new(10);
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn event_queue_drain_returns_all() {
        let mut queue = EventQueue::new(10);
        queue.push(GameEvent::GamePaused);
        queue.push(GameEvent::GameResumed);
        let events: Vec<_> = queue.drain().collect();
        assert_eq!(events.len(), 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn event_queue_wraps_around() {
        let mut queue = EventQueue::new(3);
        queue.push(GameEvent::GamePaused);
        queue.push(GameEvent::GameResumed);
        let _ = queue.pop();
        queue.push(GameEvent::FoodCollected {
            position: Vec2::new(0, 0),
            by_bullet: false,
        });
        assert_eq!(queue.pop(), Some(GameEvent::GameResumed));
    }
}

mod game_tests {
    use crate::core::{Direction, GameState, Vec2};
    use crate::game::{Game, GameBuilder};
    use crate::traits::Positioned;

    #[test]
    fn game_new_creates_playing_state() {
        let game = Game::new(20, 15);
        assert_eq!(game.state(), GameState::Playing);
    }

    #[test]
    fn game_new_has_zero_score() {
        let game = Game::new(20, 15);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn game_new_has_correct_bounds() {
        let game = Game::new(20, 15);
        assert_eq!(game.bounds(), (20, 15));
    }

    #[test]
    fn game_builder_creates_game() {
        let game = GameBuilder::new().with_bounds(20, 15).build();
        assert_eq!(game.bounds(), (20, 15));
    }

    #[test]
    fn game_set_direction_changes_snake() {
        let mut game = Game::new(20, 15);
        game.set_direction(Direction::Down);
        game.move_snake();
        assert_eq!(game.snake().direction(), Direction::Down);
    }

    #[test]
    fn game_toggle_pause_changes_state() {
        let mut game = Game::new(20, 15);
        game.toggle_pause();
        assert_eq!(game.state(), GameState::Paused);
    }

    #[test]
    fn game_toggle_pause_twice_resumes() {
        let mut game = Game::new(20, 15);
        game.toggle_pause();
        game.toggle_pause();
        assert_eq!(game.state(), GameState::Playing);
    }

    #[test]
    fn game_move_while_paused_does_nothing() {
        let mut game = Game::new(20, 15);
        let initial_head = game.snake().head();
        game.toggle_pause();
        game.move_snake();
        assert_eq!(game.snake().head(), initial_head);
    }

    #[test]
    fn game_move_snake_moves_snake() {
        let mut game = Game::new(20, 15);
        let initial_head = game.snake().head();
        game.move_snake();
        assert_ne!(game.snake().head(), initial_head);
    }

    #[test]
    fn game_fire_spawns_bullet() {
        let mut game = Game::new(20, 15);
        game.fire();
        assert_eq!(game.bullets().active_count(), 1);
    }

    #[test]
    fn game_fire_cooldown_prevents_spam() {
        let mut game = Game::new(20, 15);
        game.fire();
        let fired = game.fire();
        assert!(!fired);
    }

    #[test]
    fn game_fire_cooldown_resets() {
        let mut game = Game::new(50, 50);
        game.fire();
        for _ in 0..5 {
            game.tick();
            if game.state() == GameState::Dead {
                return;
            }
        }
        let fired = game.fire();
        assert!(fired);
    }

    #[test]
    fn game_food_exists() {
        let game = Game::new(20, 15);
        let food_pos = game.food().position();
        assert!(food_pos.in_bounds(Vec2::new(20, 15)));
    }

    #[test]
    fn game_restart_resets_score() {
        let mut game = Game::new(20, 15);
        game.tick();
        game.tick();
        game.restart();
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn game_restart_resets_state() {
        let mut game = Game::new(20, 15);
        game.toggle_pause();
        game.restart();
        assert_eq!(game.state(), GameState::Playing);
    }

    #[test]
    fn game_scope_aligned_when_food_in_line() {
        let mut game = Game::new(100, 100);
        for _ in 0..50 {
            if game.is_scope_aligned() {
                return;
            }
            game.set_direction(Direction::Up);
            game.tick();
            if game.state() == GameState::Dead {
                game.restart();
            }
            game.set_direction(Direction::Right);
            game.tick();
            if game.state() == GameState::Dead {
                game.restart();
            }
        }
    }
}

mod input_tests {
    use crate::core::Direction;
    use crate::input::{DefaultInputMapper, GameAction, InputMapper};
    use crossterm::event::KeyCode;

    #[test]
    fn input_mapper_quit_q() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Char('q')), Some(GameAction::Quit));
    }

    #[test]
    fn input_mapper_quit_esc() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Esc), Some(GameAction::Quit));
    }

    #[test]
    fn input_mapper_pause_space() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Char(' ')), Some(GameAction::Pause));
    }

    #[test]
    fn input_mapper_fire_f() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Char('f')), Some(GameAction::Fire));
    }

    #[test]
    fn input_mapper_fire_x() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Char('x')), Some(GameAction::Fire));
    }

    #[test]
    fn input_mapper_move_up_arrow() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Up),
            Some(GameAction::Move(Direction::Up))
        );
    }

    #[test]
    fn input_mapper_move_up_w() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Char('w')),
            Some(GameAction::Move(Direction::Up))
        );
    }

    #[test]
    fn input_mapper_move_up_k() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Char('k')),
            Some(GameAction::Move(Direction::Up))
        );
    }

    #[test]
    fn input_mapper_move_down() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Down),
            Some(GameAction::Move(Direction::Down))
        );
    }

    #[test]
    fn input_mapper_move_left() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Left),
            Some(GameAction::Move(Direction::Left))
        );
    }

    #[test]
    fn input_mapper_move_right() {
        let mapper = DefaultInputMapper;
        assert_eq!(
            mapper.map(KeyCode::Right),
            Some(GameAction::Move(Direction::Right))
        );
    }

    #[test]
    fn input_mapper_unknown_returns_none() {
        let mapper = DefaultInputMapper;
        assert_eq!(mapper.map(KeyCode::Char('z')), None);
    }
}
