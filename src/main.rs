use anyhow::{anyhow, Error};
use clap::Parser;
use core::time;
use notify_rust::Notification;
use scraper::{Html, Selector};
use std::thread;

/// Checks the status of a Lost Ark server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the server
    #[clap(short, long, default_value = "Trixion")]
    server_name: String,

    /// interval between checks if server is not up
    #[clap(short, long, default_value_t = 30)]
    interval: u64,
}

#[derive(Debug)]
enum Status {
    Good,
    Busy,
    Full,
    Maintenance,
}

fn get_status(server_name: &str) -> Result<Status, Error> {
    let html = ureq::get("https://www.playlostark.com/en-us/support/server-status")
        .call()?
        .into_string()?;

    let document = Html::parse_document(&html);

    let server_wrappers_selector =
        Selector::parse(".ags-ServerStatus-content-responses-response-server").unwrap();
    let server_wrapper = document
        .select(&server_wrappers_selector)
        .find(|w| w.text().any(|t| t.trim().eq(server_name)))
        .ok_or_else(|| anyhow!("missing server wrapper for selected server"))?;

    let server_status_wrapper_selector =
        Selector::parse(".ags-ServerStatus-content-responses-response-server-status").unwrap();
    let server_status_wrapper = server_wrapper
        .select(&server_status_wrapper_selector)
        .next()
        .ok_or_else(|| anyhow!("missing status wrapper for selected server"))?;
    let server_status: String = server_status_wrapper
        .value()
        .classes()
        .find(|c| c.starts_with("ags-ServerStatus-content-responses-response-server-status--"))
        .ok_or_else(|| anyhow!("missing status on wrapper for selected server"))?
        .chars()
        .skip("ags-ServerStatus-content-responses-response-server-status--".len())
        .collect();

    match server_status.as_str() {
        "good" => Ok(Status::Good),
        "busy" => Ok(Status::Busy),
        "full" => Ok(Status::Full),
        "maintenance" => Ok(Status::Maintenance),
        other => Err(anyhow!("unknown status: {}", other)),
    }
}

fn notify(server_name: &str, status: &Status) -> Result<(), Error> {
    Notification::new()
        .summary("Lost Ark")
        .body(format!("{} is up (status: {:?})", server_name, status).as_str())
        .show()?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    loop {
        let status = get_status(&args.server_name);
        match status {
            Err(e) => {
                println!(
                    "{} status is unknown, checking again in {}s ({})",
                    args.server_name, args.interval, e
                );
                thread::sleep(time::Duration::from_secs(args.interval));
            }
            Ok(Status::Maintenance) => {
                println!(
                    "{} is down for maintenance, checking again in {}s",
                    args.server_name, args.interval
                );
                thread::sleep(time::Duration::from_secs(args.interval));
            }
            Ok(other) => {
                println!("{} is up (status: {:?})", args.server_name, &other);
                notify(&args.server_name, &other).unwrap();
                break;
            }
        }
    }
}
