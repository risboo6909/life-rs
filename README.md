# life-rs

Rust implementation of Conway's game of life using Piston library.

## Build

Just run ```cargo build --release```

## Run

Eaxmple:

```./life-rs --width=100 --height=100``` 

Starts program with board size of width 100 and height 100, if no width or height specified the board will be infinite in that dimension. 

## Controls

| Key | Action |
|-----|--------|
| Left, Right, Up or Down arrow | Move camera |
| Plus or Minus | Zoom in/out |
| P | Pause/Resume |
| H | Display help |
| ESC | Quit app |
