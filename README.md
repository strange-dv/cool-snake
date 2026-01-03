# Cool Snake

An over-engineered terminal snake game built with Rust and Ratatui.

![Demo](demo/demo2.gif)

## What Makes It Cool

This isn't your regular snake game. It features:

- **Scope System** - A dotted line shows where you're aiming, turns green when aligned with food
- **Bullet Shooting** - Fire bullets to collect food faster than moving
- **Moving Food** - Food spawns from screen edges and moves across the field
- **Dangerous Food** - If food hits your body (not head), it cuts your snake

### Before

![Old Demo](demo/demo.gif)

## Build

```
cargo build --release
```

## Run

```
cargo run --release
```

### Speed Options

```
cargo run --release -- --speed 1  # Slow (120ms tick)
cargo run --release -- --speed 2  # Normal (70ms tick, default)
cargo run --release -- --speed 3  # Fast (40ms tick)
```

## Controls

### Movement
- `W` / `K` / `Up` - Move up
- `S` / `J` / `Down` - Move down
- `A` / `H` / `Left` - Move left
- `D` / `L` / `Right` - Move right

### Actions
- `F` / `X` - Fire bullet
- `Space` - Pause / Resume
- `Enter` - Restart (when dead)
- `Q` / `Esc` - Quit

## Tests

```
cargo test
```

Run with output:

```
cargo test -- --nocapture
```

## Architecture

The codebase is intentionally over-engineered with:

- Trait-based entity system (`Positioned`, `Moveable`, `Damageable`, `Segmented`)
- Phantom type state builder pattern for `Game`
- Object pool for bullets
- Event queue with ring buffer
- Type-safe `SegmentIndex` for snake segments
- `EdgeSpawnable` trait for food spawning
