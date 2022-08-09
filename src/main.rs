use std::time::Duration;

mod scanner;
use scanner::PortScanner;

fn main() {

    let scanner = PortScanner::new(Duration::from_millis(500));

    let ip = [192, 168, 1, 1];

    let results = scanner.scan(ip);

    println!("{:?}", results);
}
