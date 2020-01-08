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
Usage: connect4 [-h] [-v] [PLAYER [PLAYER]]
    PLAYER:
        h              Human player
        a[level]       AI player, where level=difficulty
    -h                 Show this help message
    -v                 If an AI is present, make it verbose

Example:
    connect4           White: Human, Black: AI[level=8]
    connect4 a6 h      White: AI[level=6], Black: Human
    connect4 h         White: Human, Black: Human
    connect4 a a9      White: AI[level=8], Black: AI[level=9]
```
