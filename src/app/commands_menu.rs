
use sysinfo::System;
use colored::*;
use crate::app::get_info_system_json;
use crate::functions::{print_and_send_json, send_json_report};
use crate::scanner::{run_scanner, run_scanner_ip_port};

/// This Rust function asynchronously retrieves system information, prints it in JSON format, and waits
/// for user input before continuing.
/// 
/// Arguments:
/// 
/// * `sys`: The `sys` parameter in the `get_info_system_command` function is a mutable reference to a
/// `System` struct. This parameter allows the function to access and potentially modify the system
/// information stored in the `System` struct.

pub async fn get_info_system_command(sys: &mut System) {
    println!("{}", "\n\nGetting system information...".bright_green().bold());
    let reporte = get_info_system_json(sys).await;
    print_and_send_json(&reporte).await;
    
    // Wait for user to press Enter
    print!("{} ", "\nPress Enter to continue...".bright_yellow().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}
/// The function `post_info_system_command` in Rust prints system information, sends a JSON report,
/// prompts the user to press Enter, and waits for user input.
/// 
/// Arguments:
/// 
/// * `sys`: The parameter `sys` is a mutable reference to a `System` struct.

pub async fn post_info_system_command(sys: &mut System) {
    println!("{}", "\n\nGetting system information...".bright_green().bold());
    let reporte = get_info_system_json(sys).await;
    send_json_report(&reporte).await;
    
    // Wait for user to press Enter
    print!("{} ", "\nPress Enter to continue...".bright_yellow().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

/// The `scan_network_command` function in Rust asynchronously scans the network and waits for the user
/// to press Enter to continue.

pub async fn scan_network_command() {
    println!("{}", "\n\nScanning network...".bright_yellow().bold());
    run_scanner().await;
    
    // Wait for user to press Enter
    print!("{} ", "\nPress Enter to continue...".bright_yellow().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}
/// The function `scan_network_ip_port_command` in Rust reads user input for an IP address and port,
/// validates the input format, scans the network using the provided IP and port, and prompts the user
/// to press Enter to continue.

pub async fn scan_network_ip_port_command() {
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "\n  ● Enter IP and port to scan: ".green().bold());
    println!("{}", "  ● Example: 192.168.x.x -p 8x".green().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    // Read IP and port in the format shown in the example: 192.168.x.x -p 8x
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    let parts: Vec<&str> = input.split(" -p ").collect();
    if parts.len() != 2 {
        eprintln!("Invalid input. Make sure to follow the format: 192.168.x.x -p 8x");
        return;
    }

    let ip = parts[0].trim();
    let port_str = parts[1].trim();

    let port = port_str.trim().parse::<u16>().expect("Error parsing port");

    println!("{}", "\n\nScanning network by IP and Port...".bright_yellow().bold());
    run_scanner_ip_port(ip, port).await;

    // Wait for user to press Enter
    print!("{} ", "\nPress Enter to continue...".bright_yellow().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}
/// The function `change_endpoint_command` in Rust allows users to input a new endpoint URL, calls a
/// function to change the endpoint, and provides feedback on the success or failure of the operation.

pub async fn change_endpoint_command() {
    use crate::api::send_info::change_endpoint;

    println!("{}", "\n\n    ► Change Endpoint URL".bright_green().bold());
    println!("{}", "    ► Example: http://<new-url>".bright_white().bold());
    let mut new_url = String::new();
    println!("\n{}", "═".repeat(56).bright_cyan());
    print!("{} ", "Enter new endpoint URL:".bright_green().bold());
    std::io::stdin().read_line(&mut new_url).expect("Error reading input");
    let new_url = new_url.trim();

    match change_endpoint(&new_url) {
        Ok(_) => {
            println!("{}", "✓ Endpoint URL changed successfully.".bright_green());
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Error changing endpoint URL:".bright_red(), e);
        }
    }

    // Wait for user to press Enter
    print!("{} ", "\nPress Enter to continue...".bright_yellow().bold());
    println!("\n{}", "═".repeat(56).bright_cyan());

    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}
