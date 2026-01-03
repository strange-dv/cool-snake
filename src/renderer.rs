use crate::core::{GameState, Vec2};
use crate::game::Game;
use crate::traits::Renderable;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct GameRenderer<'a> {
    game: &'a Game,
    config: RenderConfig,
}

pub struct RenderConfig {
    pub border_color: Color,
    pub text_color: Color,
    pub hud_color: Color,
    #[allow(dead_code)]
    pub show_debug: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            border_color: Color::DarkGray,
            text_color: Color::White,
            hud_color: Color::Cyan,
            show_debug: false,
        }
    }
}

impl<'a> GameRenderer<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self::with_config(game, RenderConfig::default())
    }

    pub fn with_config(game: &'a Game, config: RenderConfig) -> Self {
        Self { game, config }
    }

    fn render_border(&self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.border_color))
            .render(area, buf);
    }

    fn render_hud(&self, area: Rect, buf: &mut Buffer) {
        let score = self.game.score();
        let aligned = if self.game.is_scope_aligned() {
            "AIM"
        } else {
            "   "
        };

        let hud = format!(" {} | SCORE: {} ", aligned, score);

        buf.set_string(
            area.x + 2,
            area.y,
            &hud,
            Style::default().fg(self.config.hud_color),
        );
    }

    fn render_game_over(&self, area: Rect, buf: &mut Buffer) {
        let score_text = format!("SCORE: {}", self.game.score());
        let restart_text = "PRESS SPACE TO RESTART";

        let center_x = |s: &str| area.x + (area.width.saturating_sub(s.len() as u16)) / 2;
        let center_y = area.y + area.height / 2;

        let style = Style::default().fg(self.config.text_color);

        buf.set_string(center_x(&score_text), center_y - 1, &score_text, style);
        buf.set_string(center_x(restart_text), center_y + 1, restart_text, style);
    }

    fn render_paused(&self, area: Rect, buf: &mut Buffer) {
        let text = "PAUSED";
        let x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let y = area.y + area.height / 2;

        buf.set_string(x, y, text, Style::default().fg(self.config.text_color));
    }

    fn render_entities(&self, offset: Vec2, buf: &mut Buffer) {
        self.game.scope().render(offset, buf);
        self.game.snake().render(offset, buf);

        if !self.game.state().is_dead() {
            self.game.food().render(offset, buf);
        }

        self.game.bullets().render(offset, buf);
    }
}

impl Widget for GameRenderer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_border(area, buf);

        let offset = Vec2::new((area.x + 1) as i16, (area.y + 1) as i16);

        self.render_entities(offset, buf);

        match self.game.state() {
            GameState::Paused => self.render_paused(area, buf),
            GameState::Dead => self.render_game_over(area, buf),
            GameState::Playing => self.render_hud(area, buf),
        }
    }
}

pub struct MinimalRenderer<'a> {
    game: &'a Game,
}

impl<'a> MinimalRenderer<'a> {
    #[allow(dead_code)]
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }
}

impl Widget for MinimalRenderer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .render(area, buf);

        let offset = Vec2::new((area.x + 1) as i16, (area.y + 1) as i16);

        self.game.snake().render(offset, buf);

        if !self.game.state().is_dead() {
            self.game.food().render(offset, buf);
        }
    }
}
