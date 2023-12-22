use clap::Parser;
use env_logger::Env;
use live_server::listen;
use local_ip_address::local_ip;

/// Launch a local network server with live reload feature for static pages.
#[derive(Parser)]
#[clap(version)]
struct Args {
    /// Set the root path of the static assets
    #[clap(default_value = ".")]
    root: String,
    /// Set the listener host [default: LAN IP address]
    #[clap(short = 'H', long)]
    host: Option<String>,
    /// Set the listener port
    #[clap(short, long, default_value = "0")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);

    let args = Args::parse();
    let host = match args.host {
        Some(host) => host,
        None => match local_ip() {
            Err(err) => {
                log::error!(
                    "Failed to get local IP address: {}. Using \"localhost\" by default",
                    err
                );
                "localhost".to_string()
            }
            Ok(addr) => addr.to_string(),
        },
    };

    listen(&host, args.port, args.root).await.unwrap();
}
