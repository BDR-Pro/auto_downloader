use auto_downloader::update_application;
fn main() {
    let current_version: &str = "1.0.0";
    let info_url: &str = "https://example.com/version_info";
    let respone:Result<_, _> =update_application(current_version, info_url);
    
    match respone {
        Ok(()) => println!("Application updated successfully."),
        Err(e) => println!("Error updating application: {}", e),
    }
}


/*


{
    "version": "1.0.1",
    "download_url": "https://example.com/your_application_1.0.1.exe",
    "sha256_checksum": "d73d56b328d5a8ffdf27430edb4d9d68e1e2a8f2c3e2656c672e4f6b76153a2b",
    "app_name": "your_application.exe"
}




*/