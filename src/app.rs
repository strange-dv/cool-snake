use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    buffer::Buffer, layout::Rect, style::Color, style::Style, widgets::Block, widgets::Borders,
    widgets::Widget, DefaultTerminal,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pos(i16, i16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Playing,
    Paused,
    Dead,
}

struct Game {
    snake: VecDeque<Pos>,
    dir: Dir,
    next_dir: Dir,
    food: Pos,
    score: u32,
    state: State,
    bounds: (i16, i16),
}

impl Game {
    fn new(w: i16, h: i16) -> Self {
        let mut game = Self {
            snake: VecDeque::from([Pos(w / 2, h / 2)]),
            dir: Dir::Right,
            next_dir: Dir::Right,
            food: Pos(0, 0),
            score: 0,
            state: State::Playing,
            bounds: (w, h),
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let pos = Pos(
                rng.random_range(0..self.bounds.0),
                rng.random_range(0..self.bounds.1),
            );
            if !self.snake.contains(&pos) {
                self.food = pos;
                break;
            }
        }
    }

    fn tick(&mut self) {
        if self.state != State::Playing {
            return;
        }

        self.dir = self.next_dir;
        let Pos(x, y) = *self.snake.front().unwrap();
        let head = match self.dir {
            Dir::Up => Pos(x, y - 1),
            Dir::Down => Pos(x, y + 1),
            Dir::Left => Pos(x - 1, y),
            Dir::Right => Pos(x + 1, y),
        };

        if head.0 < 0
            || head.1 < 0
            || head.0 >= self.bounds.0
            || head.1 >= self.bounds.1
            || self.snake.contains(&head)
        {
            self.state = State::Dead;
            return;
        }

        self.snake.push_front(head);
        if head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn set_dir(&mut self, dir: Dir) {
        let dominated = matches!(
            (self.dir, dir),
            (Dir::Up, Dir::Down)
                | (Dir::Down, Dir::Up)
                | (Dir::Left, Dir::Right)
                | (Dir::Right, Dir::Left)
        );
        if !dominated {
            self.next_dir = dir;
        }
    }

    fn toggle_pause(&mut self) {
        self.state = match self.state {
            State::Playing => State::Paused,
            State::Paused => State::Playing,
            State::Dead => State::Dead,
        };
    }

    fn restart(&mut self) {
        *self = Self::new(self.bounds.0, self.bounds.1);
    }
}

pub fn run(mut terminal: DefaultTerminal, tick: Duration) -> Result<()> {
    let mut game: Option<Game> = None;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let (w, h) = (
                (area.width.saturating_sub(2) / 2) as i16,
                (area.height.saturating_sub(2)) as i16,
            );

            if game.is_none() || game.as_ref().unwrap().bounds != (w, h) {
                game = Some(Game::new(w, h));
            }

            f.render_widget(Renderer(game.as_ref().unwrap()), area);
        })?;

        let timeout = tick.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                let g = game.as_mut().unwrap();
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char(' ') | KeyCode::Enter if g.state == State::Dead => g.restart(),
                    KeyCode::Char(' ') | KeyCode::Enter => g.toggle_pause(),
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => g.set_dir(Dir::Up),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => g.set_dir(Dir::Down),
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => g.set_dir(Dir::Left),
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                        g.set_dir(Dir::Right)
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick {
            if let Some(g) = game.as_mut() {
                g.tick();
            }
            last_tick = Instant::now();
        }
    }
}

struct Renderer<'a>(&'a Game);

impl Widget for Renderer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .render(area, buf);

        let (ox, oy) = (area.x + 1, area.y + 1);
        let white = Style::default().fg(Color::White);

        for &Pos(x, y) in &self.0.snake {
            buf.set_string(ox + x as u16 * 2, oy + y as u16, "▓▓", white);
        }

        if self.0.state != State::Dead {
            let Pos(x, y) = self.0.food;
            buf.set_string(
                ox + x as u16 * 2,
                oy + y as u16,
                "▓▓",
                Style::default().fg(Color::Rgb(138, 43, 226)),
            );
        }

        let center = |s: &str| {
            (
                area.x + (area.width.saturating_sub(s.len() as u16)) / 2,
                area.y + area.height / 2,
            )
        };

        match self.0.state {
            State::Paused => {
                let (x, y) = center("PAUSED");
                buf.set_string(x, y, "PAUSED", white);
            }
            State::Dead => {
                let s = self.0.score.to_string();
                let (x, y) = center(&s);
                buf.set_string(x, y, &s, white);
            }
            State::Playing => {}
        }
    }
}
