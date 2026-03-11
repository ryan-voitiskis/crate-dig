use std::io::{self, Write};

use crate::config;
use crate::discogs;

#[derive(clap::Args)]
pub(crate) struct SetupArgs {
    /// Accept defaults without prompting.
    #[arg(long, short = 'y')]
    yes: bool,
}

pub(crate) fn run_setup(args: SetupArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config_path =
        config::config_path().ok_or("Could not determine config directory — is $HOME set?")?;

    let mut cfg = config::load();
    let existed = config_path.exists();

    if existed {
        println!("Existing config: {}", config_path.display());
    } else {
        println!("No config found. Creating: {}", config_path.display());
    }

    // Broker URL
    let current_url = cfg
        .discogs
        .broker
        .url
        .as_deref()
        .unwrap_or(discogs::DEFAULT_BROKER_URL);
    if args.yes {
        if cfg.discogs.broker.url.is_none() {
            cfg.discogs.broker.url = Some(discogs::DEFAULT_BROKER_URL.to_string());
        }
    } else {
        println!();
        println!("Discogs broker URL [{}]:", current_url);
        let input = read_line_trimmed()?;
        cfg.discogs.broker.url = Some(if input.is_empty() {
            current_url.to_string()
        } else {
            input
        });
    }

    // Validate URL
    if let Some(ref url) = cfg.discogs.broker.url {
        if discogs::normalize_base_url(url).is_none() {
            return Err(format!("Invalid URL: {url}").into());
        }
    }

    // Broker token — only needed for custom broker URLs
    let is_default_url = cfg
        .discogs
        .broker
        .url
        .as_deref()
        .and_then(discogs::normalize_base_url)
        .as_deref()
        == Some(discogs::DEFAULT_BROKER_URL);

    if !is_default_url {
        if !args.yes {
            println!();
            println!("Custom broker URL detected. A broker token may be required.");
            let show_token = if cfg.discogs.broker.token.is_some() {
                "(configured)"
            } else {
                "(none)"
            };
            println!("Discogs broker token [{show_token}]:");
            let input = read_line_trimmed()?;
            if !input.is_empty() {
                let cleaned = input.trim_matches('"').trim_matches('\'').to_string();
                if cleaned.is_empty() {
                    return Err("Token cannot be empty.".into());
                }
                cfg.discogs.broker.token = Some(cleaned);
            }
        }
    }

    if !is_default_url && cfg.discogs.broker.token.is_none() {
        eprintln!(
            "Warning: Custom broker URL is configured but no broker token is set. \
             Set {} or re-run setup without --yes to configure it.",
            discogs::BROKER_TOKEN_ENV
        );
    }

    config::save(&cfg)?;
    println!();
    println!("Config saved to {}", config_path.display());
    println!();
    println!("Discogs enrichment is ready. Broker auth will happen on first use.");
    Ok(())
}

fn read_line_trimmed() -> io::Result<String> {
    print!("> ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}
