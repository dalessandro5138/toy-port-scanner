use std::net::{SocketAddr, TcpStream};
use std::thread::{JoinHandle, Result};
// use std::result::Result;
use std::{io::Result as IOResult, thread, time::Duration};
// use std::any::Any

// type Res<T> = Result<T, Box<dyn Any + Send + 'static>>;
type Res<T> = IOResult<T>;

pub struct PortScanner {
    timeout: Duration,
    pub sc: SocketConnector,
}

impl PortScanner {
    pub fn new(timeout: Duration, sc: SocketConnector) -> Self {
        Self { timeout, sc }
    }
}

pub enum ExecutionContext {
    Parallel(u8),
    Sequential,
}

pub trait Execute {
    fn execute_all<F, T>(&self, f: Vec<F>) -> Vec<Res<T>>
    where
        F: FnOnce() -> T,
        T: Send + 'static,
        F: Send + 'static;
}

impl Execute for ExecutionContext {
    fn execute_all<F, T>(&self, f: Vec<F>) -> Vec<Res<T>>
    where
        F: FnOnce() -> T,
        T: Send + 'static,
        F: Send + 'static,
    {
        match self {
            ExecutionContext::Parallel(b) => {
                let a: Vec<_> = f
                    .into_iter()
                    .map(|fun| thread::spawn(move || fun()))
                    .collect();

                a.into_iter()
                    .map(|h| h.join().map_err(|e| std::panic::resume_unwind(Box::new(e))))
                    .collect::<Vec<Res<T>>>()
            }
            ExecutionContext::Sequential => todo!(),
        }
    }
}

pub struct SocketConnector {
    ec: ExecutionContext,
}

impl SocketConnector {
    pub fn new(ec: ExecutionContext) -> Self {
        Self { ec }
    }
}
pub trait Connect {
    fn connect_batch(&self, socks: Vec<SocketAddr>) -> Vec<Res<TcpStream>>;
}

// pub trait ParContext: ExecutionContext {
//     fn execute<T>(&self, f: impl FnOnce() -> T) -> JoinHandle<T>;
// }

impl Connect for PortScanner {
    fn connect_batch(&self, socks: Vec<SocketAddr>) -> Vec<Res<TcpStream>> {
        let t = self.timeout;
        let handles: Vec<_> = socks
            .into_iter()
            .map(|sock| move || TcpStream::connect_timeout(&sock, t))
            .collect();

        self.sc
            .ec
            .execute_all(handles)
            .into_iter()
            .flat_map(|a| a.map_err(|e| std::panic::resume_unwind(Box::new(e))))
            .collect::<Vec<Res<TcpStream>>>()
    }
}
