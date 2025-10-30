use sysinfo::{System, Networks, Disks, Users};
use std::net::UdpSocket;

use crate::utils::interfase::*;

/// The function `get_system_os` in Rust retrieves information about the system's operating system,
/// including OS name, version, kernel version, hostname, and uptime.
/// 
/// Returns:
/// 
/// The function `get_system_os` is returning an instance of the `SistemaOperativo` struct, which
/// contains information about the system's operating system, version, kernel version, hostname, uptime
/// in seconds, and uptime in days.
pub fn get_system_os() -> SistemaOperativo {
    let uptime = System::uptime();
    SistemaOperativo {
        os: System::name().unwrap_or_default(),
        version: System::os_version().unwrap_or_default(),
        kernel: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        uptime_segundos: uptime,
        uptime_dias: uptime / 86400,
    }
}
/// The function `get_cpu_info` retrieves information about the CPU, including details about each core
/// such as name, frequency, usage percentage, and vendor ID.
/// 
/// Arguments:
/// 
/// * `sys`: A reference to a System object with refreshed data
/// 
/// Returns:
/// 
/// The `get_cpu_info` function is returning a `CpuInfo` struct that contains information about the
/// CPU(s) in the system. The struct includes the total number of CPUs and a vector of `CpuDetalle`
/// structs, each representing detailed information about an individual CPU core.
pub fn get_cpu_info(sys: &System) -> CpuInfo {
    let cpus_detalle: Vec<CpuDetalle> = sys.cpus().iter().enumerate()
        .map(|(i, cpu)| CpuDetalle {
            id: i,
            nombre: cpu.name().to_string(),
            frecuencia_mhz: cpu.frequency(),
            uso_porcentaje: cpu.cpu_usage(),
            vendor: cpu.vendor_id().to_string(),
        })
        .collect();

    CpuInfo {
        total_cpus: sys.cpus().len(),
        cpus: cpus_detalle,
    }
}
/// This Rust function retrieves memory information such as total RAM, used RAM, free RAM, RAM usage
/// percentage, total swap space, used swap space, and free swap space.
/// 
/// Arguments:
/// 
/// * `sys`: A reference to a System object with refreshed data
/// 
/// Returns:
/// 
/// The `get_memory_info` function is returning a struct of type `MemoriaInfo` which contains various
/// memory-related information such as total RAM in gigabytes, total RAM in bytes, used RAM in
/// gigabytes, used RAM in bytes, free RAM in gigabytes, free RAM in bytes, RAM usage percentage, total
/// swap space in gigabytes, used swap space in gigabytes, and free swap
pub fn get_memory_info(sys: &System) -> MemoriaInfo {
    const BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;
    
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    MemoriaInfo {
        ram_total_gb: total_memory as f64 / BYTES_TO_GB,
        ram_total_bytes: total_memory,
        ram_usada_gb: used_memory as f64 / BYTES_TO_GB,
        ram_usada_bytes: used_memory,
        ram_libre_gb: available_memory as f64 / BYTES_TO_GB,
        ram_libre_bytes: available_memory,
        ram_uso_porcentaje: (used_memory as f64 / total_memory as f64) * 100.0,
        swap_total_gb: total_swap as f64 / BYTES_TO_GB,
        swap_usada_gb: used_swap as f64 / BYTES_TO_GB,
        swap_libre_gb: (total_swap - used_swap) as f64 / BYTES_TO_GB,
    }
}
/// The function `get_disks_info` retrieves information about disks in the system and returns it as a
/// vector of `DiscoInfo` structs.
/// 
/// Returns:
/// 
/// The function `get_disks_info` is returning a vector of `DiscoInfo` structs. Each `DiscoInfo` struct
/// contains information about a disk, such as disk number, name, mount point, file system, type, total
/// space in GB, available space in GB, used space in GB, percentage usage, and whether the disk is
/// removable.
pub fn get_disks_info() -> Vec<DiscoInfo> {
    const BYTES_TO_GB: u64 = 1024 * 1024 * 1024;
    
    let disks = Disks::new_with_refreshed_list();
    disks.iter().enumerate()
        .map(|(i, disk)| {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            
            DiscoInfo {
                numero: i + 1,
                nombre: disk.name().to_string_lossy().to_string(),
                punto_montaje: disk.mount_point().display().to_string(),
                sistema_archivos: format!("{:?}", disk.file_system()),
                tipo: format!("{:?}", disk.kind()),
                espacio_total_gb: total_space / BYTES_TO_GB,
                espacio_disponible_gb: available_space  / BYTES_TO_GB,
                espacio_usado_gb: used_space  / BYTES_TO_GB,
                uso_porcentaje: (used_space / total_space) * 100,
                removible: disk.is_removable(),
            }
        })
        .collect()
}

/// The function `get_networks_info` retrieves network information such as interface details and local
/// IP address in Rust.
/// 
/// Returns:
/// 
/// The `get_networks_info` function is returning an instance of the `RedesInfo` struct, which contains
/// information about network interfaces and the local IP address.
pub fn get_networks_info() -> RedesInfo {
    const BYTES_TO_MB: f64 = 1024.0 * 1024.0;
    
    let networks = Networks::new_with_refreshed_list();
    let redes_info: Vec<InterfazRed> = networks.iter()
        .map(|(interface_name, data)| InterfazRed {
            nombre: interface_name.to_string(),
            mac: data.mac_address().to_string(),
            recibido_mb: data.total_received() as f64 / BYTES_TO_MB,
            recibido_bytes: data.total_received(),
            transmitido_mb: data.total_transmitted() as f64 / BYTES_TO_MB,
            transmitido_bytes: data.total_transmitted(),
            paquetes_recibidos: data.total_packets_received(),
            paquetes_transmitidos: data.total_packets_transmitted(),
            errores_recibidos: data.total_errors_on_received(),
            errores_transmitidos: data.total_errors_on_transmitted(),
        })
        .collect();

    //ip local principal
    let mut ip_local = String::from("No disponible");
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0")
        && socket.connect("8.8.8.8:80").is_ok()
        && let Ok(addr) = socket.local_addr()
    {
        ip_local = addr.ip().to_string();
    }

    RedesInfo {
        interfaces: redes_info,
        ip_local_principal: ip_local,
    }
}
/// The function `get_users_info` retrieves information about users and their groups in Rust.
/// 
/// Returns:
/// 
/// The function `get_users_info` is returning an instance of `UsuariosInfo` struct.
pub fn get_users_info() -> UsuariosInfo {
    let users = Users::new_with_refreshed_list();
    let total_usuarios = users.len();

    let usuarios_detalle: Vec<UsuarioDetalle> = users.iter()
        .map(|user| {
            let grupos: Vec<String> = user.groups().iter()
                .map(|g| g.name().to_string())
                .collect();
            UsuarioDetalle {
                nombre: user.name().to_string(),
                grupos,
            }
        })
        .collect();

    UsuariosInfo {
        total: total_usuarios,
        usuarios: usuarios_detalle,
    }
}
/// The function `get_processes_info` retrieves information about processes, including the top 10
/// processes by CPU and memory usage.
/// 
/// Arguments:
/// 
/// * `sys`: A reference to a System object with refreshed data
/// 
/// Returns:
/// 
/// The `get_processes_info` function is returning a struct `ProcesosInfo` which contains information
/// about processes. The struct includes the total number of processes, the top 10 processes by CPU
/// usage, and the top 10 processes by memory usage. Each top process is represented by a
/// `ProcesoDetalle` struct which contains details such as process ID, name, CPU percentage, memory
/// usage in
pub fn get_processes_info(sys: &System) -> ProcesosInfo {
    const BYTES_TO_MB: f64 = 1024.0 * 1024.0;
    
    let mut processes_vec: Vec<_> = sys.processes().iter().collect();
    
    // Top 10 procesos por uso de CPU
    processes_vec.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
    let top_10_cpu: Vec<ProcesoDetalle> = processes_vec.iter()
        .take(10)
        .enumerate()
        .map(|(i, (pid, process))| ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / BYTES_TO_MB,
            memoria_virtual_mb: process.virtual_memory() as f64 / BYTES_TO_MB,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        })
        .collect();
    
    // Top 10 procesos por uso de memoria
    processes_vec.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    let top_10_memoria: Vec<ProcesoDetalle> = processes_vec.iter()
        .take(10)
        .enumerate()
        .map(|(i, (pid, process))| ProcesoDetalle {
            indice: i,
            pid: pid.as_u32(),
            nombre: process.name().to_string(),
            cpu_porcentaje: process.cpu_usage(),
            memoria_mb: process.memory() as f64 / BYTES_TO_MB,
            memoria_virtual_mb: process.virtual_memory() as f64 / BYTES_TO_MB,
            disco_lectura_bytes: process.disk_usage().read_bytes,
            disco_escritura_bytes: process.disk_usage().written_bytes,
        })
        .collect();
    
    ProcesosInfo {
        total: sys.processes().len(),
        top_10_cpu,
        top_10_memoria,
    }
}
