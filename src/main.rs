mod config;
mod dns;
mod error;
mod logging;
mod proxy;
mod server;

use std::process::exit;

use anyhow::Result;
use config::{Config};
use proxy::{manager::ProxyManager, validator::validate_proxies};
use server::listener::start_proxy_server;
use ctrlc;
use nix::{sys::signal::{self,Signal}};
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<()> {

    ctrlc::set_handler(move || {
        println!("{}","Captured Ctrl-C, we will now gracefully shut down...".red());
        exit(0);
    }).expect("Error setting Ctrl-C handler!");

    //signal::signal(
    //    Signal::SIGTSTP,
    //    signal::SigHandler::Handler(handle_sigstp)
    //).expect("Error setting handler for Ctrl-Z!");

    let mut config = Config::load("config.yaml")?;
    logging::init(&config)?;
    logging::info("logging successfully initialized");
    logging::trace("parsing command line arguments...");
    
    let args = Config::get_command_line_parameters(&mut config);
    
    if args.dump_config_file {
        logging::trace("dumping configuration file and exiting...");
        println!("{:?}",config);
        return Ok(());
    }

    if args.sample_proxy_file {
        logging::trace("generating a sample proxy file and exiting...");
        println!("Sample proxy file:");
        println!("socks5://192.168.0.1:8080");
        println!("socks4://192.168.1.1:9000");
        println!("https://10.0.0.1:8000");
        println!("http://10.0.1.0:8080");
        return Ok(());
    }

    if !args.no_banner {
        config::print_banner(false);
    }

    let manager = ProxyManager::from_file(&config).await?;
    
    if config.proxies.validate_on_startup && !args.skip_validation {
        logging::trace("Validating proxies...");
        validate_proxies(&config).await?;
    }
    
    if !args.validate_only {
        logging::info("Starting proxy server...");
        start_proxy_server(config, manager).await?;
    }
    Ok(())
}

extern "C" fn handle_sigstp(_signum:i32){
    println!("{}","Captured Ctrl-Z, we will now gracefully shut down...".red());
    exit(0);
}
