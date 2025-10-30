/// The `run_scanner` function asynchronously scans a list of IP addresses for open ports within a
/// specified range using a semaphore to limit concurrent scans.
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::future::join_all;
/// The `run_scanner` function in Rust asynchronously scans a range of IP addresses for open ports and
/// prints the results.

use super::{get_all_ips, scan_all_ports_optimized};

pub async fn run_scanner() {
    let ips = get_all_ips().await;
    let ports_to_scan: Arc<Vec<u16>> = Arc::new((1u16..10024u16).collect());
    
    let semaphore = Arc::new(Semaphore::new(50));
    
    let tasks: Vec<_> = ips.into_iter().map(|ip| {
        let ports = Arc::clone(&ports_to_scan);
        let sem = semaphore.clone();
        
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            
            println!("→ Escaneando IP: {}", ip);
            let open_ports = scan_all_ports_optimized(&ip, &ports).await;
            
            (ip, open_ports)
        })
    }).collect();
    
    let results = join_all(tasks).await;
    
    println!("\n=== RESULTADOS DEL ESCANEO ===");
    for (ip, open_ports) in results.into_iter().flatten() {
        if !open_ports.is_empty() {
            println!("✓ {} → {:?}", ip, open_ports);
        }
    }
}

