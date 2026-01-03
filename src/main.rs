use clap::Parser;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "cool-snake")]
#[command(about = "An over-engineered snake game with bullets and scope")]
struct Cli {
    #[arg(short, long, default_value = "2", value_parser = clap::value_parser!(u8).range(1..=3))]
    speed: u8,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let tick_ms = match cli.speed {
        1 => 120,
        2 => 70,
        3 => 40,
        _ => 70,
    };

    let terminal = ratatui::init();
    let result = cool_snake::run(terminal, Duration::from_millis(tick_ms));
    ratatui::restore();
    result
}
