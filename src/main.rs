use ntfy::{Dispatcher, Payload, Priority};
use reqwest::blocking::Client;

fn get_latest_version() -> String {
    let client = Client::new();
    let response = client
        .head("https://update.code.visualstudio.com/latest/linux-x64/insider")
        .send()
        .unwrap();
    if let Some(content_disposition) = response.headers().get(reqwest::header::CONTENT_DISPOSITION)
    {
        println!("content_disposition: {:?}", content_disposition);
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
    println!("Hello, world!");
    let version = get_latest_version();
    println!("Latest: {}", version);
    let dispatcher = Dispatcher::builder("https://ntfy.sh").build().unwrap();
    let payload = Payload::new(NTFY_CHANNEL)
        .priority(Priority::High)
        .message(format!("Latest insiders version is {}", version))
        .title("VSCode Update");
    dispatcher.send(&payload).unwrap();
}
