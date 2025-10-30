use if_addrs::get_if_addrs;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::sync::Arc;

const PER_HOST_CONCURRENCY: usize = 20;
const BATCH_SIZE: usize = 1024;
const TIMEOUT_MS: u64 = 200;

pub async fn get_all_ips() -> Vec<String> {
    match get_if_addrs() {
        Ok(interfaces) => {
            let ips: Vec<String> = interfaces
                .into_iter()
                .map(|iface| iface.addr.ip().to_string())
                .collect();
            
            if ips.is_empty() {
                vec!["No IPs found".to_string()]
            } else {
                ips
            }
        }
        Err(_) => vec!["Error reading interfaces".to_string()],
    }
}


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
                    Ok(Ok(_)) => Some(port),
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