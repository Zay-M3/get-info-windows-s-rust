
mod functions {
    pub mod helpers;
    pub mod getinfo;
}

mod utils {
    pub mod interfase;
}

mod api {
    pub mod send_info;
}


use utils::interfase::*;
use functions::helpers::*;
use functions::getinfo::*;
use sysinfo::System;

/// The function collects system information, checks specified paths, and generates a report in JSON
/// format.

fn main() {
    // Create a single System instance and refresh all data once
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let paths_to_check = ask_paths_to_check();
    let reporte = SystemReport {
        sistema_operativo: get_system_os(),
        cpu: get_cpu_info(&sys),
        memoria: get_memory_info(&sys),
        discos: get_disks_info(),
        redes: get_networks_info(),
        usuarios: get_users_info(),
        procesos: get_processes_info(&sys),
        verificacion_aplicaciones: check_path_exists(paths_to_check),
    };

    print_and_send_json(&reporte);
    
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

