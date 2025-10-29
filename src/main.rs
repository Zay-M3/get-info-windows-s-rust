use serde_json;

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
use api::send_info::*;
use functions::helpers::*;
use functions::getinfo::*;

fn main() {
    
    let paths_to_check = ask_paths_to_check();
    
    // ============= CREAR REPORTE COMPLETO =============
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

    let winput = request_input_ticket();

    // ============= GENERAR E IMPRIMIR JSON =============
    match serde_json::to_string_pretty(&reporte) {
        Ok(json) => {
            println!("{}", json);
            
            // ============= ENVIAR INFO A SERVIDOR REMOTO =============
            let url = "https://127.0.0.1:8000/api/info"; 
            let info = Info {
                id: 1,
                name: "SistemaReporte".into(),
                active: true,
                winput: winput.clone(),
            };
            
            let rt = tokio::runtime::Runtime::new().unwrap();
            match rt.block_on(send_info(url, &info)) {
                Ok(status) => println!("Información enviada con estado: {}", status),
                Err(e) => eprintln!("Error al enviar información: {}", e),
            }
        },
        Err(e) => eprintln!("Error al generar JSON: {}", e),

        
    }
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

