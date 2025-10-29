use sysinfo::{System, Networks, Disks, Users};
use std::net::UdpSocket;
use serde_json;

mod functions {
    pub mod helpers;
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

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // ============= SISTEMA OPERATIVO =============
    let uptime = System::uptime();
    let sistema_operativo = SistemaOperativo {
        os: System::name().unwrap_or_default(),
        version: System::os_version().unwrap_or_default(),
        kernel: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        uptime_segundos: uptime,
        uptime_dias: uptime / 86400,
    };
    
    // ============= CPU =============
    let mut cpus_detalle = Vec::new();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        cpus_detalle.push(CpuDetalle {
            id: i,
            nombre: cpu.name().to_string(),
            frecuencia_mhz: cpu.frequency(),
            uso_porcentaje: cpu.cpu_usage(),
            vendor: cpu.vendor_id().to_string(),
        });
    }
    
    let cpu_info = CpuInfo {
        total_cpus: sys.cpus().len(),
        cpus: cpus_detalle,
    };
    
    // ============= MEMORIA =============
    let memoria_info = MemoriaInfo {
        ram_total_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_total_bytes: sys.total_memory(),
        ram_usada_gb: sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_usada_bytes: sys.used_memory(),
        ram_libre_gb: sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        ram_libre_bytes: sys.available_memory(),
        ram_uso_porcentaje: (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0,
        swap_total_gb: sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        swap_usada_gb: sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
        swap_libre_gb: sys.free_swap() as f64 / 1024.0 / 1024.0 / 1024.0,
    };
    
    // ============= DISCOS =============
    let disks = Disks::new_with_refreshed_list();
    let mut discos_info = Vec::new();
    for (i, disk) in disks.iter().enumerate() {
        discos_info.push(DiscoInfo {
            numero: i + 1,
            nombre: disk.name().to_string_lossy().to_string(),
            punto_montaje: disk.mount_point().display().to_string(),
            sistema_archivos: format!("{:?}", disk.file_system()),
            tipo: format!("{:?}", disk.kind()),
            espacio_total_gb: disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            espacio_disponible_gb: disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            espacio_usado_gb: (disk.total_space() - disk.available_space()) as f64 / 1024.0 / 1024.0 / 1024.0,
            uso_porcentaje: ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64) * 100.0,
            removible: disk.is_removable(),
        });
    }
    
    // ============= REDES E IPs =============
    let networks = Networks::new_with_refreshed_list();
    let mut interfaces = Vec::new();
    for (interface_name, network) in &networks {
        interfaces.push(InterfazRed {
            nombre: interface_name.to_string(),
            mac: network.mac_address().to_string(),
            recibido_mb: network.total_received() as f64 / 1024.0 / 1024.0,
            recibido_bytes: network.total_received(),
            transmitido_mb: network.total_transmitted() as f64 / 1024.0 / 1024.0,
            transmitido_bytes: network.total_transmitted(),
            paquetes_recibidos: network.total_packets_received(),
            paquetes_transmitidos: network.total_packets_transmitted(),
            errores_recibidos: network.total_errors_on_received(),
            errores_transmitidos: network.total_errors_on_transmitted(),
        });
    }
    
    // IP Local
    let mut ip_local = String::from("No disponible");
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                ip_local = addr.ip().to_string();
            }
        }
    }
    
    let redes_info = RedesInfo {
        interfaces,
        ip_local_principal: ip_local,
    };
    
    // ============= USUARIOS =============
    let users = Users::new_with_refreshed_list();
    let mut usuarios_detalle = Vec::new();
    for user in &users {
        let grupos: Vec<String> = user.groups().iter()
            .map(|g| g.name().to_string())
            .collect();
        usuarios_detalle.push(UsuarioDetalle {
            nombre: user.name().to_string(),
            grupos,
        });
    }
    
    let usuarios_info = UsuariosInfo {
        total: users.len(),
        usuarios: usuarios_detalle,
    };
    
    // ============= PROCESOS =============
    let mut processes_vec: Vec<_> = sys.processes().iter().collect();
    
    // Top 10 procesos por uso de CPU
    processes_vec.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
    let mut top_10_cpu = Vec::new();
    for (i, (pid, process)) in processes_vec.iter().take(10).enumerate() {
        top_10_cpu.push(ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / 1024.0 / 1024.0,
            memoria_virtual_mb: process.virtual_memory() as f64 / 1024.0 / 1024.0,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        });
    }
    
    // Top 10 procesos por uso de memoria
    processes_vec.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    let mut top_10_memoria = Vec::new();
    for (i, (pid, process)) in processes_vec.iter().take(10).enumerate() {
        top_10_memoria.push(ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / 1024.0 / 1024.0,
            memoria_virtual_mb: process.virtual_memory() as f64 / 1024.0 / 1024.0,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        });
    }
    
    let procesos_info = ProcesosInfo {
        total: sys.processes().len(),
        top_10_cpu,
        top_10_memoria,
    };
    
    // ============= VERIFICACIÓN DE CARPETA PERSONALIZADA =============

    let paths_to_check = ask_paths_to_check();
    let mut aplicaciones = Vec::new();
    
    for path in paths_to_check {
        let existe = std::path::Path::new(&path).exists();
        let elementos = if existe {
            if let Ok(entries) = std::fs::read_dir(&path) {
                let files: Vec<_> = entries.filter_map(Result::ok).collect();
                Some(files.len())
            } else {
                None
            }
        } else {
            None
        };
        
        aplicaciones.push(AplicacionInfo {
            ruta: path.to_string(),
            existe,
            elementos,
        });
    }
    
    // ============= CREAR REPORTE COMPLETO =============
    let reporte = SystemReport {
        sistema_operativo,
        cpu: cpu_info,
        memoria: memoria_info,
        discos: discos_info,
        redes: redes_info,
        usuarios: usuarios_info,
        procesos: procesos_info,
        verificacion_aplicaciones: aplicaciones,
    };

    let winput = request_input_ticket();

    // ============= GENERAR E IMPRIMIR JSON =============
    match serde_json::to_string_pretty(&reporte) {
        Ok(json) => {
            println!("{}", json);
            
            // ============= ENVIAR INFO A SERVIDOR REMOTO =============
            let url = "https://httpbin.org/post"; 
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

