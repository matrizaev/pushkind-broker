mod config;
mod proxy;

use crate::config::load_settings;
use crate::proxy::run_pair;
use log::{error, info};
use std::thread;
use thiserror::Error;

#[derive(Debug, Error)]
enum BrokerError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error("runtime error")]
    Runtime,
}

fn main() -> Result<(), BrokerError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let settings = load_settings()?;
    info!("loaded {} pair(s)", settings.pairs.len());

    let mut handles = Vec::with_capacity(settings.pairs.len());
    for (i, pair) in settings.pairs.into_iter().enumerate() {
        let label = pair
            .name
            .clone()
            .unwrap_or_else(|| format!("pair-{}", i + 1));
        info!(
            "[{label}] XSUB bind: {}  |  XPUB bind: {}  |  HWM(rx,tx)=({}, {})",
            pair.frontend, pair.backend, pair.xsub_rcvhwm, pair.xpub_sndhwm
        );
        let handle_label = label.clone();
        handles.push(thread::spawn(move || {
            let result = run_pair(&handle_label, pair);
            if let Err(e) = &result {
                error!("[{handle_label}] error: {e}");
            }
            result
        }));
    }

    let errors: Vec<_> = handles.into_iter().filter_map(|h| h.join().err()).collect();
    if !errors.is_empty() {
        Err(BrokerError::Runtime)
    } else {
        Ok(())
    }
}
