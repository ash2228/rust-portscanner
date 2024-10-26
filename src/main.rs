use std::{net::SocketAddr, sync::Mutex, time::{Duration, Instant}};
use tokio::{net::TcpStream, task};
use std::sync::Arc;
use num_cpus;
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
    let threds = num_cpus::get() * 100;
    println!("Scanning from ports 0..10000");
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = vec![];
    let timer = Instant::now();
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
