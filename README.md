#Connect4 in Rust

## Building

### Linux / OSX
```bash
$ curl https://sh.rustup.rs -sSf | sh
```

### Windows
```bash
$ scoop install rustup
```

```bash
$ cd <this repository>
$ cargo build --release
```

## Running

```bash
$ cd <this repository>
$ cd target/release
$ connect4
```

```bash
USAGE:
    connect4 [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -1, --p1 <PLAYER>    Sets the player 1 (a: AI, h: Human) [default: h]  [possible values: a, h]
    -2, --p2 <PLAYER>    Sets the player 2 (a: AI, h: Human) [default: a]  [possible values: a, h]
    -s, --size <SIZE>    Sets the size of the board [default: 8]
```