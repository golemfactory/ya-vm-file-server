#[macro_use]
pub mod core;
mod implementation;
mod input_args;

use crate::core::attributes_cache::*;
use crate::core::lib_utils::Result;
use crate::core::srv::srv_async;
use crate::implementation::unpfs::Unpfs;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use input_args::ServerOptions;
use std::env;
use std::sync::Arc;
use structopt;
use structopt::StructOpt;
use tokio::{fs, sync::Mutex};

async fn unpfs_main(server_options: ServerOptions) -> Result<i32> {
    let mount_point_metadata = fs::metadata(&server_options.mount_point).await;

    match mount_point_metadata {
        Ok(mount_point_metadata) => {
            if !mount_point_metadata.is_dir() {
                return res!(io_err!(
                    Other,
                    std::format!(
                        "Mount point {} must be a directory",
                        server_options.mount_point
                    )
                ));
            }
        }
        Err(err) => {
            return res!(io_err!(
                Other,
                std::format!(
                    "Mount point {} not found: {}",
                    server_options.mount_point,
                    err
                )
            ));
        }
    }
    log::info!(
        "Starting server {}: {}",
        server_options.network_protocol,
        server_options.network_address
    );
    srv_async(
        Unpfs {
            realroot: server_options.mount_point.into(),
            vap: Arc::new(Mutex::new(VirtualAttributesProvider::new())),
        },
        &server_options.network_protocol,
        &server_options.network_address,
    )
    .await
    .and(Ok(0))
}

pub fn build_rs_log(log_path: &str) -> anyhow::Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(log_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}

//sudo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser 192.168.174.1 ./mnt
#[tokio::main]
async fn main() {
    let server_options: ServerOptions = ServerOptions::from_args();

    if server_options.debug {
        env::set_var("RUST_LOG", "debug");
    }

    if let Some(ref log_path) = server_options.log_path {
        build_rs_log(log_path).unwrap_or_else(|e| {
            eprintln!("Error when building log: {:?}", e);
            panic!("Error when building log: {:?}", e)
        });
    } else {
        env_logger::init();
    }

    log::debug!("Runtime VM starting - log level debug message ...");
    log::info!("Runtime VM starting - log level info message ...");

    let exit_code = unpfs_main(server_options).await.unwrap_or_else(|e| {
        log::error!("{:?}", e);
        -1
    });

    std::process::exit(exit_code);
}
