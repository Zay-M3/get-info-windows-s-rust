use sysinfo::System;
use colored::*;
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

pub async fn start_menu_app(_sys: &mut System) {
    // ASCII Art Bannergit status
    let banner = r#"
 ██████╗ ███████╗████████╗██╗███╗   ██╗███████╗ ██████╗ 
██╔════╝ ██╔════╝╚══██╔══╝██║████╗  ██║██╔════╝██╔═══██╗
██║  ███╗█████╗     ██║   ██║██╔██╗ ██║█████╗  ██║   ██║
██║   ██║██╔══╝     ██║   ██║██║╚██╗██║██╔══╝  ██║   ██║
╚██████╔╝███████╗   ██║   ██║██║ ╚████║██║     ╚██████╔╝
 ╚═════╝ ╚══════╝   ╚═╝   ╚═╝╚═╝  ╚═══╝╚═╝      ╚═════╝ 
    Sistema de Información de Windows en Rust"#;

    println!("{}", banner.bright_green().bold());
    
    // Info box
    let info = r#"
╔════════════════════════════════════════════════════════╗
║  GitHub: https://github.com/Zay-M3                     ║
║  Project: get-info-windows-s-rust                      ║
╚════════════════════════════════════════════════════════╝"#;
    
    println!("{}", info.bright_yellow());
    
    // Menu options
    println!("\n{}", "═".repeat(56).bright_cyan());
    println!("{}", "                MENÚ PRINCIPAL".bright_white().bold());
    println!("{}", "═".repeat(56).bright_cyan());
    
    println!("\n  {}  {}", "1.".bright_blue().bold(), "Obtener información del sistema".white());
    println!("  {}  {}", "2.".bright_blue().bold(), "Escanear red".white());
    println!("  {}  {}", "3.".bright_red().bold(), "Salir".white());
    
    println!("\n{}", "═".repeat(56).bright_cyan());
    print!("\n{} ", "Seleccione una opción:".bright_white().bold());
}

