use crate::config::Config;
use anyhow::Result;
use reqwest::Proxy;
use tokio::fs;
use crate::logging;
use std::{path::Path};
use crate::error::ProssimoError;
use colored::Colorize;

pub async fn validate_proxies(config: &Config) -> Result<()> {
    logging::trace(format!("Validating proxies from file: {}", &config.proxies.source_file).as_str());
    if !Path::new(&config.proxies.source_file).exists() {
        let errmsg=format!("Unable to validate proxies: Source file: {} does not exist.", &config.proxies.source_file);
        logging::error(&errmsg);
        return Err(ProssimoError::ConfigMissing(errmsg).into());
    } 
    let contents = fs::read_to_string(&config.proxies.source_file).await?;
    logging::info(format!("Loaded proxies from file: {}",&config.proxies.source_file).as_str());
    let mut live = Vec::new();

    let proxy_count = contents.lines().count();
    if proxy_count == 0 {
        logging::error("the proxy list is empty");
        return Err(ProssimoError::ConfigInvalid(format!("the proxy list: {} can not be empty.",config.proxies.source_file)).into())
    }else if proxy_count == 1 {
        let warn_msg = format!("The proxy list: {} only contained 1 proxy address.",config.proxies.source_file);
        logging::warn(&warn_msg);
    } else {
        let warn_msg = format!("starting to check {} proxy addresses...",proxy_count);
        logging::info(&warn_msg);
    }

    let mut valid_count = 0;
    let mut invalid_count = 0;

    for line in contents.lines() {
        let mut is_valid = false; 
        let proxy = Proxy::all(line)?;
        let client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        if let Ok(resp) = client.get(&config.proxies.validation_url).send().await {
            let ip = resp.text().await?;
            if line.contains(ip.trim()) {
                live.push(line.to_string());
                fs::write(&config.proxies.live_output_file,live.join("\n")).await?
            }
            valid_count += 1;
            is_valid = true;
        } else {
            invalid_count += 1;
        }
        let mut r="LIVE".green();
        if !is_valid { r = "DEAD".red();}
        println!(
            "{0}: {1}\t\t{2}\t[ LIVE:{3} DEAD:{4} ]",
            "CHECK".blue(),
            line,
            r,
            valid_count,
            invalid_count
        );
    }

    //fs::write(&config.proxies.live_output_file, live.join("\n")).await?;
    Ok(())
}
