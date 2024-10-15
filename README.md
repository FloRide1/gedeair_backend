# Gedeair Backend
> By Florian 'FloRide' Reimat

# About

Gedeair Backend is a RESTful API designed to serve as the backend for a tabletop role-playing game (RPG) system, known in French as a "Jeu de Rôle" (JDR), which is where the name Gedeair originates. The system provides essential features for managing game mechanics, player interactions, character data, and more, allowing Game Masters and players to focus on the experience without worrying about technicalities. Built with performance and scalability in mind, Gedeair Backend is a solution for both small games and large campaigns.

## Usage
### Manual
```sh
# Dependencies (Optional)
nix develop

# Setup example env
mv .env.prod.example .env

cargo run --release
# or if you want to build the exec
cargo build --release

# You can find the exec in the <project_dir>/target/release/gedeair_backend
```

### Nix
```
# Setup example env
mv .env.prod.example .env

nix build
```

### Docker

```sh
docker build -t <your-image-name> .

docker run <your-image-name>
```

> [!TIP]
> You can also look at [DockerHub](https://hub.docker.com/r/floride/gedeair_backend) for the official image:
> | tag | Explanation |
> |----------|-----------------------------------------|
> | master | The master branch of github (not safe) |
> | latest | The latest safest version (recommended) |
> | vX.X.X | The image version |
