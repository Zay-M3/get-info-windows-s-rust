
use sysinfo::{System};
mod utils;
mod api;
use utils::{SystemReport};
mod functions;
use functions::{
    get_system_os, 
    get_cpu_info, 
    get_memory_info, 
    get_disks_info, 
    get_networks_info, 
    get_users_info, 
    get_processes_info, 
    check_path_exists, 
    ask_paths_to_check,
    print_and_send_json
};
mod scanner;
use scanner::*;


/// The function `start` gathers system information and paths to check, creates a system report, and
/// prints/sends it as JSON.
/// 
/// Arguments:
/// 
/// * `sys`: The `sys` parameter is a mutable reference to a `System` struct or object. It is being
/// passed to the `start` function to gather various system information and generate a system report.
async fn start(sys: &mut System) {
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

    print_and_send_json(&reporte).await;

    // Run the async scanner to get all IPs
    run_scanner().await;
}

#[tokio::main]
async fn main() {
    // Create a single System instance and refresh all data once
    let mut sys = System::new_all();
    sys.refresh_all();

    start(&mut sys).await;
    println!("Press Enter to exit...");
    
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

