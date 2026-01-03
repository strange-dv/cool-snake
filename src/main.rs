mod app;

use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::run(terminal, Duration::from_millis(70));
    ratatui::restore();
    result
}
