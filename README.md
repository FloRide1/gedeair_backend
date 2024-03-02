# Gedeair Backend

> By Florian "FloRide" Reimat

## About

This project is a backend for an RPG (Or in french JDR, thus the name) system.

## Usage

### Manual

```sh
# Dependencies (Optional)
$ nix-shell

$ cargo run --release
# or if you want to build
$ cargo build --release
$ ./target/release/gedeair_backend
```

### Docker

```
$ docker build -t <your-image-name> .

$ docker run <your-image-name>
```
