use ntfy::{Dispatcher, Payload, Priority};
use reqwest::blocking::Client;
use std::process::Command;

fn get_archlinux_version(package_name: &str) -> String {
    let output = Command::new("pacman")
        .arg("-Qi")
        .arg(package_name)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.split('\n').collect();
    let version_line = lines
        .iter()
        .find(|line| line.starts_with("Version"))
        .unwrap();
    let version = version_line.split(": ").nth(1).unwrap().to_string();
    let main_version = version.split("-").nth(0).unwrap().to_string();

    main_version
}

fn get_latest_version() -> String {
    let client = Client::new();
    let response = client
        .head("https://update.code.visualstudio.com/latest/linux-x64/insider")
        .send()
        .unwrap();
    if let Some(content_disposition) = response.headers().get(reqwest::header::CONTENT_DISPOSITION)
    {
        // extract the filename from the content disposition header
        let filename = content_disposition
            .to_str()
            .unwrap()
            .split("filename=")
            .nth(1)
            .unwrap();
        // extract the version number from the filename
        let version = filename
            .split("-")
            .nth(3)
            .unwrap()
            .split(".")
            .nth(0)
            .unwrap();

        return version.to_string();
    }
    panic!("Could not get latest version from visualstudio.com");
}

const NTFY_CHANNEL: &str = "dcecef08-1839-40ed-b0ed-eee980594295";

fn main() {
    let version = get_latest_version();
    let archlinux_version = get_archlinux_version("visual-studio-code-insiders-bin");
    println!("       Installed: {}", archlinux_version);
    println!("Published latest: {}", version);

    if version != archlinux_version {
        let dispatcher = Dispatcher::builder("https://ntfy.sh").build().unwrap();
        let payload = Payload::new(NTFY_CHANNEL)
            .priority(Priority::High)
            .message(format!("New insiders version available {}", version))
            .title("VSCode Update");
        dispatcher.send(&payload).unwrap();
    } else {
        println!("No new version available");
    }
}
