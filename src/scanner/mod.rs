// Scanner module - network scanning utilities
use if_addrs::get_if_addrs;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::sync::Arc;
use std::io;
use colored::*;

const PER_HOST_CONCURRENCY: usize = 20;
const BATCH_SIZE: usize = 1024;
const TIMEOUT_MS: u64 = 200;

/// The function `get_all_ips` retrieves all IP addresses associated with the system's network
/// interfaces in Rust asynchronously.
/// 
/// Returns:
/// 
/// The function `get_all_ips` returns a `Vec<String>` containing either a list of IP addresses as
/// strings if successfully retrieved from network interfaces, or an error message string if there was
/// an issue reading the interfaces. If no IP addresses are found, it returns a single string indicating
/// that no IPs were found.

pub async fn get_all_ips() -> Vec<String> {
    match get_if_addrs() {
        Ok(interfaces) => {
            let ips: Vec<String> = interfaces
                .into_iter()
                .map(|iface| iface.addr.ip().to_string())
                .collect();
            
            if ips.is_empty() {
                vec!["No se encontraron IPS".to_string()]
            } else {
                ips
            }
        }
        Err(_) => vec!["Error leyendo la interfaz".to_string()],
    }
}


/// The function `scan_all_ports_optimized` asynchronously scans multiple ports on a given IP address in
/// batches, utilizing a semaphore to limit concurrency and handling timeouts for connection attempts.
/// 
/// Arguments:
/// 
/// * `ip`: The `ip` parameter in the `scan_all_ports_optimized` function is a string slice (`&str`)
/// representing the IP address of the host you want to scan for open ports.
/// * `ports`: The `ports` parameter is an array of unsigned 16-bit integers representing the list of
/// ports to scan for the given IP address.
/// 
/// Returns:
/// 
/// The function `scan_all_ports_optimized` returns a `Vec<u16>` containing the open ports found during
/// the port scanning process.

pub async fn scan_all_ports_optimized(ip: &str, ports: &[u16]) -> Vec<u16> {
    let semaphore = Arc::new(Semaphore::new(PER_HOST_CONCURRENCY));
    let mut open_ports = Vec::new();
    
    // Procesar en lotes
    for chunk in ports.chunks(BATCH_SIZE) {
        let mut tasks = Vec::new();
        
        for &port in chunk {
            let ip = ip.to_string();
            let sem = semaphore.clone();
            
            let task = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                
                let socket_addr = format!("{}:{}", ip, port);
                let connect = tokio::net::TcpStream::connect(&socket_addr);
                
                match timeout(Duration::from_millis(TIMEOUT_MS), connect).await {
                    Ok(Ok(_)) => { 
                        println!("  ✓ Puerto abierto: {}", port.to_string().bright_green());
                        Some(port)
                    },
                    Ok(Err(e)) if e.kind() == io::ErrorKind::PermissionDenied => {
                        eprintln!("⚠ Permission denied for port {}", port.to_string().bright_yellow());
                        None
                    },
                    _ => None,
                }
            });
            
            tasks.push(task);
        }
        
        let results = join_all(tasks).await;
        open_ports.extend(
            results.into_iter()
                .filter_map(|r| r.ok().flatten())
        );
        
        // Pequeña pausa entre lotes para no saturar
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    open_ports
}

pub mod run_scanner;
pub use run_scanner::*;