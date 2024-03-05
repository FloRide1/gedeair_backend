use clap::{command, Parser};

/// The backend of an RPG system
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// The address for hosting the backend in format: "IP:PORT" (IP 0.0.0.0 means broadcast, 127.0.0.1 means local)
    #[arg(env, default_value = "0.0.0.0:3000")]
    pub address: String,

    /// Disable default tracing
    #[arg(env, long, default_value_t = false)]
    pub disable_tracing: bool,

    /// The database user
    #[arg(env, long, default_value = "postgres")]
    pub postgres_user: String,

    /// The database password, required unless "--database_url" / "DATABASE_URL" is set
    #[arg(env, long, required_unless_present("database_url"))]
    pub postgres_password: Option<String>,

    /// The database host (it can be an ip or an domain name)
    #[arg(env, long, default_value = "localhost")]
    pub postgres_host: String,

    /// The database port
    #[arg(env, long, default_value_t = 5432)]
    pub postgres_port: u16,

    /// The postgres database name
    #[arg(env, long, default_value = "postgres")]
    pub postgres_db: String,

    /// The postgres database schema
    #[arg(env, long)]
    pub postgres_schema: Option<String>,

    /// The full url for connection, it should look like this:
    /// "postgres://<username>:<password>@<host>:<port>/<database>"
    #[arg(env, long)]
    pub database_url: Option<String>,
}
