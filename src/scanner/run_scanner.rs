/// The `run_scanner` function asynchronously scans a list of IP addresses for open ports within a
/// specified range using a semaphore to limit concurrent scans.
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::future::join_all;

use crate::scanner::{
    get_all_ips,
    scan_all_ports_optimized
};

pub async fn run_scanner() {
    let ips = get_all_ips().await;
    let ports_to_scan: Vec<u16> = (1u16..1024u16).collect();
    
    let semaphore = Arc::new(Semaphore::new(50));
    
    let tasks: Vec<_> = ips.into_iter().map(|ip| {
        let ports = ports_to_scan.clone();
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
    for result in results {
        if let Ok((ip, open_ports)) = result {
            if !open_ports.is_empty() {
                println!("✓ {} → {:?}", ip, open_ports);
            }
        }
    }
}

