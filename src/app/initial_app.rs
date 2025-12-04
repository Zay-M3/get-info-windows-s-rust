use std::process::exit;

use sysinfo::System;
use colored::*;
use crate::app::*;
use crate::functions::{
    get_cpu_info, get_disks_info, get_memory_info, get_networks_info, get_processes_info,
    get_system_os, get_users_info, ask_paths_to_check, check_path_exists,
};
use crate::utils::SystemReport;


/// The function `start` gathers system information and paths to check, creates a system report, and
/// prints/sends it as JSON.
/// 
/// Arguments:
/// 
/// * `sys`: The `sys` parameter is a mutable reference to a `System` struct or object. It is being
/// passed to the `start` function to gather various system information and generate a system report.
pub async fn get_info_system_json(sys: &mut System) -> SystemReport {
    let paths_to_check = ask_paths_to_check();
    let reporte = SystemReport {
        sistema_operativo: get_system_os(),
        cpu: get_cpu_info(sys),
        memoria: get_memory_info(sys),
        discos: get_disks_info(),
        redes: get_networks_info(),
        usuarios: get_users_info(),
        procesos: get_processes_info(sys),
        verificacion_aplicaciones: check_path_exists(paths_to_check),
    };

    reporte
}
/// The `start_menu_app` function in Rust displays a menu with options to get system information, send
/// information to an endpoint, scan the network, change the endpoint, or exit the program based on user
/// input.
/// 
/// Arguments:
/// 
/// * `sys`: The `sys` parameter in the `start_menu_app` function is a mutable reference to a `System`
/// struct or object. This parameter is used to interact with system-related functionalities within the
/// function, such as getting system information, sending system information to an endpoint, scanning
/// the network, changing endpoint settings

pub async fn start_menu_app(sys: &mut System) {

    if !control::SHOULD_COLORIZE.should_colorize() {
        colored::control::set_override(false);
    }

    loop {
        // Clear screen (optional)
        print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear screen
        
        // ASCII Art Banner
        let banner = r#"
 ██████╗ ███████╗████████╗██╗███╗   ██╗███████╗ ██████╗ 
██╔════╝ ██╔════╝╚══██╔══╝██║████╗  ██║██╔════╝██╔═══██╗
██║  ███╗█████╗     ██║   ██║██╔██╗ ██║█████╗  ██║   ██║
██║   ██║██╔══╝     ██║   ██║██║╚██╗██║██╔══╝  ██║   ██║
╚██████╔╝███████╗   ██║   ██║██║ ╚████║██║     ╚██████╔╝
 ╚═════╝ ╚══════╝   ╚═╝   ╚═╝╚═╝  ╚═══╝╚═╝      ╚═════╝ 
    Windows System Information Tool in Rust"#;

        println!("{}", banner.bright_green().bold());
        
        // Info box
        let info = r#"
╔════════════════════════════════════════════════════════╗
║  GitHub: https://github.com/Zay-M3                     ║
║  Project: GetInfo                                      ║
╚════════════════════════════════════════════════════════╝"#;
        
        println!("{}", info.bright_yellow());
        
        // Menu options
        println!("\n{}", "═".repeat(56).bright_cyan());
        println!("{}", "                MAIN MENU".bright_white().bold());
        println!("{}", "═".repeat(56).bright_cyan());
        
        println!("\n  {}  {}", "1.".bright_blue().bold(), "Get system information".white());
        println!("  {}  {}", "2.".bright_blue().bold(), "Send system information to endpoint".white());
        println!("  {}  {}", "3.".bright_blue().bold(), "Scan entire network".white());
        println!("  {}  {}", "4.".bright_blue().bold(), "Scan network - IP and Port".white());
        println!("  {}  {}", "5.".bright_blue().bold(), "Change endpoint global".white());
        println!("  {}  {}", "6.".bright_blue().bold(), "Check SSH dist".white());
        println!("  {}  {}", "0.".bright_red().bold(), "Exit".white());

        print!("\n{} ", "Select an option:".bright_white().bold());
        println!("\n{}", "═".repeat(56).bright_cyan());

        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).expect("Error reading input");
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                get_info_system_command(sys).await;
            },
            
            "2" => {
                post_info_system_command(sys).await;
            },
            "3" => {
                scan_network_command().await;
            },
            "4" => {
                scan_network_ip_port_command().await;
            },
            
            "5" => {
                change_endpoint_command().await;
            },
            "6" => {
                check_ssh_dist_command().await;
            },
            "0" => {
                println!("{}", "\n\n✓ Exiting program...".bright_red().bold());
                println!("{}", "Goodbye!\n".bright_green());
                //add un time of 5 seconds before exit
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                exit(0);
                
            },
            _ => {
                println!("{}", "\n✗ Invalid option. Please try again.".bright_red().bold());
                
                // Wait before showing menu again
                println!("\n{} ", "Press Enter to continue...".bright_yellow());
                let mut _dummy = String::new();
                std::io::stdin().read_line(&mut _dummy).unwrap();
            }
        }
    }
}
