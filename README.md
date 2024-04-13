# Gedeair Backend

> By Florian "FloRide" Reimat

## About

This project is a backend for an RPG (Or in french JDR, thus the name) system.

## Usage

### Manual

You need to provide a [PostgreSQL Database](https://www.postgresql.org/) connection, using either **C**ommand **L**ine **I**nterface arguments, Environment Variable or load them through an `.env` file, every argument can be seen with the `--help` (`-h` short version) arguments,
Some of the connection argument have default values, but you can override them with no worries. Here is an example of an `.env`file with all connection values filled:

```env
# DATABASE
# Either like this
POSTGRES_USER="gedeair_db_user"
POSTGRES_PASSWORD="my_secret_password"
POSTGRES_PORT=5432
POSTGRES_HOST="<gedeair_database_hostname>"

# Or like this (/!\ This one take priority)
DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

# OPENID
FRONTEND_BASE_URL="https://my_website_frontend_url.com"
BACKEND_BASE_URL="https://my_website_backend_url.com/with_its_endpoint"
OPENID_ISSUER="" # For Keycloak: "https://<my_keycloak_server>/realms/<my_realm>"
OPENID_CLIENT_ID="my_gedeair_client_id"
OPENID_CLIENT_SECRET=""
```

#### Run it

```sh
# Dependencies (Optional)
nix develop

cargo run --release
# or if you want to build
cargo build --release
./target/release/gedeair_backend
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
> | latest | The latest safest version (recommanded) |
> | vX.X.X | The image version |
