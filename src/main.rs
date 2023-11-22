use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

fn main() {
    let mut previous_price: f64 = 0.0;

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

        let current_price = String::from_utf8_lossy(&jq_output.stdout);
        let current_price: f64 = current_price.trim().parse().expect("Failed to parse price as f64");

        if previous_price != 0.0 {
            let price_change = current_price - previous_price;
            let percentage_change = (price_change / previous_price) * 100.0;

            //println!("Price Change: {:.6} USD", price_change);
            println!("Percentage Change: {:.6}%", percentage_change);
        }

        previous_price = current_price;

        thread::sleep(Duration::from_secs(10));
    }
}
