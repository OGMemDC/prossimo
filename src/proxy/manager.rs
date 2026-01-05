use super::types::Proxy;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::logging;
use std::path::Path;
use crate::error::ProssimoError;


#[derive(Debug)]
pub struct ProxyManager {
    proxies: Vec<Proxy>,
    index: AtomicUsize,
}

impl ProxyManager {
    pub async fn next(&self) -> Proxy {
        let i = self.index.fetch_add(1, Ordering::SeqCst);
        self.proxies[i % self.proxies.len()].clone()
    }
}

impl ProxyManager {
    pub async fn from_file(config: &crate::config::Config) -> anyhow::Result<Self> {
        logging::trace(format!("Loading proxies from file: {}", &config.proxies.live_output_file).as_str());
        if !Path::new(&config.proxies.live_output_file).exists() {
            let errmsg=format!("Unable to load proxies: Proxy file: {} does not exist.", &config.proxies.live_output_file);
            logging::error(&errmsg);
            return Err(ProssimoError::ConfigMissing(errmsg).into());
        }
        let mut contents = tokio::fs::read_to_string(&config.proxies.live_output_file).await?;
        logging::trace(&contents);
        let mut all_chars=contents.chars();
        
        logging::trace(format!("Last Char {:?}",all_chars.last()).as_str());
        while contents.ends_with('\n') {
            logging::trace("trimming new line character from end of file");
            let mut chars = contents.chars();
            chars.next_back();
            contents=chars.as_str().into();
        }

        let proxies = contents
            .lines()
            .filter_map(|line| line.parse::<Proxy>().ok())
            .collect::<Vec<_>>();
        let tracemsg = format!("loaded {0} proxies from the file {1}.",proxies.iter().count(),config.proxies.source_file);
        logging::trace(&tracemsg);
        
        Ok(Self {
            proxies,
            index: AtomicUsize::new(0),
        })
    }
}

impl Clone for ProxyManager {
    fn clone(&self) -> Self {
        Self {
            proxies: self.proxies.clone(),
            index: AtomicUsize::new(self.index.load(Ordering::SeqCst)),
        }
    }
}




