use std::io;
use std::path::PathBuf;

use muxy_app_core::settings::{PairingTokens, Settings};
use muxy_client::{Client, RemoteEndpoint};
use muxy_protocol::DeviceCredential;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("Pass the Muxy profile directory"))?;
    let settings = Settings::load(&profile.join("settings.toml"))?;
    let tokens = PairingTokens::load(&profile.join("paired-servers.toml"))?;
    let mut reports = Vec::new();
    for entry in settings.servers {
        let Some(paired) = &entry.paired else {
            continue;
        };
        let fingerprint = paired
            .fingerprint()
            .ok_or_else(|| io::Error::other("Invalid certificate fingerprint"))?;
        let token = tokens
            .token(entry.id)
            .ok_or_else(|| io::Error::other("Missing pairing token"))?;
        let result = Client::connect_remote(
            &RemoteEndpoint {
                hosts: paired.hosts.clone(),
                port: paired.port,
                fingerprint,
            },
            DeviceCredential {
                device: paired.device,
                token,
            },
        );
        match result {
            Ok(client) => {
                let ping = client.ping();
                let catalog = client.catalog();
                reports.push(serde_json::json!({
                    "name": entry.name, "authenticated": true,
                    "version": client.server_info().build.version,
                    "ping": ping.is_ok(), "error": ping.err().map(|error| error.to_string()),
                    "catalog_projects": catalog.as_ref().ok().map(|page| page.projects.len()),
                    "catalog_error": catalog.err().map(|error| error.to_string()),
                }));
                client.disconnect();
            }
            Err(error) => reports.push(serde_json::json!({
                "name": entry.name, "authenticated": false, "error": error.to_string(),
            })),
        }
    }
    serde_json::to_writer(io::stdout().lock(), &reports)?;
    Ok(())
}
