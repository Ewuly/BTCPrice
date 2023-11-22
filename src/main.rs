use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

//main
fn main() {
    loop {
        let curl_output = Command::new("curl")
            .args(&["-s", "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute curl command");

        let curl_stdout = curl_output.stdout.expect("Failed to read stdout of curl");

        let jq_output = Command::new("jq")
            .args(&["-r", ".price"])
            .stdin(Stdio::from(curl_stdout))
            .output()
            .expect("Failed to execute jq command");

        let price = String::from_utf8_lossy(&jq_output.stdout);
        print!("{}", price);

        thread::sleep(Duration::from_secs(1));
    }
}