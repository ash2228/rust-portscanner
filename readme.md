# Rust Port Scanner

A simple and efficient multithreaded port scanner written in Rust. This tool allows you to scan a specified host for open and closed server ports.

## Features

- **Multithreaded Scanning**: Utilizes all available CPU cores to maximize performance and speed during the scanning process.
- **Concurrency**: Leveraging the `tokio` runtime for asynchronous networking operations.
- **User-Friendly CLI**: Built with `clap` for easy command-line argument parsing.

## Requirements

- Rust (with Cargo)
- Tokio (async runtime)
- Clap (command-line argument parser)
- Num CPUs (to utilize available CPU cores)

## Installation

To build and run this project, ensure you have Rust installed, then clone this repository:

```bash
git clone https://github.com/ash2228/rust-portscanner.git
cd rust-portscanner
cargo build --release
./target/release/port_scanner <host>