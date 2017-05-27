# life-rs

Rust implementation of Conway's game of life using Piston library.

## Build

Just run ```cargo build --release```

## Run

Eaxmples:

`./life-rs`

Starts program with infinite board.

`./life-rs --cols=100 --rows=100` 

Starts program with board of width 100 and height 100, if no width or height specified the board will be infinite in that dimension.

`./life-rs --file file_name`

Starts program with initial board configuration from file `file_name`

## Controls

| Key | Action |
|-----|--------|
| Left, Right, Up or Down arrow | Move camera |
| Plus or Minus | Zoom in/out |
| s or f | Slower or faster evolution |
| p | Pause/Resume |
| h | Display help |
| ESC | Quit app |
