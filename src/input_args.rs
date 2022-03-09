use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Rust crossplatform 9p server")]
pub struct ServerOptions {
    #[structopt(
        short = "a",
        long = "network-address",
        default_value = "127.0.0.1:7878",
        help = "Address to listen on: 127.0.0.1:7878"
    )]
    pub network_address: String,

    #[structopt(
        short = "p",
        long = "network-protocol",
        help = "For now only tcp/ip server is supported",
        default_value = "tcp"
    )]
    pub network_protocol: String,

    #[structopt(
        short = "m",
        long = "mount-point",
        help = "Path to map root of the filesystem to"
    )]
    pub mount_point: String,

    #[structopt(long = "log-path", help = "Path to log file")]
    pub log_path: Option<String>,

    #[structopt(short, long)]
    pub debug: bool,
}
