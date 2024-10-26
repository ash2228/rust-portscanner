use std::{net::SocketAddr, sync::Mutex, time::{Duration, Instant}};
use tokio::{net::TcpStream, task};
use std::sync::Arc;
use clap::Parser;
use notify_rust::{Notification, Timeout};

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
    println!("Scanning Server Ports...");
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let ports = vec![21, 22, 23, 25, 53, 80, 110, 137, 138, 139, 143, 443, 445, 548, 587, 993, 995, 1433, 1701, 1723, 3306, 5432, 8008, 8443];
    let timer = Instant::now();
    let mut tasks = vec![];
    for port in ports {
        let host = host.clone();
        let open_ports = Arc::clone(&open_ports);
        let handle = task::spawn(async move{
            let ip_addr = format!("{host}:{port}");
            match port_exists(&ip_addr).await {
                Some(true) => {println!("Open port {port}");open_ports.lock().unwrap().push(port);},
                Some(false) => {println!("Closed port {port}")},
                None => {}
            }
        });
        tasks.push(handle);
    }
    for task in tasks {
        task.await.unwrap();
    }
    let elapsed = timer.elapsed();
    Notification::new().summary("Port Scanner").body(format!("Scanning Complete in {:.2?}", elapsed).as_str()).icon("Rust").timeout(Timeout::Milliseconds(1000)).appname("Rust").show().unwrap();
    println!("{} ports are open on {host}\nTook {:.2?}", open_ports.lock().unwrap().len(), elapsed);
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
