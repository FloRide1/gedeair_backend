use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// The address for hosting the backend in format: "IP:PORT" (IP 0.0.0.0 means broadcast, 127.0.0.1 means local)
    #[arg(default_value = "0.0.0.0:3000")]
    pub address: String,

    /// Parameter for disabling default tracing
    #[arg(long, default_value_t = false)]
    pub disable_tracing: bool,
}
