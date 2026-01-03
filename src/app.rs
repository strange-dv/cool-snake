use crate::core::GameState;
use crate::game::GameBuilder;
use crate::input::{DefaultInputMapper, GameAction, InputMapper};
use crate::renderer::GameRenderer;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

pub struct AppConfig {
    pub tick_duration: Duration,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tick_duration: Duration::from_millis(70),
        }
    }
}

pub struct App<M: InputMapper> {
    config: AppConfig,
    input_mapper: M,
}

impl App<DefaultInputMapper> {
    pub fn new() -> Self {
        Self::with_mapper(DefaultInputMapper)
    }
}

impl<M: InputMapper> App<M> {
    pub fn with_mapper(mapper: M) -> Self {
        Self {
            config: AppConfig::default(),
            input_mapper: mapper,
        }
    }

    pub fn with_config(mut self, config: AppConfig) -> Self {
        self.config = config;
        self
    }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut game_state = GameSessionState::new();
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| {
                let area = f.area();
                let (w, h) = (
                    (area.width.saturating_sub(2) / 2) as i16,
                    (area.height.saturating_sub(2)) as i16,
                );

                game_state.ensure_initialized(w, h);

                if let Some(game) = game_state.game() {
                    f.render_widget(GameRenderer::new(game), area);
                }
            })?;

            let timeout = self
                .config
                .tick_duration
                .saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(k) = event::read()? {
                    if k.kind != KeyEventKind::Press {
                        continue;
                    }

                    if let Some(action) = self.input_mapper.map(k.code) {
                        match game_state.handle_action(action) {
                            ActionResult::Continue => {}
                            ActionResult::Quit => return Ok(()),
                        }
                    }
                }
            }

            if last_tick.elapsed() >= self.config.tick_duration {
                game_state.tick();
                last_tick = Instant::now();
            }
        }
    }
}

impl Default for App<DefaultInputMapper> {
    fn default() -> Self {
        Self::new()
    }
}

struct GameSessionState {
    game: Option<crate::game::Game>,
    current_bounds: (i16, i16),
}

impl GameSessionState {
    fn new() -> Self {
        Self {
            game: None,
            current_bounds: (0, 0),
        }
    }

    fn ensure_initialized(&mut self, w: i16, h: i16) {
        if self.game.is_none() || self.current_bounds != (w, h) {
            self.game = Some(GameBuilder::new().with_bounds(w, h).build());
            self.current_bounds = (w, h);
        }
    }

    fn game(&self) -> Option<&crate::game::Game> {
        self.game.as_ref()
    }

    fn game_mut(&mut self) -> Option<&mut crate::game::Game> {
        self.game.as_mut()
    }

    fn tick(&mut self) {
        if let Some(g) = self.game.as_mut() {
            g.tick();
        }
    }

    fn handle_action(&mut self, action: GameAction) -> ActionResult {
        let Some(game) = self.game_mut() else {
            return ActionResult::Continue;
        };

        match action {
            GameAction::Quit => ActionResult::Quit,
            GameAction::Restart if game.state() == GameState::Dead => {
                game.restart();
                ActionResult::Continue
            }
            GameAction::Pause if game.state() == GameState::Dead => {
                game.restart();
                ActionResult::Continue
            }
            GameAction::Pause => {
                game.toggle_pause();
                ActionResult::Continue
            }
            GameAction::Fire => {
                game.fire();
                ActionResult::Continue
            }
            GameAction::Move(dir) => {
                game.set_direction(dir);
                game.move_snake();
                ActionResult::Continue
            }
            GameAction::Restart => ActionResult::Continue,
        }
    }
}

enum ActionResult {
    Continue,
    Quit,
}

pub fn run(terminal: DefaultTerminal, tick: Duration) -> Result<()> {
    App::new()
        .with_config(AppConfig {
            tick_duration: tick,
        })
        .run(terminal)
}
