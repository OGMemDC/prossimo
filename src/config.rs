use hyper::Server;
use serde::{Deserialize};
use std::path::Path;
use crate::{error::ProssimoError, logging::trace};
use clap::{Parser, ValueEnum, builder::Str};
use crate::logging;

#[derive(ValueEnum,Clone,Debug,PartialEq)]
pub enum OutputMode {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None,
}

#[derive(Parser,Debug)]
#[command(
    name = "prossimo", 
    version ="0.0.1", 
    author ="O.G. BitBlt - bitbltog@proton.me", 
    about ="A high-performance proxy rotator for linux developed entirely in Rust.")]
pub struct CommandLineParameters {
    #[arg(long, help="file containing proxies, for sample format use --sample-proxy-file", default_value="none")]
    pub proxies: String,
    #[arg(long, help="location of an alternate config file to use", default_value="none")]
    pub config: String,
    #[arg(long, help="how much output to the screen is desired.", default_value="none")]
    pub output: OutputMode,
    #[arg(long, help="the local ip address to bind to", default_value="none")]
    pub ipaddress: String,
    #[arg(long, help="the local port to listen on", default_value="0")]
    pub lport: u16,
    #[arg(long, help="the number of requests before rotating the proxy", default_value="0")]
    pub rotate_every: u16,
    #[arg(long, help="skip proxy validation step", default_value="false")]
    pub skip_validation: bool,
    #[arg(long, help="just validate proxies and exit", default_value="false")]
    pub validate_only: bool,
    #[arg(long, help="output a sample proxy file", default_value="false")]
    pub sample_proxy_file: bool,
    #[arg(long, help="dump config file", default_value="false")]
    pub dump_config_file: bool,
    #[arg(long,help="don't show banner on startup",default_value="false")]
    pub no_banner: bool,
}

pub fn print_info() {
    println!(" Prossimo was designed to be used as an essential tool in your workflow");
    println!(" for pentesters or those in need of remaining anonymous. It is not by ");
    println!(" any means a complete anonymity platform and should not be thought of ");
    println!(" as such. The author recommends using a VPN, at least while running the ");
    println!(" proxy validations so that your IP reputation does not get damaged.");
    println!("");
    println!(" This software is free for anyone to use, as such the author provides no");
    println!(" support.");
    println!("");
    println!(" The author holds no claims of liability if you use this software with ");
    println!(" nefarious intent. This is your own choice and only you will be ");
    println!(" responsible for the outcome of such decisions. This statement is made ");
    println!(" with absolutely no judgements implied.");
    println!("");
    println!(" Do not believe in the collective wisdom of individual ignorance,");
    println!("");
    println!("       O.G. BitBlt - bitbltog@proton.me");
}

pub fn print_banner(info: bool) {
    println!("  _______________________________________________________________________");
    println!(" |     _____   ______  _____  _______ _______ _____ _______  _____       |");
    println!(" |    |_____] |_____/ |     | |______ |______   |   |  |  | |     |      |");
    println!(" |    |       |    \\_ |_____| ______| ______| __|__ |  |  | |_____|      |");                                                            
    println!(" |_______________________________________________________________________|");
    println!(" |                                                                       |");
    println!(" |             A high performance proxy rotator for linux.               |");
    println!(" |                                                                       |");
    println!(" |          Author:       O.G. BitBlt - bitbltog@proton.me               |");
    println!(" |            Date:       January 2026                                   |");
    println!(" |         Version:       0.0.1                                          |");
    println!(" |         Git Hub:       http://github.com/ogbitblt/prossimo            |");
    println!(" |         License:       MIT                                            |");
    println!(" |_______________________________________________________________________|");
    println!("");
    if info { print_info();}
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub proxies: ProxiesConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub listen_port: u16,
    pub rotate_every_requests: u16,
}


#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]    
pub struct ProxiesConfig  {
    pub source_file: String,
    pub live_output_file: String,
    pub validation_url: String,
    pub timeout_seconds: u64,
    pub validate_on_startup: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, ProssimoError> {
        if !Path::new(path).exists() {
            return Err(ProssimoError::ConfigMissing(path.into()));
        }
        let contents = std::fs::read_to_string(path)?;
        serde_yaml::from_str(&contents)
            .map_err(|e| ProssimoError::ConfigInvalid(e.to_string()))
    }

    pub fn set_server(&mut self,server: ServerConfig) {
        self.server=server;
    }

    pub fn set_logging(&mut self, logging:LoggingConfig){
        self.logging=logging;
    }

    pub fn set_proxies(&mut self, proxies:ProxiesConfig){
        self.proxies=proxies
    }

    pub fn get_command_line_parameters(&mut self) -> CommandLineParameters {
        logging::trace("parsing command line....");
        let args=CommandLineParameters::parse();

        let tracemsg=format!("{:?}",args);
        logging::trace(&tracemsg);

        if args.ipaddress != "none" && self.server.listen_addr != args.ipaddress {
            let tracemsg = format!("setting ip address {}",args.ipaddress);
            logging::trace(&tracemsg);
            self.server.set_listen_addr(args.ipaddress.clone());
        }

        if args.lport != 0 && self.server.listen_port != args.lport {
            let tracemsg = format!("setting listening port {}",args.lport);
            logging::trace(&tracemsg);
            self.server.set_listen_port(args.lport);
        }

        if args.proxies != "none" && self.proxies.source_file != args.proxies {
            let tracemsg = format!("setting proxy list file {}",args.proxies);
            logging::trace(&tracemsg);
            self.proxies.set_source_file(args.proxies.clone());
        }

        if args.rotate_every != 0 && self.server.rotate_every_requests != args.rotate_every {
            let tracemsg = format!("setting rotate value to {}", args.rotate_every);
            logging::trace(&tracemsg);
            self.server.set_rotate_every_requests(args.rotate_every);
        }

        if args.output != OutputMode::None {
            let tracemsg = format!("setting output level to {:?}",args.output);
            logging::trace(&tracemsg);
            match args.output {
                OutputMode::Debug   => self.logging.set_level("debug".into()),
                OutputMode::Error   => self.logging.set_level("error".into()),
                OutputMode::Info    => self.logging.set_level("info".into()),
                OutputMode::Trace   => self.logging.set_level("trace".into()),
                OutputMode::Warn    => self.logging.set_level("warn".into()), 
                OutputMode::None    => todo!(),
            }
        }
        logging::info("finished parsing command line parameters.");
        return args;
    }
}

impl ServerConfig {
    pub fn set_listen_addr(&mut self, listen_addr: String) {
        self.listen_addr=listen_addr
    }

    pub fn set_listen_port(&mut self, listen_port: u16) {
        self.listen_port=listen_port
    }

    pub fn set_rotate_every_requests(&mut self, rotate_every: u16){
        self.rotate_every_requests=rotate_every;
    }
}

impl LoggingConfig {
    pub fn set_level(&mut self, level: String){
        self.level = level
    }
}

impl ProxiesConfig {
    pub fn set_source_file(&mut self, source_file: String){
        self.source_file=source_file
    }
    pub fn set_validate_on_startup(&mut self,validate_on_startup: bool){
        self.validate_on_startup=validate_on_startup
    }
}




