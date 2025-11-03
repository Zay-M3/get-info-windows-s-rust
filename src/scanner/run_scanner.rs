/// The `run_scanner` function asynchronously scans a list of IP addresses for open ports within a
/// specified range using a semaphore to limit concurrent scans.
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::future::join_all;
use colored::*;
/// The `run_scanner` function in Rust asynchronously scans a range of IP addresses for open ports and
/// prints the results.

use super::{get_all_ips, scan_all_ports_optimized};
/// The `run_scanner` function in Rust asynchronously scans a range of IPs for open ports and displays
/// the results.

pub async fn run_scanner() {
    let ips = get_all_ips().await;
    let ports_to_scan: Arc<Vec<u16>> = Arc::new((1u16..10024u16).collect());
    
    let semaphore = Arc::new(Semaphore::new(50));
    
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "           NETWORK SCANNER".bright_white().bold());
    println!("{}", "═".repeat(56).bright_cyan());
    println!("\n{} Scanning {} IPs...\n", "→".bright_blue().bold(), ips.len().to_string().bright_yellow());
    
    let tasks: Vec<_> = ips.into_iter().map(|ip| {
        let ports = Arc::clone(&ports_to_scan);
        let sem = semaphore.clone();
        
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            
            println!("{} Scanning IP: {}", "→".bright_blue(), ip.to_string().bright_cyan());
            let open_ports = scan_all_ports_optimized(&ip, &ports).await;
            
            (ip, open_ports)
        })
    }).collect();
    
    let results = join_all(tasks).await;
    
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "           SCAN RESULTS".bright_white().bold());
    println!("{}", "═".repeat(56).bright_cyan());
    
    let mut found_any = false;
    for (ip, open_ports) in results.into_iter().flatten() {
        if !open_ports.is_empty() {
            found_any = true;
            let ports_str = format!("{:?}", open_ports);
            println!("{} {} {} {}", 
                "✓".bright_green().bold(), 
                ip.to_string().bright_cyan(), 
                "→".bright_white(), 
                ports_str.bright_yellow()
            );
        }
    }
    
    if !found_any {
        println!("{} {}", 
            "ℹ".bright_blue().bold(), 
            "No open ports found on any scanned IPs.".bright_white()
        );
    }
    
    println!("{}", "═".repeat(56).bright_cyan());
}

/// The `run_scanner_ip_port` function in Rust asynchronously scans a specified IP address on a given
/// port and displays the scan results.
/// 
/// Arguments:
/// 
/// * `ip`: The `ip` parameter in the `run_scanner_ip_port` function is a string reference (`&str`)
/// representing the IP address that will be scanned for open ports.
/// * `port`: The `port` parameter in the `run_scanner_ip_port` function is the port number that will be
/// scanned for the specified IP address. It is of type `u16`, which represents an unsigned 16-bit
/// integer used to identify specific network ports.

pub async fn run_scanner_ip_port(ip: &str, port: u16) {
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "        IP & PORT SCANNER".bright_white().bold());
    println!("{}", "═".repeat(56).bright_cyan());
    
    println!("\n{} Scanning IP: {} on port: {}", 
        "→".bright_blue().bold(), 
        ip.bright_cyan(), 
        port.to_string().bright_yellow()
    );
    
    let ports_to_scan: Arc<Vec<u16>> = Arc::new(vec![port]);
    
    let open_ports = scan_all_ports_optimized(ip, &ports_to_scan).await;
    
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "           SCAN RESULTS".bright_white().bold());
    println!("{}", "═".repeat(56).bright_cyan());
    
    if !open_ports.is_empty() {
        println!("\n{} {} {} {}", 
            "✓".bright_green().bold(), 
            ip.bright_cyan(), 
            "→".bright_white(), 
            format!("Port {} is OPEN", port).bright_green()
        );
    } else {
        println!("\n{} {} {} {}", 
            "✗".bright_red().bold(), 
            ip.bright_cyan(), 
            "→".bright_white(), 
            format!("Port {} is CLOSED", port).bright_red()
        );
    }
    
    println!("{}", "═".repeat(56).bright_cyan());
}

