use reqwest::blocking::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read};
use std::process::Command;

#[derive(Deserialize)]
pub struct VersionInfo {
    version: String,
    download_url: String,
    sha256_checksum: String,
    app_name: String,
}

pub fn get_version_info(url: &str) -> Result<VersionInfo, Box<dyn std::error::Error>> {
    let client = Client::new();
    let version_info: VersionInfo = client.get(url).send()?.json()?;
    Ok(version_info)
}

pub fn download_new_version(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create(file_path)?;
    io::copy(&mut response, &mut file)?;
    Ok(())
}

pub fn compute_file_sha256(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(buffer);
    Ok(hex::encode(hasher.finalize()))
}

pub fn update_application(current_version: &str, info_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let version_info: VersionInfo = get_version_info(info_url)?;
    if version_info.version == current_version {
        println!("Application is up-to-date.");
        return Ok(());
    }

    let temp_file_path = format!("{}_new.exe", version_info.version);
    download_new_version(&version_info.download_url, &temp_file_path)?;

    let downloaded_file_hash = compute_file_sha256(&temp_file_path)?;
    if downloaded_file_hash != version_info.sha256_checksum {
        return Err("Downloaded file hash does not match expected hash.".into());
    }

    // Remove old version. Be careful with this step; make sure you're deleting the correct file.
    let app_name: String = version_info.app_name;
    fs::remove_file(app_name.clone())?;

    // Rename the new executable to the original name. Adjust paths as necessary.
    fs::rename(&temp_file_path, app_name.clone())?;

    // Run the new version. Adjust the command as necessary.
    Command::new(app_name.clone()).spawn()?;

    Ok(())
}


/*
template.json

{
    "version": "1.0.1",
    "download_url": "https://example.com/your_application_1.0.1.exe",
    "sha256_checksum": "d73d56b328d5a8ffdf27430edb4d9d68e1e2a8f2c3e2656c672e4f6b76153a2b",
    "app_name": "your_application.exe"
}


*/