# Gedeair Backend

> By Florian "FloRide" Reimat

## About

This project is a backend for an RPG (Or in french JDR, thus the name) system.

## Usage

### Manual

You need to provide a [PostgreSQL Database](https://www.postgresql.org/) connection, using either **C**ommand **L**ine **I**nterface arguments, Environment Variable or load them through an .env file, every argument can be seen with the --help (-h short version) arguments,
Some of the connection argument have default values, but you can override them with no worries. Here is an example of an .env file with all connection values filled:

```env
POSTGRES_USER="postgres"
POSTGRES_PASSWORD="postgres"
POSTGRES_PORT=5432
POSTGRES_HOST="localhost"
POSTGRES_DB="my_database_name"
```

#### Run it

```sh
# Dependencies (Optional)
nix-shell

cargo run --release
# or if you want to build
cargo build --release
./target/release/gedeair_backend
```

### Docker

```
docker build -t <your-image-name> .

docker run <your-image-name>
```

> [!TIP]
> You can also look at [DockerHub](https://hub.docker.com/r/floride/gedeair_backend) for the official image:
> | tag | Explanation |
> |----------|-----------------------------------------|
> | master | The master branch of github (not safe) |
> | latest | The latest safest version (recommanded) |
> | vX.X.X | The image version |

### Dev

You want to add more features, edit one or submit a fix ? Great ! Here is some command that can help you

> [!NOTE]
> I provide `shell.nix` for [Nix](https://nixos.wiki/wiki/Nix_package_manager), don't hesitate.

```
# Dependencies (Optional)
nix-shell

# Add the exemple as the current .env file (You can edit it)
cp .env.default .env

# Launch the dev docker compose environment (with PostgresSQL + PgAdmin)
docker-compose up -d

# Run tests
cargo test --all-features --workspace

# Run dev Program with -h argument
cargo run -- -h
```

> [!WARNING]
> I won't accept any commit containing a .vscode, .idea, etc... folder/file, i'm happy that you love your IDE / Text Editor but please keep it (and it's files) for yourself
