use std::net::{SocketAddr, TcpStream};
use std::{io::Result as IOResult, thread, time::Duration};

mod iter_chunker;
use iter_chunker::ChunkExt;

type Res<T> = IOResult<T>;

const MIN_PORT: u16 = u16::MIN;
const MAX_PORT: u16 = u16::MAX;

pub struct PortScanner {
    timeout: Duration,
}

impl PortScanner {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    pub fn scan(&self, ip: [u8; 4]) -> Vec<IOResult<TcpStream>> {
        (MIN_PORT..MAX_PORT)
            .map(|port| SocketAddr::from((ip, port)))
            .chunk(2000)
            .into_iter()
            .flat_map(|socks| self.connect_batch(socks))
            .filter(|result| result.is_ok())
            .collect::<Vec<IOResult<TcpStream>>>()
    }
}

pub trait Execute {
    fn execute_all<F, T>(&self, f: Vec<F>) -> Vec<Res<T>>
    where
        F: FnOnce() -> T,
        T: Send + 'static,
        F: Send + 'static;
}

impl Execute for PortScanner {
    fn execute_all<F, T>(&self, f: Vec<F>) -> Vec<Res<T>>
    where
        F: FnOnce() -> T,
        T: Send + 'static,
        F: Send + 'static,
    {
        let a: Vec<_> = f
            .into_iter()
            .map(|fun| thread::spawn(move || fun()))
            .collect();

        a.into_iter()
            .map(|h| h.join().map_err(|e| std::panic::resume_unwind(Box::new(e))))
            .collect::<Vec<Res<T>>>()
    }
}

pub trait Connect {
    fn connect_batch(&self, socks: Vec<SocketAddr>) -> Vec<Res<TcpStream>>;
}

impl Connect for PortScanner {
    fn connect_batch(&self, socks: Vec<SocketAddr>) -> Vec<Res<TcpStream>> {
        let t = self.timeout;
        let handles: Vec<_> = socks
            .into_iter()
            .map(|sock| move || TcpStream::connect_timeout(&sock, t))
            .collect();

        self.execute_all(handles)
            .into_iter()
            .flat_map(|a| a.map_err(|e| std::panic::resume_unwind(Box::new(e))))
            .collect::<Vec<Res<TcpStream>>>()
    }
}
