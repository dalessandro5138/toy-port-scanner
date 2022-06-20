use std::arch::x86_64::_SIDD_CMP_EQUAL_EACH;
use std::io::Result as IOResult;
use std::net::{SocketAddr, TcpStream};
use std::thread::Result;
use std::time::Duration;
use std::time::Instant;

mod iter_chunker;
use iter_chunker::ChunkExt;

mod scanner;
use scanner::{Connect, ExecutionContext, PortScanner, SocketConnector};

fn main() {
    let now = Instant::now();

    let sc = SocketConnector::new(ExecutionContext::Parallel(10));
    let scanner = PortScanner::new(Duration::from_millis(3000), sc);

    let result = (50..90)
        .map(|port| SocketAddr::from(([192, 168, 1, 1], port)))
        .chunk(10)
        .into_iter()
        .flat_map(|socks| scanner.connect_batch(socks))
        .collect::<Vec<IOResult<TcpStream>>>();

    let elapsed = now.elapsed();

    println!("{:?}", elapsed);
    println!("{:?}", result);
}
