
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

/// The function collects system information, checks specified paths, and generates a report in JSON
/// format.

fn main() {
    
    let paths_to_check = ask_paths_to_check();
    let reporte = SystemReport {
        sistema_operativo: get_system_os(),
        cpu: get_cpu_info(),
        memoria: get_memory_info(),
        discos: get_disks_info(),
        redes: get_networks_info(),
        usuarios: get_users_info(),
        procesos: get_processes_info(),
        verificacion_aplicaciones: check_path_exists(paths_to_check),
    };

    print_and_send_json(&reporte);
    
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

