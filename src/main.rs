use std::{net::SocketAddr, sync::Mutex, time::{Duration, Instant}};
use tokio::{net::TcpStream, task};
use std::sync::Arc;
use clap::Parser;
use notify_rust::{Notification, Timeout};
use figlet_rs::FIGfont;

/// Simple program to print "Made by Ash" in ASCII art and scan ports on a specified host.
#[derive(Parser)]
struct Cli {
    /// Text to display in ASCII art
    #[arg(short, long, default_value = "Made by Ash")]
    text: String,

    /// Hostname or IP address to scan
    host: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // Load the standard font
    let font = FIGfont::standard().expect("Error loading font");

    // Generate ASCII art for the text
    let figure = font.convert(&args.text);

    // Print the ASCII art if the conversion was successful
    if let Some(figure) = figure {
        println!("{}", figure);
    } else {
        eprintln!("Error: Could not generate ASCII art for the given text");
    }

    // Begin port scanning
    println!("Port Scanner");
    let host = args.host.trim().to_string();
    println!("Scanning Server Ports...");

    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let ports = vec![21, 22, 23, 25, 53, 80, 110, 137, 138, 139, 143, 443, 445, 548, 587, 993, 995, 1433, 1701, 1723, 3306, 5432, 8008, 8443];
    let timer = Instant::now();
    let mut tasks = vec![];

    for port in ports {
        let host = host.clone();
        let open_ports = Arc::clone(&open_ports);
        let handle = task::spawn(async move {
            let ip_addr = format!("{host}:{port}");
            match port_exists(&ip_addr).await {
                Some(true) => {
                    println!("Open port {port}");
                    open_ports.lock().unwrap().push(port);
                },
                Some(false) => println!("Closed port {port}"),
                None => {}
            }
        });
        tasks.push(handle);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let elapsed = timer.elapsed();
    Notification::new()
        .summary("Port Scanner")
        .body(&format!("Scanning Complete in {:.2?}", elapsed))
        .icon("Rust")
        .timeout(Timeout::Milliseconds(1000))
        .appname("Rust")
        .show()
        .unwrap();

    println!("{} ports are open on {}\nTook {:.2?}", open_ports.lock().unwrap().len(), host, elapsed);
}

async fn port_exists(addr: &str) -> Option<bool> {
    let socket_addr: SocketAddr = addr.parse().ok()?;
    let timeout = Duration::from_millis(500);
    match tokio::time::timeout(timeout, TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => Some(true),
        Ok(Err(_)) => Some(false),
        Err(_) => None,
    }
}
