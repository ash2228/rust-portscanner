use std::{net::SocketAddr, sync::Mutex, time::Duration};
use tokio::{net::TcpStream, task};
use std::sync::Arc;
use num_cpus;
use clap::Parser;

#[derive(Parser)]
struct Args {
    host: String
}

#[tokio::main]
async fn main() {
    println!("Port Scanner");
    let args = Args::parse();
    let host = args.host;
    let host = host.trim().to_string();
    let threds = num_cpus::get();
    println!("Scanning from ports 0..10000");
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = vec![];
    for i in 0..threds {
        let host = host.clone();
        let open_ports = Arc::clone(&open_ports);
        let handle = task::spawn(async move{
            let trim_host = host.trim();
            for j in 1..10000/threds {
                let port = (i*10000/threds)+j;
                let ip_addr = format!("{trim_host}:{port}");
                match port_exists(&ip_addr).await {
                    Some(true) => {println!("Port {port} is open");open_ports.lock().unwrap().push(port);},
                    Some(false) => println!("Port {port} is closed"),
                    None => {}
                }
            }
        });
        tasks.push(handle);
    }
    for task in tasks {
        task.await.unwrap();
    }
    println!("{} ports are open on {host}", open_ports.lock().unwrap().len());
}

async fn port_exists(addr: &str) -> Option<bool> {
    let socket_addr: SocketAddr = addr.parse().ok()?;
    let timeout = Duration::from_secs(1);
    match tokio::time::timeout(timeout, TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => Some(true),
        Ok(Err(_)) => Some(false),
        Err(_) => None,
    }
}
