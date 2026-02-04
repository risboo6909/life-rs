# life-rs

Conway's Game of Life implemented in Rust using Piston.

## Build

```bash
cargo build --release
```

## Run

Run from the project root:

```bash
cargo run -- [OPTIONS]
```

Examples:

```bash
# Infinite board
cargo run

# Finite board: 100x100 cells
cargo run -- --cols 100 --rows 100

# Finite board with custom window size
cargo run -- --cols 120 --rows 80 --width 1280 --height 720

# Load initial pattern from file
cargo run -- --file examples/glider.rle
```

Board size:

- `--cols <N>` sets board width in cells
- `--rows <N>` sets board height in cells
- if omitted, that dimension is infinite

Window size:

- `--width <N>` window width in pixels (default `1024`)
- `--height <N>` window height in pixels (default `768`)

## Controls

| Key | Action |
|-----|--------|
| `P` | Start/pause simulation |
| `S` | Step simulation by one generation |
| `R` | Generate random board (when paused) |
| `C` | Clear board |
| `X` | Paste pattern from clipboard (RLE/text parser input) |
| `G` | Show/hide grid |
| `F` | Fit board to screen |
| `Arrow keys` | Move camera |
| `+` / `-` | Zoom in/out |
| `Mouse drag` | Draw/erase cells |
| `Esc` | Quit |

## Help

```bash
cargo run -- --help
```
